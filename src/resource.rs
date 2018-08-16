use std::collections::HashMap;
use std::ops::Deref;
use yaml_hash_from_file;

const RESOURCES_FILE: &'static str = "Resources.yaml";

pub struct Resource {
    pub name: String,
    pub value: u32,
}

// Type alias because we'll be using this to build our ResourceMap internally.
type _ResourceMap = HashMap<String, Resource>;

// ResourceMap is a tuple of one element, which is our hashmap.
pub struct ResourceMap (_ResourceMap);

// For now ResourceMap just has a single load() function, that reads in
// all our resources.
impl ResourceMap {
    pub fn load() -> ResourceMap {
        return read_resources();
    }
}

// Here's the inheritance-like magic. Deref allows us to use our ResourceMap
// like a HashMap.
impl Deref for ResourceMap {

    // This to the Deref implementation what we can be used as.
    type Target = _ResourceMap;

    // And here's how we get our underlying struct.
    fn deref(&self) -> &_ResourceMap {
        &self.0
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

    return ResourceMap(map);
}
