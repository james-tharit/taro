#[test]
fn get_request_works_correctly() {
    let test_cases = [
        ("GET /", "HTTP/1.1 200 OK", "html/hello.html"),
        ("GET /sleep", "HTTP/1.1 200 OK", "html/hello.html"),
        (
            "GET /not-implemented",
            "HTTP/1.1 404 NOT FOUND",
            "html/404.html",
        ),
    ];

    for (req_line, expected_status, expected_filename) in test_cases.iter() {
        let req = format!("{} HTTP/1.1", req_line);
        let (status, filename) = crate::connection::handle_request(&req);
        println!(
            "Request: {}, Status: {}, File: {}",
            req_line, status, filename
        );
        assert_eq!(&status, expected_status);
        assert_eq!(&filename, expected_filename);
    }
}
