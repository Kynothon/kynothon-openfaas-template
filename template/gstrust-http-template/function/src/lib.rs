extern crate gstreamer as gst; 

use hyper::{Body, Request, Response};

pub async fn handle(_req: Request<Body>) -> Response<Body> {
    let version = gst::version_string();
    let output = format!("GStreamer Version: {} on Rust ⚙\n", version);


    Response::new(Body::from(output))
}
