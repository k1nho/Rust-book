use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request = reader.lines().next().unwrap().unwrap();
    //let http_request: Vec<_> = reader
    //.lines()
    //.map(|res| res.unwrap())
    //.take_while(|line| !line.is_empty())
    //.collect();

    let (status, filename) = match &request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index/html")
        }
        _ => ("HTTP/1.1 400 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(filename).unwrap();
    let content_len = content.len();
    let response = format!("{status}\r\nContent-Length: {content_len}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();

    //println!("Request: {:#?}", http_request);
}
