mod andy_error;
mod args;

use andy_error::AndyError;

use clap::Parser;

use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::io::Read;
use std::net::SocketAddr;

fn get_content_type(path: &std::path::Path) -> Result<&str, AndyError> {
    if let Some(ext) = path.extension() {
        match ext.to_str().ok_or(AndyError::BadUriPath)? {
            "html" => Ok("text/html"),
            "css" => Ok("text/css"),
            "js" => Ok("text/javascript"),
            "wasm" => Ok("application/wasm"),
            _ => Ok("application/octet-stream"),
        }
    } else {
        Ok("application/octet-stream")
    }
}

async fn handle(
    req: Request<Body>,
    path: std::sync::Arc<std::path::PathBuf>,
) -> Result<Response<Body>, AndyError> {
    let path_end = std::path::Path::new(&req.uri().path()[1..]);
    let path = path.as_path().join(path_end);
    let mut f = std::fs::File::open(path)?;
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data)?;

    let content_type = get_content_type(path_end)?;

    Ok(hyper::Response::builder()
        .status(hyper::StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("content-type", content_type)
        .body(data.into())?)
}

#[tokio::main]
async fn main() -> Result<(), AndyError> {
    let args = args::AndyArgs::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    let path = std::sync::Arc::new(args.root_serve_path.clone());
    let make_service = make_service_fn(move |_conn| {
        let path = path.clone();
        async move { Ok::<_, Infallible>(service_fn(move |body| handle(body, path.clone()))) }
    });

    let server = Server::bind(&addr).serve(make_service);

    server.await?;

    eprintln!("关了");

    Ok(())
}
