extern crate gstreamer as gst;
use hyper::{Body, Request, Response};

pub async fn handle(_req: Request<Body>) -> Response<Body> {
    let gst_version = gst::version_string();
    let output = format!("GStreamer Version: {}", gst_version);

    Response::new(Body::from(output))
}
