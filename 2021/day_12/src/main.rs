use std::env;
use util::input_operations::{read_file_to_string,split_lines};
use std::collections::HashSet;

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


    fn print_all_paths(&mut self, u: usize, d: usize, path: &mut Vec<String>, no_of_paths: &mut usize) {
        let node_name = self.nodes[u].name.clone();
        // Only small caves should be visited once
        if node_name.bytes().all(|b| matches!(b, b'a'..=b'z')) {
            self.nodes[u].is_visited = true;
        }
        path.push(node_name.clone());

        if u == d {
            println!("{:?}", path);
            *no_of_paths += 1;
        }
        else {
            for edge in self.edges.clone().iter().filter(|x| x.from.name.eq(&node_name)) {
                if !self.nodes.iter().find(|x| x.name.eq(&edge.to.name)).unwrap().is_visited {
                    let node_index = &self.nodes.iter().position(|x| x.name.eq(&edge.to.name)).unwrap();
                    self.print_all_paths(*node_index, d, path, no_of_paths)
                }
            }
        }
        path.pop();
        self.nodes[u].is_visited = false;
        
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
        .collect::<Vec<Node>>();
    // Also used invert edges
    edges.extend(edges.iter().filter(|x| !x.from.name.eq("start") && !x.to.name.eq("end")).map(|x| Edge::new(x.to.clone(), x.from.clone())).collect::<HashSet<Edge>>());
    let mut graph = Graph::new(nodes.clone(), edges);
    let paths = &mut Vec::<String>::new();
    let mut no_of_paths = 0;
    graph.print_all_paths(nodes.iter().position(|x| x.name == "start").unwrap(), 
        nodes.iter().position(|x| x.name == "end").unwrap(), paths, &mut no_of_paths);
    println!("{}", no_of_paths);
}