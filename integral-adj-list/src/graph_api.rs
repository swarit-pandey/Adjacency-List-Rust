pub struct Edge {
    src: i32,
    dest: i32,
    cost: i32,
}

pub struct Vertex {
    label: i32,
    cost: i32,
}

use std::io;

/// Initiate response, and get the number of edges in graph
pub fn init_response() -> i32 {
    let n: i32 = get_number_of_edges();
    return n
}

// Takes input from console
pub fn get_number_of_edges() -> i32 {
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Invalid");
    let ret = ip.trim().parse().expect("Internal Error");
    return ret
}

/// Get data related to edges <src, dest, cost>
pub fn get_edge_data(n: i32) -> Vec<Edge> {
    let mut keep_edge_data: Vec<Edge> = Vec::new();

    let mut track = n;
    while track > 0 {
        let edge_data: Vec<i32> = parse_edge_data();

        let edges_be = Edge {
            src: edge_data[0],
            dest: edge_data[1],
            cost: edge_data[2],
        };

        keep_edge_data.push(edges_be);
        track -= 1;
    }

    return keep_edge_data
}    

// Takes input from console
pub fn parse_edge_data() -> Vec<i32> {
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Invalid");
    let ret: Vec<i32> = ip.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
    return ret
}

/// Builds a directed weighted graph, takes vector of edges as
/// input. 
pub fn build_digraph(vector_of_edges: &Vec<Edge>) -> Vec<Vec<Vertex>> {
    // To get the size of the adj_list
    let size = get_size(vector_of_edges);

    // Attach a vector to a vector, to create an adj_list
    let mut adj_list: Vec<Vec<Vertex>> = attach_vec(size);

    for e in vector_of_edges {
        let node = Vertex {
            label: e.dest,
            cost: e.cost,
        };

        let src_idx = e.src as usize;
        adj_list[src_idx].push(node);
    }

    return adj_list
}

pub fn build_graph(vector_of_edges: &Vec<Edge>) -> Vec<Vec<Vertex>> {
    // To get the size of the adj_list
    let size = get_size(vector_of_edges);

    // Attach a vector to vector, to create an adj_list
    let mut adj_list = attach_vec(size);

    for e in vector_of_edges {
        let src_idx = e.src as usize;
        let dest_idx = e.dest as usize;

        let node = Vertex {
            label: e.dest,
            cost: e.cost,
        };
        adj_list[src_idx].push(node);

        let node = Vertex {
            label: e.src,
            cost: e.cost,
        };
        adj_list[dest_idx].push(node);
    }

    return adj_list
}

pub fn get_size(edge_data: &Vec<Edge>) -> i32 {
    let mut size = i32::MIN;

    for e in edge_data{
        size = std::cmp::max(size, std::cmp::max(e.dest, e.src));
    }
    size += 1;
    return size
}

pub fn attach_vec(size: i32) -> Vec<Vec<Vertex>> {
    let mut adj_list_ret: Vec<Vec<Vertex>> = Vec::new();
    
    let mut track = size;
    while track > 0 {
        let new_vec: Vec<Vertex> = Vec::new();
        adj_list_ret.push(new_vec);
        track -= 1;
    }

    return adj_list_ret
}

use ansi_term::Colour::{Red};

/// To print digraph
pub fn print_digraph(adj_list: &Vec<Vec<Vertex>>) {
    let mut vec_size = adj_list.len();
    let mut idx: usize = 0;

    while vec_size > 0 {
        for v in &adj_list[idx] {
            print!("{:?} {} {:?}({:?}) ", idx, Red.paint("->"), v.label, v.cost);
        }
        idx += 1;
        vec_size -= 1;
        println!();
    }
    println!();
}

/// To print undirected graph
pub fn print_graph(adj_list: &Vec<Vec<Vertex>>) {
    let mut vec_size = adj_list.len();
    let mut idx: usize = 0;

    while vec_size > 0 {
        for v in &adj_list[idx] {
            print!("{:?} {} {:?}({:?}) ", idx, Red.paint("<->"), v.label, v.cost);
        }
        idx += 1;
        vec_size -= 1;
        println!();
    }
    println!();
}

