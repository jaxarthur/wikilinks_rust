use json::object;
use std::collections::HashMap;
use std::env;
use std::io::stdin;
use std::net::IpAddr;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tiny_http::{Header, Method, Request, Response, Server};
use url::Url;
mod data_structures;
use data_structures::{Graph, GraphError};

fn input() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line = String::from(line.trim());
    return line;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        http_server()
    } else if args[1] == "convert" {
        convert()
    } else if args[1] == "interactive" {
        interactive()
    } else {
        println!("Invalid Argument")
    }
}

fn http_server() {
    let graph = Arc::new(Graph::load());
    let server = Server::http("0.0.0.0:8000").unwrap();
    let mut last_requests: HashMap<IpAddr, Instant> = HashMap::default();
    println!("Ready");

    for request in server.incoming_requests() {
        let address = request.remote_addr().ip();
        if last_requests.contains_key(&address) {
            let last_request = last_requests[&address];
            if Instant::now().duration_since(last_request) < Duration::from_secs(5) {
                http_respond(
                    request,
                    None,
                    Some("Please wait before submitting another query."),
                );
                continue;
            }
        }
        last_requests.insert(address, Instant::now());

        if request.method() == &Method::Get {
            let graph = Arc::clone(&graph);
            thread::spawn(move || http_thread(request, graph));
        }
    }
}

fn http_thread(request: Request, graph: Arc<Graph>) {
    // Parse url
    let url_string = String::from("http://0.0.0.0") + &String::from(request.url());
    let url = Url::parse(&url_string).unwrap();
    let mut pairs: HashMap<String, String> = HashMap::new();
    for pair in url.query_pairs() {
        pairs.insert(pair.0.to_string(), pair.1.to_string());
    }

    // Ensure valid data
    if !pairs.contains_key("from") || !pairs.contains_key("to") {
        http_respond(request, None, Some("Malformed Request"));
        return;
    }

    let path_result = graph.get_shortest_path(pairs["from"].clone(), pairs["to"].clone());

    if path_result.is_err() {
        let error = path_result.unwrap_err();
        if error == GraphError::LinkNotPresent {
            http_respond(request, None, Some("A URL is Invalid"));
        } else if error == GraphError::NoPath {
            http_respond(request, None, Some("No Path Between Provided URLs"));
        } else {
            http_respond(request, None, Some("Internal Error"));
        }
        return;
    }

    http_respond(request, Some(path_result.unwrap()), None);
}

fn http_respond(request: Request, path: Option<Vec<String>>, error: Option<&str>) {
    let object = object!("path": path, "error": error);
    let object_dump = object.dump();
    let data = object_dump.as_bytes();
    let header =
        Header::from_bytes(&b"Content-Type"[..], &b"text/plain; charset=UTF-8"[..]).unwrap();
    let response = Response::empty(200)
        .with_header(header)
        .with_data(data, Some(data.len()));
    request.respond(response).unwrap();
}

fn convert() {
    Graph::convert()
}

fn interactive() {
    let graph = Graph::load();

    loop {
        println!("From: ");
        let from_link = input();
        println!("To: ");
        let to_link = input();
        let path = graph.get_shortest_path(from_link, to_link);
        if path.is_err() {
            println!("An error occurred: {}", path.unwrap_err())
        } else {
            println!("{:?}", path.unwrap());
        }
    }
}
