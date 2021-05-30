use ahash::{AHashMap, AHashSet};
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub enum GraphError {
    LinkNotPresent,
    VertexNotFound,
    NoPath,
}

impl Error for GraphError {
    fn description(&self) -> &str {
        match *self {
            GraphError::LinkNotPresent => "That link name is not present in graph",
            GraphError::NoPath => "No path is present between the two vertices.",
            GraphError::VertexNotFound => "The specified vertex could not be found.",
        }
    }
}

impl Display for GraphError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            GraphError::LinkNotPresent => write!(f, "That link name is not present in graph"),
            GraphError::NoPath => write!(f, "No path is present between the two vertices."),
            GraphError::VertexNotFound => write!(f, "The specified vertex could not be found."),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Vertex {
    link: String,
    connected: Vec<usize>,
}

impl Vertex {
    fn new(link: String) -> Vertex {
        return Vertex {
            link,
            connected: vec![],
        };
    }

    fn get_connected(&self) -> &Vec<usize> {
        return &self.connected;
    }

    fn add_connected(&mut self, connected: usize) {
        self.connected.push(connected)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Graph {
    vertices: Vec<Vertex>,
}

impl Graph {
    fn new() -> Graph {
        return Graph {
            vertices: Vec::default(),
        };
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
        return id < self.vertices.len();
    }

    fn find_by_link(&self, link: String) -> Result<usize, GraphError> {
        for (index, vertex) in self.vertices.iter().enumerate() {
            if vertex.link == link {
                return Ok(index);
            }
        }

        return Err(GraphError::LinkNotPresent);
    }

    fn get_neighbors(&self, id: usize) -> Result<&Vec<usize>, GraphError> {
        if self.has_vertex(id) {
            return Ok(self.vertices[id].get_connected());
        }

        return Err(GraphError::VertexNotFound);
    }

    pub fn convert() {
        let mut graph = Graph::new();
        let mut connections: Vec<Vec<usize>> = Vec::default();
        let mut reader = csv::ReaderBuilder::new()
            .flexible(true)
            .has_headers(false)
            .from_path("links.csv")
            .unwrap();

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

        let serialized = bincode::serialize(&graph).unwrap();
        let file = File::create("data.bin").unwrap();
        let mut encoder = DeflateEncoder::new(file, Compression::best());
        encoder.write_all(&serialized[..]).unwrap();
        encoder.finish().unwrap();
    }

    pub fn load() -> Graph {
        let file = File::open("data.bin").unwrap();
        let decoder = DeflateDecoder::new(file);
        return bincode::deserialize_from(decoder).unwrap();
    }

    pub fn get_shortest_path(
        &self,
        from_link: String,
        to_link: String,
    ) -> Result<Vec<String>, GraphError> {
        let from_id = self.find_by_link(from_link)?;
        let to_id = self.find_by_link(to_link)?;

        if !(self.has_vertex(from_id) && self.has_vertex(to_id)) {
            return Err(GraphError::VertexNotFound);
        }

        let mut layers: Vec<AHashSet<usize>> = vec![AHashSet::default()];
        layers[0].insert(from_id);

        let mut parents: AHashMap<usize, usize> = AHashMap::default();
        parents.insert(from_id, from_id);

        loop {
            let mut current_layer: AHashSet<usize> = AHashSet::default();
            for vertex_id in layers.get(layers.len() - 1).unwrap() {
                for other_id in self.get_neighbors(vertex_id.clone())? {
                    if !parents.contains_key(other_id) {
                        current_layer.insert(other_id.clone());
                        parents.insert(other_id.clone(), vertex_id.clone());
                    }
                }
            }

            if current_layer.len() == 0 {
                return Err(GraphError::NoPath);
            }

            layers.push(current_layer);

            if parents.contains_key(&to_id) {
                break;
            }
        }

        let mut path = vec![to_id];
        loop {
            let previous = path[path.len() - 1];
            let parent = parents[&previous];
            if parent == previous {
                break;
            }
            path.push(parent)
        }
        path.reverse();

        let link_path: Vec<String> = path
            .iter()
            .map(|id| self.vertices[id.clone()].link.clone())
            .collect();

        return Ok(link_path);
    }
}
