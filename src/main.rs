use std::{net::SocketAddr, env};

use axum::{Router, routing::get};


#[tokio::main]
async fn main() {

    //port
    let port = match env::var("AXUM_PORT"){
        Ok(p) => {
            let p_u16 : u16 = p.parse().unwrap();
            p_u16
        },
        Err(_) => 7070,
    };

    let router = Router::new().route("/version", get(print_version));

    let addr = SocketAddr::from(([0,0,0,0],port));

    println!("===>> Server run on {addr} \n");

    axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .unwrap();
}

async fn print_version() -> String {
    String::from("Your version is v1")
}
