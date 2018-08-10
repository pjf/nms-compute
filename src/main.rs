extern crate yaml_rust;

use std::fs::File;
use std::io::Read;
use yaml_rust::{YamlLoader,Yaml};
use yaml_rust::yaml::{Hash,Array};
use std::collections::HashMap;

const RESOURCES_FILE: &'static str = "Resources.yaml";
const RECIPES_FILE:   &'static str = "Recipes.yaml";

struct FormattingWidth {
    name: usize,
    input: usize,
    output: usize,
    qty: usize,
    value: usize,
    profit: usize,
}

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

    let width = FormattingWidth {
        // Some table elements have fixed widths
        qty:    3,
        value:  7,
        profit: 7,
        // And some we'll dynamically generate some.
        // I honestly thought these would be more succinct when I started.
        name:   max_width(recipes.iter().map(|x| &x.name)),
        output: max_width(recipes.iter().map(|x| &x.output.resource.name)),
        input:  max_width(recipes.iter().flat_map(|x| &x.inputs).collect::<Vec<&InputOutput>>().iter().map(|x| &x.resource.name)),
    };

    print_header(&width);

    // Walk through all our recipes and print them!
    for recipe in recipes.iter() {

        // Input_val tracks the cost of the entire recipe
        let mut input_val  = 0;
        let output_val = recipe.output.resource.value * recipe.output.qty;

        let mut input_qty  = 0;

        print!("{name:<width$} | ", name=recipe.name, width=width.name);

        // Print all the inputs. Also sum their values and total amount used.
        for input in recipe.inputs.iter() {

            // line_val is how much individual input costs.
            let line_val = input.resource.value * input.qty;
            print!(
                "{qty:>qty_width$} × {input:<input_width$} ({value:>value_width$}u)",
                qty=input.qty, input=input.resource.name, value=line_val,
                qty_width=width.qty, input_width=width.input, value_width=width.value
            );
            input_val += line_val;
            input_qty += input.qty;
        }

        let profit = (output_val as f64 - input_val as f64) / input_qty as f64;
        println!(
            " |{qty:>qty_width$} × {output:<output_width$} ({value:>value_width$}u) | {profit:>profit_width$}u",
            qty=recipe.output.qty, output=recipe.output.resource.name, value=output_val, profit=profit,
            qty_width=width.qty, output_width=width.output, value_width=width.value, profit_width=width.profit
        );
    }
}

fn print_header(width: &FormattingWidth) {

    let input_width  = width.input  + width.qty + width.value + 7; // Magic 7 is our padding and '×' signs.
    let output_width = width.output + width.qty + width.value + 6;

    let header = format!(
        "{reaction:^reaction_width$} | {input:^input_width$} | {output:^output_width$} | {profit:^profit_width$}",
        reaction="Reaction", input="Input", output="Output", profit="Profit",
        reaction_width=width.name, input_width=input_width, output_width=output_width, profit_width=width.profit+1
    );

    // This takes advantage that we can fill formats with a custom character,
    // so we format an empty string into a field and fill it wish dashes. :)
    let dashes = format!("{:-^len$}", "", len=header.len());

    println!("{}\n{}",header,dashes);
}

fn max_width<'a>(strings: impl Iterator<Item=&'a String>) -> usize {
    let mut max_width = 0;
    for string in strings {
        if string.len() > max_width {
            max_width = string.len();
        }
    }
    return max_width;
}

// Reads our resources from the YAML configuration file.
fn read_resources() -> HashMap<String, Resource> {
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
                name: name,
                value: value
            }
        );
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

        println!("{}",name);

        // Build all our inputs
        let mut inputs = Vec::new();
        for input in recipe.get(&Yaml::from_str("inputs")).iter() {

            // XXX - This me getting a single key-value pair. Is there a better way?
            let (input, qty) = input.as_hash().unwrap().iter().last().unwrap();

            // println!("- {}", input.as_str().unwrap());

            // Turn the key from our YAML file into an actual resource struct.
            let resource = resources.get(input.as_str().unwrap()).unwrap();
            
            // Add our input to our vector.
            inputs.push(Input { resource: resource, qty: qty.as_i64().unwrap() as u32 });
        }

        // There can only ever be one output, so we'll drill down to it directly.
        // XXX - OMFG, there's got to be a better way than this, please?
        let (output, qty) = recipe.get(&Yaml::from_str("output")).unwrap().as_hash().unwrap().iter().last().unwrap();
        // println!("+ {}", output.as_str().unwrap());
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
