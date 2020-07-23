# OpenFaaS Async Rust HTTP Template

This repository contains the template for OpenFaaS using the upgraded `of-watchdog` which allows for higher throughput.

```
$ faas template store pull https://github.com/GnaphronG/asyncrust-openfaas-http-template.git
$ faas new --list
Languages available as templates:
- asyncRust-http-template
```

This template uses `hyper` server. This allows additional context available in the request (by providing the full body to the handler) and more control over the response by passing it back to the HTTP reponse context.

## Using the template
First, pull the template with the faas CLI and create a new function:

```
$ faas template store pull https://github.com/GnaphronG/asyncrust-openfaas-http-template.git
$ faas-cli new --lang asyncRust-http-template <function name>
```

In the directory that was created, using the name of you function, you'll find `lib.rs`. It will look like this:

``` rust
use hyper::{Body, Request, Response};

const PHRASE: &str = "Hello, World!";

pub async fn handle(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
}
```

This is a simple implementation of a hello-world function. 

