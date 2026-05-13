use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use oxrdf::Triple;
use crate::error::ReadError;
use serde_json::Value;
mod json_import;
mod error;
mod graph_import;

pub fn read_json(path: &String) -> Result<Value, ReadError> {
    match json_import::read_json(path) {
        Ok(file) => {Ok(file)}
        Err(err) => {
            Err(err)
        }
    }
}
fn prefix_to_path() -> Result<HashMap<String, PathBuf>, ReadError>{
    let mut  prefix_to_path = HashMap::new();
    let file_path = "assets/prefix_file_name.json".to_string();
    let file = read_json(&file_path)?;
    match file {
        Value::Object(object) => {
            for (prefix, file_name) in object {
                match file_name {
                    Value::String(file_name) => {
                        let complete_path = complete_path(file_name);
                        prefix_to_path.insert(prefix, complete_path);
                    }
                    _ => {
                        return Err(ReadError::FileContent(format!("File '{}' should contain a string for 'file_name', but found: '{}'", file_path, file_name)));
                    }
                }
            }
        }
        _ => {
            return Err(ReadError::FileContent(format!("File '{:?}' expected to be Object, but it is not: '{:?}'",file_path, file)));
        }
    }
    Ok(prefix_to_path)
}

fn complete_path(file_name: String) -> PathBuf {
    ["assets", "ontologies", &file_name].iter().collect()

}

pub fn prefix_to_graph(prefixes: Vec<String>) -> Result<HashMap<String, String>, ReadError> {
    let prefix_to_path = prefix_to_path()?;
    let mut prefix_to_graph: HashMap<String, Vec<Triple>> = HashMap::new();
    for prefix in prefixes {
        match prefix_to_path.get(&prefix) {
            None => {
                println!("Prefix {} not found in prefix_to_path, ignored. Add first to 'assets/prefix_file_name.json'.", prefix);
            }
            Some(path) => {

                match path.extension(){
                    None => {
                        return Err(ReadError::FilePath(format!("File must have an extension: '{:?}'", path)));
                    }
                    Some(extension) => {
                        match extension.to_str().unwrap() {
                            "rdf" => {
                                let graph = graph_import::read_rdf_xml_to_triples(&path.to_str().unwrap().to_string())?;
                                prefix_to_graph.insert(prefix.clone(), graph);
                            }
                            &_ => {
                                return Err(ReadError::FilePath(format!("File extension  unknown: '{:?}'", extension)));
                            }
                        }
                    }
                }
            }
        }
    }
    todo!()
    //Ok(prefix_to_graph)
}
