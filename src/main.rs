use std::net::TcpListener;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let listner:TcpListener = TcpListener::bind("127.0.0.1:8000")?;

    run(listner)?.await
}
