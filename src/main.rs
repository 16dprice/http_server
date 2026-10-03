use http_server::{
    ThreadPool,
    routing::route_parser::{HTTPMethod, parse_request_line},
};
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

const PORT: usize = 7878;

fn main() {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();
    let pool = ThreadPool::new(4);

    println!("Server listening on port {}...", PORT);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down...");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let request_line = parse_request_line(request_line);

    let not_found_return_val = (404, fs::read_to_string("templates/404.html").unwrap());

    let (status_code, contents) = match request_line {
        Err(_) => not_found_return_val,
        Ok(r) => match r.method {
            HTTPMethod::GET => {
                let route_controller = r.route.to_controller();
                let res = route_controller();
                (res.status_code, res.contents)
            }
            _ => not_found_return_val,
        },
    };

    let length = contents.len();
    let status_msg = if status_code == 200 {
        "OK"
    } else {
        "NOT FOUND"
    };
    let response = format!(
        "HTTP/1.1 {status_code} {status_msg}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
