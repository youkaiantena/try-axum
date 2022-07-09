use axum::{ extract::Query, response::Json, routing::get, Router };
use rand::{ thread_rng, Rng };
use serde::{ Serialize, Deserialize };
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

#[derive(Serialize)]
struct RandomNumber {
    number: usize,
}

async fn handler(Query(range): Query<RangeParameters>) -> Json<RandomNumber> {
    // Generate a random number in range parsed from query.
    let random_number = RandomNumber {
        number: thread_rng().gen_range(range.start..range.end)
    };

    // Send response in html format.
    Json(random_number)
}
