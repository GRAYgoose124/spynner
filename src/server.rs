use crate::script_handler::ScriptHandler;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde_json::json;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn start_server(script_handler: Arc<ScriptHandler>) -> Result<()> {
    // create a service for handling incoming requests
    let make_svc = make_service_fn(move |_conn| {
        let handler = Arc::clone(&script_handler);
        async { Ok::<_, hyper::Error>(service_fn(move |req| handle(req, Arc::clone(&handler)))) }
    });

    let addr = ([127, 0, 0, 1], 3001).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Running server on {:?}", addr);
    server.await?;

    Ok(())
}

async fn handle(req: Request<Body>, handler: Arc<ScriptHandler>) -> Result<Response<Body>> {
    // get script name from request
    let script_name = get_script_name(&req);

    let script_content = handler.load_script(&script_name);

    if script_content.is_empty() {
        return Ok(Response::new(Body::from("Script not found\n")));
    }

    // execute script
    let result = handler.execute_script(&script_content);
    println!("Result: {}", result);

    // Convert the result to a JSON response
    let json_response = json!({ "result": result });
    Ok(Response::new(Body::from(json_response.to_string())))
}

fn get_script_name(req: &Request<Body>) -> String {
    // replace this with actual script name extraction
    let name = req.uri().path().trim_start_matches('/').to_string();
    println!("Script name: {}", name);
    name
}
