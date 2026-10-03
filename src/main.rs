use std::{
    fs, io::{BufReader, prelude::*}, net::{TcpListener, TcpStream}, thread::{self, sleep}, time::Duration,
};
use http_server::{ThreadPool, routing::{route_parser::{HTTPMethod, parse_request_line}, routes::Route}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

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

    let not_found_return_val = ("HTTP/1.1 404 NOT FOUND", "templates/404.html");

    let (status_line, filename) = match request_line {
        Err(_) => not_found_return_val,
        Ok(r) => match r.method {
            HTTPMethod::GET => match r.route {
                Route::Root => ("HTTP/1.1 200 OK", "templates/hello.html"),
                Route::Sleep => {
                    thread::sleep(Duration::from_secs(5));
                    ("HTTP/1.1 200 OK", "templates/hello.html")
                }
            }
            _ => { not_found_return_val }
        }
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
