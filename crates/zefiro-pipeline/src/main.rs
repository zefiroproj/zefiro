use petgraph::graph::DiGraph;
use petgraph::algo::toposort;
use petgraph::visit::IntoNodeReferences;

fn main() {
    let mut graph = DiGraph::new();
    
    let calib_node = graph.add_node("calib");
    let fastp_node = graph.add_node("fastp");
    let vidjil_node = graph.add_node("vidjil");
    let igblast_node = graph.add_node("igblast");
    let corrector_node = graph.add_node("cdr3nt-error-corrector");

    graph.add_edge(calib_node, vidjil_node, ());
    graph.add_edge(fastp_node, vidjil_node, ());
    graph.add_edge(vidjil_node, igblast_node, ());
    graph.add_edge(fastp_node, corrector_node, ());
    graph.add_edge(igblast_node, corrector_node, ());

    let sorted = toposort(&graph, None).expect("Graph is not a DAG!");
    println!("Topological order: {:?}", sorted);

    let entry_points: Vec<_> = graph.node_references()
        .filter(|(node, _)| graph.edges_directed(*node, petgraph::Incoming).count() == 0)
        .map(|(_, name)| name)
        .collect();
    println!("Entry points: {:?}", entry_points);
}
