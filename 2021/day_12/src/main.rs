use std::env;
use util::input_operations::{read_file_to_string,split_lines};
use std::collections::HashSet;
use itertools::Itertools;

trait IsLowercase {
    fn is_lowercase(&self) -> bool;
}

impl IsLowercase for &str {
    fn is_lowercase(&self) -> bool {
        self.bytes().all(|b| matches!(b, b'a'..=b'z'))
    }
}

impl IsLowercase for String {
    fn is_lowercase(&self) -> bool {
        self.bytes().all(|b| matches!(b, b'a'..=b'z'))
    }
}

trait HasDuplicates {
    fn has_lowercase_duplicates(&self) -> bool;
}

impl HasDuplicates for Vec<String> {
    fn has_lowercase_duplicates(&self) -> bool {
        self
            .iter()
            .filter(|x| x.is_lowercase())
            .filter(|x| self
                .iter()
                .filter(|y| x.eq(y))
                .count() == 2
            )
            .count() == 2
    }
}

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
struct Node {
    name: String,
    is_visited: bool
}

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
struct Edge {
    from: Node,
    to: Node
}

struct Graph {
    edges: HashSet<Edge>,
    nodes: Vec<Node>
}

impl Node {
    fn new(name: &str) -> Node {
        Node {
            name: String::from(name),
            is_visited: false
        }
    }
}

impl Edge {
    fn new(from: Node, to: Node) -> Edge {
        Edge {
            from: from,
            to: to
        }
    }
}

impl Graph {
    fn new(nodes: Vec<Node>, edges: HashSet<Edge>) -> Graph {
        Graph {
            nodes: nodes,
            edges:edges
        }
    }

    fn print_all_paths(&mut self, current: usize, end: usize, path: &mut Vec<String>, no_of_paths: &mut usize) {
        let node_name = self.nodes[current].name.clone();
        // Small caves should be visited once, except for one small cave
        //if node_name.is_lowercase() && (path.contains(&node_name) || path.has_lowercase_duplicates()) {
            //self.nodes[current].is_visited = true;
        //}
        path.push(node_name.clone());

        if current == end {
            println!("{:?}", path);
            *no_of_paths += 1;
        }
        else {
            for edge in self.edges
                .clone()
                .iter()
                .filter(|x| x.from.name.eq(&node_name)) {
                    let node_index = &self.nodes.iter()
                        .position(|x| x.name.eq(&edge.to.name))
                        .unwrap();
                    if !(path.contains(&self.nodes[*node_index].name) && path.has_lowercase_duplicates()) || !&self.nodes[*node_index].name.is_lowercase() {
                        self.print_all_paths(*node_index, end, path, no_of_paths)
                    }
            }
        }
        path.pop();        
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);
    let mut edges = split_lines(&input)
        .map(|x| x.split("-").collect::<Vec<&str>>())
        .map(|x| Edge::new(Node::new(x[0]), Node::new(x[1])))
        .collect::<HashSet<Edge>>();
    let nodes = edges
        .iter()
        .map(|x| vec![x.from.clone(), x.to.clone()])
        .flatten()
        .unique()
        .collect::<Vec<Node>>();
    // Also used inverted edges
    edges.extend(edges.iter()
        .filter(|x| !x.from.name.eq("start") && !x.to.name.eq("end"))
        .map(|x| Edge::new(x.to.clone(), x.from.clone()))
        .collect::<HashSet<Edge>>());
    let mut graph = Graph::new(nodes.clone(), edges);
    let paths = &mut Vec::<String>::new();
    let mut no_of_paths = 0;
    graph.print_all_paths(nodes.iter().position(|x| x.name == "start").unwrap(), 
        nodes.iter().position(|x| x.name == "end").unwrap(), paths, &mut no_of_paths);
    println!("{}", no_of_paths);
}