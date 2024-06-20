use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

const IP: &str = "127.0.0.1";
const PORT: i32 = 7878;

const HTTP_200: &str = "HTTP/1.1 200 OK";

fn get_html_from_path(path: &str) -> String {
    return fs::read_to_string(path).unwrap();
}

fn generate_html_content(filename: &str) -> String {
    let contents = get_html_from_path(filename);
    let length = contents.len();

    // empty line is represented by "\r\n\r\n" in ASCII
    // line break is "\r\n"
    return format!("\r\nContent-Length: {length}\r\n\r\n{contents}");
}

fn handle_request(request_line: &str) -> (&str, &str) {
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => (HTTP_200, "html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            (HTTP_200, "html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "html/404.html"),
    };

    return (&status_line, &filename);
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {request_line:#?}");

    let (status_line, filename) = handle_request(&request_line);

    let content = generate_html_content(filename);

    let response = format!("{status_line}{content}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let server_address = format!("{IP}:{PORT}");
    let listener = TcpListener::bind(&server_address).unwrap();
    println!("Server start at: {server_address}");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

#[cfg(test)]
#[test]
fn get_request_works_correctly() {
    let test_cases = [
        ("GET /", HTTP_200, "html/hello.html"),
        ("GET /sleep", HTTP_200, "html/hello.html"),
        (
            "GET /not-implemented",
            "HTTP/1.1 404 NOT FOUND",
            "html/404.html",
        ),
    ];

    for (req_line, expected_status, expected_filename) in test_cases.iter() {
        let req = format!("{} HTTP/1.1", req_line);
        let (status, filename) = handle_request(&req);
        println!(
            "Request: {}, Status: {}, File: {}",
            req_line, status, filename
        );
        assert_eq!(&status, expected_status);
        assert_eq!(&filename, expected_filename);
    }
}
