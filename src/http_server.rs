pub fn spawn_http_server() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = "0.0.0.0";

    tokio::spawn(async move {
        let server = simple_server::Server::new(|_request, mut response| {
            Ok(response.body(b"Hello Rust!".to_vec()).unwrap())
        });
        server.listen(address, &port);
    });
}
