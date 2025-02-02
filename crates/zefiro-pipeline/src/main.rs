use petgraph::algo::toposort;
use petgraph::visit::IntoNodeReferences;

use zefiro_cwl::CwlSchema;

fn main() {
    let file_path = "../zefiro-cwl/test_data/cwl/wf-schema.yml";

    if let CwlSchema::Workflow(wf) =
        CwlSchema::from_path(file_path).expect("Failed to deserialize CWL schema")
    {
        let graph = wf.to_graph();
        let sorted = toposort(&graph, None).expect("Graph is not a DAG!");
        println!("Topological order: {:?}", sorted);

        let entry_points: Vec<_> = graph
            .node_references()
            .filter(|(node, _)| graph.edges_directed(*node, petgraph::Incoming).count() == 0)
            .map(|(_, name)| name)
            .collect();
        println!("Entry points: {:?}", entry_points);
    }
}
