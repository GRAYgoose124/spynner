extern crate hyper;
extern crate pyo3;
extern crate rayon;

use pyo3::prelude::*;
// use rayon::prelude::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // load scripts
    let scripts = load_scripts();

    // share scripts across threads
    let shared_scripts = Arc::new(scripts);

    // create a service for handling incoming requests
    let make_svc = make_service_fn(move |_conn| {
        let scripts = Arc::clone(&shared_scripts);
        async { Ok::<_, hyper::Error>(service_fn(move |req| handle(req, Arc::clone(&scripts)))) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Running server on {:?}", addr);
    server.await?;

    Ok(())
}

async fn handle(
    req: Request<Body>,
    scripts: Arc<HashMap<String, String>>,
) -> Result<Response<Body>> {
    // get script name from request
    let script_name = get_script_name(&req);

    // find script
    match scripts.get(&script_name) {
        Some(s) => {
            // execute script
            let result = execute_script(s);
            println!("Result: {}", result);

            Ok(Response::new(Body::from(result)))
        }
        None => Ok(Response::new(Body::from("Script not found"))),
    }
}

fn load_scripts() -> HashMap<String, String> {
    let script_root = match std::env::current_exe() {
        Ok(mut path) => {
            path.pop();
            path.join("scripts")
        }
        Err(e) => {
            eprintln!("Error finding scripts directory: {}", e);
            return HashMap::new();
        }
    };

    let mut scripts = HashMap::new();
    let script_names = vec!["hello"];

    for script_name in script_names {
        let script_path = script_root.join(format!("{}.py", script_name));

        // Create a PathBuf from the script path
        let path_buf = PathBuf::from(script_path.clone());

        // Check if the script file exists before attempting to read it
        if !path_buf.exists() {
            eprintln!("Script file '{}' does not exist.", script_path.display());
            continue;
        }

        println!("Loading script file '{}'", script_path.display());

        // Read the script file into a string
        match fs::read_to_string(&path_buf) {
            Ok(script_content) => {
                // Insert the script content into the hashmap
                scripts.insert(script_name.to_string(), script_content);
            }
            Err(e) => {
                eprintln!("Error reading script file '{}': {}", script_name, e);
            }
        }
    }

    scripts
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
        py.run(script, None, Some(locals));
        locals
            .get_item("result")
            .unwrap()
            .extract::<String>()
            .unwrap_or_else(|_| "None".to_string())
    })
}
