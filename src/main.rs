use std::env;
use std::io::stdin;
use std::sync::Arc;
use std::thread;
use std::collections::HashMap;
use tiny_http::{Server, Method, Request, Response, Header};
use url::Url;
mod data_structures;
use data_structures::Graph;

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
    println!("Ready");

    for request in server.incoming_requests() {
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
    if !(pairs.contains_key("from") && pairs.contains_key("to")) {
        request.respond(Response::empty(400)).unwrap();
        return
    }

    let path_result = graph.get_shortest_path(pairs["from"].clone(), pairs["to"].clone());

    if path_result.is_err() {
        let error = path_result.unwrap_err();
        let error_message = error.to_string();
        let error_message_bytes = error_message.as_bytes();
        let header = Header::from_bytes(&b"Content-Type"[..], &b"text/plain; charset=UTF-8"[..]).unwrap();
        let response = Response::empty(500).with_header(header).with_data(error_message_bytes, Some(error_message_bytes.len()));
        request.respond(response).unwrap();
        return
    }

    let path = path_result.unwrap();
    let json_path = json::stringify(path);
    let json_path_bytes = json_path.as_bytes();
    let header = Header::from_bytes(&b"Content-Type"[..], &b"text/plain; charset=UTF-8"[..]).unwrap();
    let response = Response::empty(200).with_header(header).with_data(json_path_bytes, Some(json_path_bytes.len()));
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
