use crate::model::Node;
use crate::Params;

pub fn print_params(params: &Params) {
    if !params.silent {
        dbg!(params);
    }
}

pub fn print_public_rpc_nodes_stats(nodes: &Vec<Node>, silent: bool) {
    if !silent {
        println!("public rpc nodes: {}", nodes.len());
    }
}

pub fn print_good_rpc_nodes_stats(nodes: &Vec<Node>, silent: bool) {
    if !silent {
        println!("good rpc nodes: {}", nodes.len());
        dbg!(nodes);
    }
}
