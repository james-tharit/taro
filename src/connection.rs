use std::fmt::Error;
use std::time::Duration;
use std::{fs, thread};
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

pub(crate) const HTTP_200: &str = "HTTP/1.1 200 OK";

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

pub(crate) fn handle_request(request_line: &str) -> (&str, &str) {
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

fn middleware(content: &String) -> Result<String, Error> {
    let modified_content = content.to_owned();
    Ok(modified_content)
}

pub(crate) fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {request_line:#?}");

    match middleware(&request_line) {
        Ok(modified) => println!("Modified content: {}", modified),
        Err(e) => eprintln!("Error: {}", e),
    }

    let (status_line, filename) = handle_request(&request_line);

    let content = generate_html_content(filename);

    let response = format!("{status_line}{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
