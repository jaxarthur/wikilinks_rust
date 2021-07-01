use std::sync::Arc;
use crate::data_structures::{Graph, GraphError};
use crate::include_static_pages;
use crate::html_templates::{OutlineTemplate, ErrorTemplate};

use tokio::runtime::Runtime;
use hyper::{Server, Request, Body, Response};
use hyper::server::conn::AddrStream;
use hyper::service::{service_fn, make_service_fn};
use askama::Template;
use std::net::SocketAddr;
use std::future::Future;
use std::convert::Infallible;
use std::collections::HashMap;

struct HttpContext {
    static_pages: HashMap<String, String>,

    graph: Graph
}

pub fn http_server() {
    let graph = Graph::new();
    let static_pages = include_static_pages!("simple.min.css");
    let context = Arc::new(HttpContext{static_pages, graph});
    let rt = Runtime::new().unwrap();
    rt.block_on(http_runtime(context))
}

async fn http_runtime(context: Arc<HttpContext>) {
    let addr = SocketAddr::from(([127,0,0,1], 8080));

    let service = make_service_fn(|conn: &AddrStream| async move {
        let context = context.clone();
        Ok::<_,Infallible>(service_fn(|req: Request<Body>| async move {http_handle(req, context)}))
    });
}

async fn http_handle(req: Request<Body>, context: Arc<HttpContext>) -> Result<Response<Body>, Infallible> {
    let body = Body::from("");
    let response = Response::new(body);
    return Ok(response);
}

/*pub fn http_server() {
    let graph = Arc::new(Graph::load());
    let server = Server::http("0.0.0.0:8080").unwrap();
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
*/