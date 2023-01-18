use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();

    format!("http://127.0.0.1:{}", port);

    run(listener)
        .expect("Failed to bind to a random port")
        .await
}
