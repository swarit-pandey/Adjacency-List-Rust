use graph_builder;
use std::io;
use std::time::Instant;
use ansi_term::Colour::{Red, Green, Cyan};
use ansi_term::Style;

#[allow(unused_variables)]
fn main() {
    let edges: Vec<graph_builder::Edge> = initiate();

    let adj_list: Vec<Vec<graph_builder::Vertex>> = build_adj_list(&edges);
}

// Initiate building graph
fn initiate() -> Vec<graph_builder::Edge> {
    println!("Enter the number of edges: ");
    let size: i32 = graph_builder::init_response();
    let dummy_size = size;

    println!("Enter the edge data {}", Style::new().italic().paint("<src dest cost>"));
    let edge_data: Vec<graph_builder::Edge> = graph_builder::get_edge_data(dummy_size);
    return edge_data
}

// Build adjency list
fn build_adj_list(edge_data: & Vec<graph_builder::Edge>) -> Vec<Vec<graph_builder::Vertex>> {
    println!("To build directed weighted graph press {}", Cyan.paint("[D]"));
    println!("To build undirected weighted graph press {}", Cyan.paint("[U]"));

    let response: String = parse_input();

    if &response == "D" {
        let start = Instant::now();
        let directed_graph = graph_builder::build_digraph(&edge_data);
        let time_taken = start.elapsed(); 
        println!("{} {:?}", Green.paint("Build successful in"), (time_taken));

        println!("Press {} to print graph, otherwise {}", Cyan.paint("[Y]"), Cyan.paint("[N]"));

        let get_ip: String = parse_input();

        if get_ip == "Y" {
            graph_builder::print_digraph(&directed_graph);
        }

        directed_graph
    } else if &response == "U" {
        let start = Instant::now();
        let graph = graph_builder::build_graph(&edge_data);
        let time_taken = start.elapsed();
        println!("{} {:?}", Green.paint("Build successful in"), (time_taken));

        println!("Press {} to print graph, otherwise {}", Cyan.paint("[Y]"), Cyan.paint("[N]"));

        let get_ip: String = parse_input();
        
        if get_ip == "Y" {
            graph_builder::print_graph(&graph);
        }

        graph
    } else {
        panic!("{}", Red.paint("Build unseccessful (invalid response): ABORTING"));
    }
}

fn parse_input() -> String {
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Invalid");
    let ret: String = ip.trim().parse().expect("Invalid");
    ret
}