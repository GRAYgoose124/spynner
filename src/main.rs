mod script_handler;
mod server;

use crate::script_handler::ScriptHandler;
use crate::server::start_server;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error + Send + Sync>> {
    let script_root = match env::current_exe() {
        Ok(mut path) => {
            path.pop();
            path.join("scripts")
        }
        Err(e) => {
            eprintln!("Error finding scripts directory: {}", e);
            return Err(e.into());
        }
    };

    let handler = Arc::new(ScriptHandler::new(script_root));

    start_server(handler).await
}
