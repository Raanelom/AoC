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
    name: String
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
            name: String::from(name)
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

    fn other_node(&self, this_node: &str) -> Option<&Node> {
        if self.from.name.eq(&this_node) {
            return Some(&self.to);
        }
        if self.to.name.eq(&this_node) {
            return Some(&self.from);
        }
        return None
    }

    fn contains(&self, node_name: &str) -> bool {
        return (self.to.name.eq(node_name) || self.from.name.eq(node_name)) 
            && !self.other_node(node_name).unwrap().name.eq("start")
            && !node_name.eq("end");
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
        path.push(node_name.clone());

        if current == end {
            println!("{:?}", path);
            *no_of_paths += 1;
        }
        else {
            for edge in self.edges
                .clone()
                .iter()
                .filter(|x| x.contains(&node_name)) {
                    let node_index = &self.nodes.iter()
                        .position(|x| x.name.eq(&edge.other_node(&node_name).unwrap().name))
                        .unwrap();
                    if !(path.contains(&self.nodes[*node_index].name) && path.has_lowercase_duplicates()) 
                        || !&self.nodes[*node_index].name.is_lowercase() {
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
    // First determine the edges
    let edges = split_lines(&input)
        .map(|x| x.split("-").collect::<Vec<&str>>())
        .map(|x| Edge::new(Node::new(x[0]), Node::new(x[1])))
        .collect::<HashSet<Edge>>();
    // Then the nodes
    let nodes = edges
        .iter()
        .map(|x| vec![x.from.clone(), x.to.clone()])
        .flatten()
        .unique()
        .collect::<Vec<Node>>();
    let start_position = nodes.iter().position(|x| x.name == "start").unwrap();
    let end_position = nodes.iter().position(|x| x.name == "end").unwrap();
    let mut graph = Graph::new(nodes, edges);
    let mut no_of_paths = 0;
    graph.print_all_paths(start_position, end_position, &mut Vec::<String>::new(), &mut no_of_paths);
    println!("{}", no_of_paths);
}