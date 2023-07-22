extern crate hyper;
extern crate pyo3;
extern crate rayon;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // create a service for handling incoming requests
    let make_svc = make_service_fn(move |_conn| async {
        Ok::<_, hyper::Error>(service_fn(move |req| handle(req)))
    });

    let addr = ([127, 0, 0, 1], 3001).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Running server on {:?}", addr);
    server.await?;

    Ok(())
}

async fn handle(req: Request<Body>) -> Result<Response<Body>> {
    // get script name from request
    let script_name = get_script_name(&req);

    let script_content = load_script(&script_name);

    if script_content.is_empty() {
        return Ok(Response::new(Body::from("Script not found\n")));
    }

    // execute script
    let result = execute_script(&script_content);
    println!("Result: {}", result);

    Ok(Response::new(Body::from(format!("{}\n", result))))
}

fn load_script(name: &str) -> String {
    let script_root = match std::env::current_exe() {
        Ok(mut path) => {
            path.pop();
            path.join("scripts")
        }
        Err(e) => {
            eprintln!("Error finding scripts directory: {}", e);
            return "".to_string();
        }
    };

    let script_path = script_root.join(format!("{}.py", name));

    // Create a PathBuf from the script path
    let path_buf = PathBuf::from(script_path.clone());

    // Check if the script file exists before attempting to read it
    if !path_buf.exists() {
        eprintln!("Script file '{}' does not exist.", script_path.display());
        return "".to_string();
    }

    println!("Loading script file '{}'", script_path.display());

    // Read the script file into a string
    match fs::read_to_string(&path_buf) {
        Ok(script_content) => script_content,
        Err(e) => {
            eprintln!("Error reading script file '{}': {}", name, e);
            "".to_string()
        }
    }
}

fn get_script_name(req: &Request<Body>) -> String {
    // replace this with actual script name extraction
    let name = req.uri().path().trim_start_matches('/').to_string();
    println!("Script name: {}", name);
    name
}

fn execute_script(script: &str) -> String {
    Python::with_gil(|py| {
        let locals = PyDict::new(py);
        py.run(script, None, Some(locals)).unwrap();
        // attempt to get locals()['result'] or return "None" as a string
        match locals.get_item("result") {
            Some(result) => result.str().unwrap().to_string(),
            None => "None".to_string(),
        }
    })
}
