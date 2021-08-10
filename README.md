# OpenFaaS GStreamer Async Rust HTTP Template

This repository contains the template for OpenFaaS using the upgraded `of-watchdog` which allows for higher throughput.

```
$ faas template pull https://github.com/Kynothon/gst-rust-http-template.git
$ faas new --list
Languages available as templates:
- grtust-http-template
```

This template uses `hyper` server. This allows additional context available in the request (by providing the full body to the handler) and more control over the response by passing it back to the HTTP reponse context.

## Using the template
First, pull the template with the faas CLI and create a new function:

```
$ faas template pull https://github.com/Kynothon/gstrust-http-template.git
$ faas-cli new --lang gstrust-http-template <function name>
```

In the directory that was created, using the name of you function, you'll find `lib.rs`. It will look like this:

``` rust
extern crate gstreamer as gst;
use hyper::{Body, Request, Response};

pub async fn handle(_req: Request<Body>) -> Response<Body> {
    let gst_version = gst::version_string();
    let output = format!("GStreamer Version: {}", gst_version);

    Response::new(Body::from(output))
}
```

This is a simple implementation of a hello-world function. 

