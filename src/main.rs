use std::env;
use std::io::stdin;
mod macros;
mod data_structures;
use data_structures::Graph;
mod webserver;
use webserver::webserver_start;
mod html_templates;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        webserver_start()
    } else if args[1] == "convert" {
        convert()
    } else if args[1] == "interactive" {
        interactive()
    } else {
        println!("Invalid Argument")
    }
}

fn convert() {
    Graph::convert()
}

fn input() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line = String::from(line.trim());
    return line;
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
