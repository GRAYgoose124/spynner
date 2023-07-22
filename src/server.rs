use crate::script_handler::ScriptHandler;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde_json::json;
use std::collections::HashMap;
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

fn run_script(script_name: &str, handler: Arc<ScriptHandler>) -> Result<Response<Body>> {
    let script_content = handler.load_script(script_name);

    if script_content.is_empty() {
        return Ok(Response::new(Body::from("Script not found")));
    }

    // execute script
    let result = handler.execute_script(&script_content);
    println!("Result: {}", result);

    // Convert the result to a JSON response
    let json_response = json!({ "result": result });
    Ok(Response::new(Body::from(json_response.to_string())))
}

async fn handle(req: Request<Body>, handler: Arc<ScriptHandler>) -> Result<Response<Body>> {
    let uri = req.uri();
    let path_segments: Vec<_> = uri.path().trim_start_matches('/').split('/').collect();

    // /<service>?
    match path_segments.as_slice() {
        ["script", script_name] => run_script(script_name, Arc::clone(&handler)),
        ["script"] => {
            // parse script_name query parameter
            let query_params: HashMap<String, String> = uri
                .query()
                .map(|v| form_urlencoded::parse(v.as_bytes()).into_owned().collect())
                .unwrap_or_default();

            let script_name = query_params
                .get("name")
                .map(|v| v.as_str())
                .ok_or_else(|| "Missing script_name query parameter".to_string())?;

            run_script(script_name, Arc::clone(&handler))
        }
        _ => Ok(Response::new(Body::from("Invalid Request"))),
    }
}
