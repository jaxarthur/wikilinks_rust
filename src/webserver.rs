use std::sync::Arc;
use crate::data_structures::{Graph, GraphError};
use crate::{include_static_pages, string};
use crate::html_templates::{error_template};

use bytes::Bytes;
use h2::server::Connection;
use http::Response;
use tokio::runtime::Runtime;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::spawn_blocking;
use h2::server;
use url::Url;
use http::response::Builder;

use std::collections::HashMap;

struct Context {
    static_pages: HashMap<String, String>,
    graph: Graph
}

pub fn webserver_start() {
    let rt = Runtime::new().unwrap();
    rt.block_on(webserver_runtime())
}

async fn webserver_runtime() {
    let graph = Graph::new();
    let static_pages = include_static_pages!("simple.min.css");
    let context = Arc::new(Context{static_pages, graph});

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let conn_result = listener.accept().await;
        if conn_result.is_ok() {
            let (socket, _peer_ip) = conn_result.unwrap();
            let h2_result = server::handshake(socket).await;
            if h2_result.is_ok() {
                let h2 = h2_result.unwrap();
                let context = context.clone();
                spawn_blocking(move || {webserver_handle(h2, context)});
            } else {
                eprintln!("Handshake Error: {}", h2_result.unwrap_err())
            }
        } else {
            eprintln!("Listener Accept Error: {}", conn_result.unwrap_err())
        }
    }
}

async fn webserver_handle(mut h2: Connection<TcpStream, Bytes>, context: Arc<Context>) {
    let request_option = h2.accept().await;
    if request_option.is_some() {
        let request_result = request_option.unwrap();
        if request_result.is_ok() {
            let (request, mut send_response) = request_result.unwrap();
            let uri = request.uri();
            let url = Url::parse(&uri.to_string()).unwrap();
            let (response, data) = webserver_get_page(url).await;
            let mut test_stream = send_response.send_response(response, true).unwrap();
            test_stream.send_data(data, true).unwrap();
        }
    }
}

async fn webserver_get_page(url: Url) -> (Response<()>, Bytes) {
    let mut status_code: u16 = 200;
    let mut page = String::new();
    let path = url.path_segments();

    //Routing
    //Check if it root
    if (path.is_none()) {

    } else {
        status_code = 404;
        page = error_template(404, string!("Page Not Found"));
    }

    let response_builder = Builder::new();
    let response = response_builder.status(status_code).body(()).unwrap();
    let data = Bytes::from(page);
    return (response, data)
}

/*pub fn webserver_server() {
    let graph = Arc::new(Graph::load());
    let server = Server::webserver("0.0.0.0:8080").unwrap();
    let mut last_requests: HashMap<IpAddr, Instant> = HashMap::default();
    println!("Ready");

    for request in server.incoming_requests() {
        let address = request.remote_addr().ip();
        if last_requests.contains_key(&address) {
            let last_request = last_requests[&address];
            if Instant::now().duration_since(last_request) < Duration::from_secs(5) {
                webserver_respond(
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
            thread::spawn(move || webserver_thread(request, graph));
        }
    }
}

fn webserver_thread(request: Request, graph: Arc<Graph>) {
    // Parse url
    let url_string = String::from("webserver://0.0.0.0") + &String::from(request.url());
    let url = Url::parse(&url_string).unwrap();
    let mut pairs: HashMap<String, String> = HashMap::new();
    for pair in url.query_pairs() {
        pairs.insert(pair.0.to_string(), pair.1.to_string());
    }

    // Ensure valid data
    if !pairs.contains_key("from") || !pairs.contains_key("to") {
        webserver_respond(request, None, Some("Malformed Request"));
        return;
    }

    let path_result = graph.get_shortest_path(pairs["from"].clone(), pairs["to"].clone());

    if path_result.is_err() {
        let error = path_result.unwrap_err();
        if error == GraphError::LinkNotPresent {
            webserver_respond(request, None, Some("A URL is Invalid"));
        } else if error == GraphError::NoPath {
            webserver_respond(request, None, Some("No Path Between Provided URLs"));
        } else {
            webserver_respond(request, None, Some("Internal Error"));
        }
        return;
    }

    webserver_respond(request, Some(path_result.unwrap()), None);
}

fn webserver_respond(request: Request, path: Option<Vec<String>>, error: Option<&str>) {
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