use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::fs;
use std::path::PathBuf;

pub struct ScriptHandler {
    script_root: PathBuf,
}

impl ScriptHandler {
    pub fn new(script_root: PathBuf) -> Self {
        Self { script_root }
    }

    pub fn load_script(&self, name: &str) -> String {
        let script_path = self.script_root.join(format!("{}.py", name));

        // Create a PathBuf from the script path
        let path_buf = PathBuf::from(script_path.clone());

        // Check if the script file exists before attempting to read it
        if !path_buf.exists() {
            eprintln!("Script file \'{}\' does not exist.", script_path.display());
            return "".to_string();
        }

        println!("Loading script file \'{}\'", script_path.display());

        // Read the script file into a string
        match fs::read_to_string(&path_buf) {
            Ok(script_content) => script_content,
            Err(e) => {
                eprintln!("Error reading script file \'{}\': {}", name, e);
                "".to_string()
            }
        }
    }

    pub fn execute_script(&self, script: &str) -> String {
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
}
