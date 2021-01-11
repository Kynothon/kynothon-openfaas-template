use hyper::{Body, Request, Response};

const PHRASE: &str = "Hello, Rust ⚙";

pub async fn handle(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
}
