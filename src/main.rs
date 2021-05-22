use std::env;
use std::io::stdin;
use std::io::Write;
use std::time::Instant;
use std::fs::File;
use ahash::{AHashMap, AHashSet};
use serde::{Serialize, Deserialize};
use flate2::Compression;
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;

#[derive(Serialize, Deserialize)]
struct Vertex {
    link: String,
    connected: Vec<usize>
}

impl Vertex{
    fn new(link: String) -> Vertex {
        return Vertex{link, connected: vec![] }
    }

    fn get_connected(&self) -> &Vec<usize> {
        return &self.connected
    }

    fn add_connected(&mut self, connected: usize) {
        self.connected.push(connected)
    }
}

#[derive(Serialize, Deserialize)]
struct Graph {
    vertices: Vec<Vertex>
}

impl Graph {
    fn new() -> Graph {
        return Graph{ vertices: Vec::default() }
    }

    fn add_vertex(&mut self, link: String) {
        self.vertices.push(Vertex::new(link))
    }

    fn add_connection(&mut self, from: usize, to: usize) {
        if self.has_vertex(from) && self.has_vertex(to) {
            self.vertices[from].add_connected(to)
        }
    }

    fn has_vertex(&self, id: usize) -> bool {
        return id < self.vertices.len()
    }

    fn find_by_link(&self, link: String) -> Result<usize, bool> {
        for (index, vertex) in self.vertices.iter().enumerate() {
            if vertex.link == link {
                return Ok(index);
            }
        }

        return Err(true)
    }

    fn get_neighbors(&self, id: usize) -> Result<&Vec<usize>, bool> {
        if self.has_vertex(id) {
            return Ok(self.vertices[id].get_connected());
        }

        return Err(true)
    }

    fn from_csv(path: &str) -> Graph {
        let mut graph = Graph::new();
        let mut connections: Vec<Vec<usize>> = Vec::default();
        let mut reader = csv::ReaderBuilder::new().flexible(true).has_headers(false).from_path(path).unwrap();

        println!("Reading in vertices and edges");

        for record in reader.records() {
            let record_data = record.unwrap();
            let line: Vec<&str> = record_data.iter().collect();
            graph.add_vertex(String::from(line[0]));
            connections.push(line[1..].iter().map(|s| s.parse().unwrap()).collect());
        }

        println!("Successfully loaded vertices");

        for (from_index, conns) in connections.iter().enumerate() {
            for to_index in conns {
                graph.add_connection(from_index, to_index.clone());
            }
        }

        println!("Successfully loaded edges");

        return graph
    }

    fn load() -> Graph {
        let bytes = include_bytes!("../data.bin");
        let decoder = DeflateDecoder::new(&bytes[..]);
        return bincode::deserialize_from(decoder).unwrap();
    }

    fn save(graph: Graph) {
        let serialized = bincode::serialize(&graph).unwrap();
        let file = File::create("data.bin").unwrap();
        let mut encoder = DeflateEncoder::new(file, Compression::best());
        encoder.write_all(&serialized[..]).unwrap();
        encoder.finish().unwrap();
    }

    fn get_shortest_path_ahash(&self, from_id: usize, to_id: usize) -> Result<Vec<usize>, bool> {
        if !(self.has_vertex(from_id) && self.has_vertex(to_id)) {
            return Err(true)
        }

        let mut layers: Vec<AHashSet<usize>> = vec![AHashSet::default()];
        layers[0].insert(from_id);

        let mut parents: AHashMap<usize, usize> = AHashMap::default();
        parents.insert(from_id, from_id);

        loop {
            let mut current_layer: AHashSet<usize> = AHashSet::default();
            for vertex_id in layers.get(layers.len() - 1).unwrap() {
                for other_id in self.get_neighbors(vertex_id.clone()).unwrap() {
                    if !parents.contains_key(other_id) {
                        current_layer.insert(other_id.clone());
                        parents.insert(other_id.clone(), vertex_id.clone());
                    }
                }
            }

            if current_layer.len() == 0 {
                return Err(true)
            }

            layers.push(current_layer);

            if parents.contains_key(&to_id) {
                break
            }
        }

        let mut path = vec![to_id];
        loop {
            let previous = path[path.len() - 1];
            let parent = parents[&previous];
            if parent == previous {
                break
            }
            path.push(parent)
        }
        path.reverse();
        
        return Ok(path)
    }
}

fn input() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line = String::from(line.trim());
    return line
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        default()
    } else if args[1] == "convert" {
        convert()
    } else if args[1] == "interactive" {
        interactive()
    } else {
        println!("Invalid Argument")
    }
}

fn default() {
    Graph::load();
}

fn convert() {
    let graph = Graph::from_csv("links.csv");
    Graph::save(graph)
}

fn interactive() {
    let graph = Graph::from_csv("links.csv");

    loop {
        let from_id;
        loop {
            println!("From:");
            let from_link = input();
            let from_result = graph.find_by_link(from_link);
            if from_result.is_err() {
                println!("That source does not exist in the database!");
                continue
            }
            from_id = from_result.unwrap();
            break
        }
        let to_id;
        loop {
            println!("To:");
            let from_link = input();
            let from_result = graph.find_by_link(from_link);
            if from_result.is_err() {
                println!("That destination does not exist in the database!");
                continue
            }
            to_id = from_result.unwrap();
            break
        }
        let start_time_new = Instant::now();
        let path_ids_result = graph.get_shortest_path_ahash(from_id, to_id);
        let end_time_new = Instant::now();
        if path_ids_result.is_err() {
            println!{"There is no path between the two"}
            continue
        }
        let path_ids = path_ids_result.unwrap();
        let mut path_links: Vec<String> = vec![];
        for path_id in path_ids {
            path_links.push(graph.vertices[path_id].link.clone())
        }
        println!{"{:?}", path_links}

        println!{"Times:"};
        println!{"New: {:?}", end_time_new.duration_since(start_time_new)}
    }
}
