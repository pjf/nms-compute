extern crate yaml_rust;

use std::fs::File;
use std::io::Read;
use yaml_rust::{YamlLoader,Yaml};
use yaml_rust::yaml::{Hash,Array};
use std::collections::HashMap;

const RESOURCES_FILE: &'static str = "Resources.yaml";
const RECIPES_FILE:   &'static str = "Recipes.yaml";

struct Resource {
    name: String,
    value: u32,
}

struct InputOutput<'a> {
    resource: &'a Resource,
    qty: u32,
}

// Inputs and outputs both use the same structure,
// but typing InputOutput every time is was once long and made
// it unclear which it is. So let's do some aliasing.

type Input<'a>  = InputOutput<'a>;
type Output<'a> = InputOutput<'a>;

struct Recipe<'a> {
    name: String,
    inputs: Vec<Input<'a>>,
    output: Output<'a>,
}

fn main() {

    let resources = read_resources();
    let recipes   = read_recipes(&resources);

    for recipe in recipes.iter() {
        let mut input_val  = 0;
        let output_val = recipe.output.resource.value * recipe.output.qty;

        let mut input_qty  = 0;

        println!("== {} ==", recipe.name);  // Pretty header

        // Print all the inputs. Also sum their values and total amount used.
        for input in recipe.inputs.iter() {
            let line_val = input.resource.value * input.qty;
            println!("Input:  {} × {} ({}u)", input.qty, input.resource.name, line_val);
            input_val += line_val;
            input_qty += input.qty;
        }

        println!("Output: {} × {} ({}u)\n", recipe.output.qty, recipe.output.resource.name, output_val);

        let profit = (output_val as f64 - input_val as f64) / input_qty as f64;
        println!("Profit per input = {}u\n\n", profit);
    }
}

// Reads our resources from the YAML configuration file.
fn read_resources() -> HashMap<String, Resource> {
    let mut map = HashMap::new();
    let resources = yaml_hash_from_file(RESOURCES_FILE);

    for (name, value) in resources.iter() {
        let name  = String::from(name.as_str().unwrap());
        println!("Resource: {}", name);
        let value = value.as_i64().unwrap() as u32;
        map.insert(name.clone(), Resource { name: name, value: value });
    }

    return map;
}

// Reads recipes from our recipes file and returns a vector of them.
fn read_recipes(resources: &HashMap<String, Resource>) -> Vec<Recipe> {
    let recipes = yaml_array_from_file(RECIPES_FILE);
    let mut result = Vec::new();

    for recipe in recipes.iter() {
        let recipe = recipe.as_hash().unwrap();
        let name   = String::from(recipe.get(&Yaml::from_str("name")).unwrap().clone().into_string().unwrap());

        // Build all our inputs
        let mut inputs = Vec::new();
        for input in recipe.get(&Yaml::from_str("inputs")).iter() {

            // XXX - This me getting a single key-value pair. Is there a better way?
            let (input, qty) = input.as_hash().unwrap().iter().last().unwrap();

            // Turn the key from our YAML file into an actual resource struct.
            let resource = resources.get(input.as_str().unwrap()).unwrap();
            
            // Add our input to our vector.
            inputs.push(Input { resource: resource, qty: qty.as_i64().unwrap() as u32 });
        }

        // There can only ever be one output, so we'll drill down to it directly.
        // XXX - OMFG, there's got to be a better way than this, please?
        let (output, qty) = recipe.get(&Yaml::from_str("output")).unwrap().as_hash().unwrap().iter().last().unwrap();
        let output = resources.get(output.as_str().unwrap()).unwrap();
        let output = Output { resource: &output, qty: qty.as_i64().unwrap() as u32 };

        result.push(Recipe { name: name, inputs: inputs, output: output });
    }

    return result;
}

// Reads the file specified and turns it into a Yaml::Hash
fn yaml_hash_from_file(filename: &str) -> Hash {
    return yaml_from_file(filename).into_hash().unwrap();
}

fn yaml_array_from_file(filename: &str) -> Array {
    return yaml_from_file(filename).into_vec().unwrap();
}

fn yaml_from_file(filename: &str) -> Yaml {
    let mut fh = File::open(filename).expect(&format!("Could not open {}", filename));

    let mut yaml = String::new();
    fh.read_to_string(&mut yaml).expect(&format!("Reading {} failed", filename));

    let result = YamlLoader::load_from_str(&yaml).unwrap()[0].clone();

    return result;
}
