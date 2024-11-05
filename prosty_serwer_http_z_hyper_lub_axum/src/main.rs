//use axum::{
//    routing::get,
//    Router,
//};
//use std::net::SocketAddr;
//
//#[tokio::main]
//async fn main() {
//    // Tworzenie tras
//    let app = Router::new()
//        .route("/", get(root))
//        .route("/hello", get(hello)) // Obsługuje tylko GET dla /hello
//        .route("/status", get(status));
//
//    // Ustawienie adresu serwera
//    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//    println!("Serwer uruchomiony na http://{}", addr);
//
//    // Uruchomienie serwera
//    axum::Server::bind(&addr)
//        .serve(app.into_make_service())
//        .await
//        .unwrap();
//}


use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler dla trasy "/"
async fn root() -> &'static str {
    "Welcome to my server"
}

// Handler dla trasy "/hello" (GET)
async fn hello() -> &'static str {
    "Hello, World!"
}

// Handler dla trasy "/status"
async fn status() -> &'static str {
    "Server is running"
}


//// hyper
//
//use hyper::{Body, Request, Response, Server};
//use hyper::service::{make_service_fn, service_fn};
//use std::convert::Infallible;
//use std::net::SocketAddr;
//
//#[tokio::main]
//async fn main() {
//    // Ustawienie adresu serwera
//    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//    println!("Serwer uruchomiony na http://{}", addr);
//
//    // Tworzenie serwera z handlerem dla różnych tras
//    let make_svc = make_service_fn(|_conn| async {
//        Ok::<_, Infallible>(service_fn(router))
//    });
//
//    // Uruchomienie serwera
//    let server = Server::bind(&addr).serve(make_svc);
//
//    // Oczekiwanie na zakończenie pracy serwera
//    if let Err(e) = server.await {
//        eprintln!("Błąd serwera: {}", e);
//    }
//}
//
//// Router - obsługuje różne trasy
//async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//    match req.uri().path() {
//        "/" => Ok(Response::new(Body::from("Welcome to my server"))),
//        "/hello" => Ok(Response::new(Body::from("Hello, World!"))),
//        "/status" => Ok(Response::new(Body::from("Server is running"))),
//        _ => Ok(Response::new(Body::from("404 Not Found"))),
//    }
//}
