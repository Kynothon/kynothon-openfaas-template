#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Request, Response, Server};

fn main() {
    // Configure a runtime that runs everything on the current thread
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    // Combine it with a `LocalSet,  which means it can spawn !Send futures...
    let local = tokio::task::LocalSet::new();
    local.block_on(&rt, run());
}

async fn handler_fn(_req: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(handler::handle(_req).await)
}

async fn run() {
    let addr = ([0, 0, 0, 0], 3000).into();

    let make_service = make_service_fn(|_conn| {
        async { Ok::<_, Error>(service_fn(handler_fn)) }
    });

    let server = Server::bind(&addr).executor(LocalExec).serve(make_service);

    println!("Listening on http://{}", addr);

    // The server would block on current thread to await !Send futures.
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

// Since the Server needs to spawn some background tasks, we needed
// to configure an Executor that can spawn !Send futures...
#[derive(Clone, Copy, Debug)]
struct LocalExec;

impl<F> hyper::rt::Executor<F> for LocalExec
where
    F: std::future::Future + 'static, // not requiring `Send`
{
    fn execute(&self, fut: F) {
        // This will spawn into the currently running `LocalSet`.
        tokio::task::spawn_local(fut);
    }
}
