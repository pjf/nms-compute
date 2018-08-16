use std::collections::HashMap;
use yaml_hash_from_file;

const RESOURCES_FILE: &'static str = "Resources.yaml";

pub struct Resource {
    pub name: String,
    pub value: u32,
}

pub type ResourceMap = HashMap<String, Resource>;

// Rust doesn't let me add an impl to HashMap<x,y> because it's not defined in my crate
// (it's from std::), but I *can* add a trait and implementation.
// XXX - Is this the best way to do this?
pub trait Load<T> {
    fn load() -> T;
}

impl Load<ResourceMap> for ResourceMap {
    fn load() -> ResourceMap {
        return read_resources();
    }
}

// Reads our resources from the YAML configuration file.
fn read_resources() -> ResourceMap {
    let mut map = HashMap::new();
    let resources = yaml_hash_from_file(RESOURCES_FILE);

    for (name, value) in resources.iter() {

        // If there's no name then our YAML wouldn't have parsed.
        let name  = String::from(name.as_str().unwrap());

        let value = match value.as_i64() {
            None => panic!("Resource {} has no value", name),
            Some(val) => val as u32
        };

        // Add our resource to our map.
        map.insert(name.clone(),
            Resource {
                name,
                value
            }
        );
    }

    return map;
}
