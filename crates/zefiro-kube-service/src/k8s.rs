use kube::{Api, Client};
use kube::api::{DeleteParams, ListParams, PostParams};
use k8s_openapi::api::core::v1::{Pod, Container, ContainerState, ContainerStatus};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;
use chrono::{Utc, DateTime};
use log::{info, warn, error};
use thiserror::Error;

#[derive(Clone)]
pub struct KubernetesClient {
    namespace: String,
    pod: Arc<Mutex<Option<Pod>>>,
    pod_monitor: Arc<PodMonitor>,
    completion_result: Arc<Mutex<Option<CompletionResult>>>,
    tool_log: Arc<Mutex<Vec<LogEntry>>>,
}

#[derive(Debug, Clone)]
pub struct CompletionResult {
    pub exit_code: i32,
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub finish_time: Option<DateTime<Utc>>,
    pub log: Vec<LogEntry>,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub pod_name: String,
    pub entry: String,
}

impl KubernetesClient {
    pub async fn new(namespace: String, pod_monitor: Arc<PodMonitor>) -> Self {
        Self {
            namespace,
            pod: Arc::new(Mutex::new(None)),
            pod_monitor,
            completion_result: Arc::new(Mutex::new(None)),
            tool_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn submit_pod(&self, client: Client, pod_body: Pod) -> Result<(), KubernetesClientError> {
        let api: Api<Pod> = Api::namespaced(client, &self.namespace);
        let pod = api.create(&PostParams::default(), &pod_body).await?;

        info!("Created k8s pod name {} with uid {:?}", pod.metadata.name.clone().unwrap_or_default(), pod.metadata.uid);
        self.pod_monitor.add(pod.clone()).await;

        let mut current_pod = self.pod.lock().unwrap();
        *current_pod = Some(pod);
        Ok(())
    }

    pub fn should_delete_pod() -> bool {
        match env::var("CALRISSIAN_DELETE_PODS").unwrap_or_default().to_lowercase().as_str() {
            "false" | "no" | "0" => false,
            _ => true,
        }
    }

    pub async fn delete_pod_name(&self, client: Client, pod_name: &str) -> Result<(), KubernetesClientError> {
        let api: Api<Pod> = Api::namespaced(client, &self.namespace);
        match api.delete(pod_name, &DeleteParams::default()).await {
            Ok(_) => Ok(()),
            Err(kube::Error::Api(err)) if err.code == 404 => {
                // Pod not found, consider it already deleted
                Ok(())
            }
            Err(err) => Err(KubernetesClientError::KubeError(err)),
        }
    }

    pub async fn follow_logs(&self, client: Client) -> Result<(), KubernetesClientError> {
        let pod_name = self.pod.lock().unwrap()
            .as_ref()
            .and_then(|pod| pod.metadata.name.clone())
            .ok_or(KubernetesClientError::PodNotSet)?;

        info!("[{}] follow_logs start", pod_name);

        let api: Api<Pod> = Api::namespaced(client.clone(), &self.namespace);
        let log_stream = api.log_stream(&pod_name, &Default::default()).await?;

        tokio::pin!(log_stream);
        while let Some(line) = log_stream.next().await {
            match line {
                Ok(line) => {
                    let log_entry = String::from_utf8_lossy(&line).trim().to_string();
                    info!("[{}] {}", pod_name, log_entry);

                    let log_entry = LogEntry {
                        timestamp: Utc::now().to_rfc3339(),
                        pod_name: pod_name.clone(),
                        entry: log_entry,
                    };

                    self.tool_log.lock().unwrap().push(log_entry);
                }
                Err(err) => {
                    error!("[{}] Error reading logs: {:?}", pod_name, err);
                    break;
                }
            }
        }

        info!("[{}] follow_logs end", pod_name);
        Ok(())
    }

    pub async fn wait_for_completion(&self, client: Client) -> Result<CompletionResult, KubernetesClientError> {
        let pod_name = self.pod.lock().unwrap()
            .as_ref()
            .and_then(|pod| pod.metadata.name.clone())
            .ok_or(KubernetesClientError::PodNotSet)?;

        let api: Api<Pod> = Api::namespaced(client.clone(), &self.namespace);

        // Wait for pod completion
        loop {
            let pod = api.get(&pod_name).await?;
            if let Some(status) = pod.status {
                if let Some(phase) = status.phase {
                    if phase == "Succeeded" || phase == "Failed" {
                        info!("Pod {} has terminated with phase: {}", pod_name, phase);

                        let container_status = status.container_statuses.unwrap_or_default().get(0).cloned();
                        if let Some(state) = container_status.and_then(|status| status.state) {
                            self.handle_completion(state).await;
                        }

                        if Self::should_delete_pod() {
                            self.delete_pod_name(client.clone(), &pod_name).await?;
                            self.pod_monitor.remove(&pod_name).await;
                        }
                        break;
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }

        let completion_result = self.completion_result.lock().unwrap().clone();
        completion_result.ok_or(KubernetesClientError::IncompleteStatus)
    }

    async fn handle_completion(&self, state: ContainerState) {
        if let Some(terminated) = state.terminated {
            let exit_code = terminated.exit_code.unwrap_or(-1);
            let start_time = terminated.started_at;
            let finish_time = terminated.finished_at;

            let completion_result = CompletionResult {
                exit_code,
                cpu: None,   // Extract resource requests if needed
                memory: None, // Extract resource requests if needed
                start_time,
                finish_time,
                log: self.tool_log.lock().unwrap().clone(),
            };

            *self.completion_result.lock().unwrap() = Some(completion_result);
        }
    }
}

#[derive(Error, Debug)]
pub enum KubernetesClientError {
    #[error("Kubernetes client error: {0}")]
    KubeError(#[from] kube::Error),

    #[error("Pod not set")]
    PodNotSet,

    #[error("Incomplete pod status")]
    IncompleteStatus,
}
