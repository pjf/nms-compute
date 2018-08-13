#[macro_use] extern crate prettytable;
extern crate yaml_rust;
extern crate csv;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::error::Error;

use yaml_rust::{YamlLoader,Yaml};
use yaml_rust::yaml::{Hash,Array};

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;

use csv::StringRecord;

const RESOURCES_FILE: &'static str = "Resources.yaml";
const RECIPES_FILE:   &'static str = "Recipes.yaml";
const REFINING_CSV:   &'static str = "Refinery.csv";

// This struct used to have a lot more fields. :)
struct FormattingWidth {
    qty: usize,
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
    let mut recipes   = read_recipes(&resources);
    recipes.extend(read_refinery(&resources));

    // This struct used to have a lot more fields. :)
    let width = FormattingWidth {
        qty:    3,
    };

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Reaction", "Input", "Input", "Input", "Output", "Profit/input", "Profit Total"]);

    // Walk through all our recipes and print them!
    for recipe in recipes.iter() {

        // Input_val tracks the cost of the entire recipe
        let mut input_val  = 0;
        let output_val = recipe.output.resource.value * recipe.output.qty;

        let mut input_qty  = 0;

        let mut row = Vec::<Cell>::new();

        row.push(Cell::new(&recipe.name));

        let mut inputs = 0;

        // Print all the inputs. Also sum their values and total amount used.
        for input in recipe.inputs.iter() {

            // line_val is how much individual input costs.
            let line_val = input.resource.value * input.qty;

            row.push(
                Cell::new(
                    &format!("{qty:>qty_width$} × {input}",
                        qty=input.qty, input=input.resource.name,
                        qty_width=width.qty
                    )
                )
            );

            input_val += line_val;
            input_qty += input.qty;
            inputs += 1;
        }

        // Fill input cells for recipes with fewer than three ingredients
        while inputs < 3 {
            row.push(Cell::new(""));
            inputs += 1;
        }

        // Output product
        row.push(
            Cell::new(
                &format!("{qty:>qty_width$} × {output}",
                    qty=recipe.output.qty, output=recipe.output.resource.name,
                    qty_width=width.qty
                )
            )
        );

        let profit: f64  = output_val as f64 - input_val as f64;
        let profit_ea    = profit / input_qty as f64;

        // Profits are styled to be right-aligned

        row.push(Cell::new(&format!("{:.*}",2,profit_ea)).style_spec("r"));
        row.push(Cell::new(&format!("{}",profit)).style_spec("r"));

        table.add_row(Row::new(row));
    }

    table.printstd();
}

// Reads our refinery recipes, which are from
// https://docs.google.com/spreadsheets/d/1m3D-ElN7ek3Y0f-1XDt0IW2l6HxfXi5n5Yr7VLwLbg4/edit#gid=1526138107
fn read_refinery(resources: &HashMap<String, Resource>) -> Vec<Recipe> {
    let fh = File::open(REFINING_CSV).expect(&format!("Could not open {}", REFINING_CSV));

    let mut csv = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(fh)
    ;

    let mut rows = csv.records();

    // First three lines of the refinery file are headers and notes, skip them.
    for _ in 0..3 {
        rows.next();
    }

    let mut recipes = Vec::new();

    for record in rows {
        let record = record.unwrap();

        let name = record[6].to_string();

        let recipe = read_refinery_record(&resources, &record);

        match recipe {
            Ok(r)  => recipes.push(r),
            Err(e) => println!("Skipping {}: {}", name, e)
        }

    }

    return recipes;
}

fn read_refinery_record<'a>(resources: &'a HashMap<String, Resource>, record: &StringRecord) -> Result<Recipe<'a>, Box<Error>> {

    let output = Output {
        resource: resources.get(&record[2]).ok_or("Output resource lookup failed")?,
        qty:      record[3].parse()?
    };

    let mut inputs = Vec::new();

    // First input should always be there
    inputs.push(
        Input {
            resource: resources.get(&record[7]).ok_or("Input resource lookup failed")?,
            qty:      record[8].parse()?
        }
    );

    // 2nd input may be empty
    if ! record[9].is_empty() {
        inputs.push(
            Input {
                resource: resources.get(&record[9]).ok_or("Input resource lookup failed")?,
                qty:      record[10].parse()?
            }
        )
    }

    // 3rd input may be empty
    // XXX - This is copy/paste from above with different magic numbers. :(
    if ! record[11].is_empty() {
        inputs.push(
            Input {
                resource: resources.get(&record[11]).ok_or("Input resource lookup failed")?,
                qty:      record[12].parse()?
            }
        )
    }

    return Ok(Recipe {
        name: record[6].to_string(),
        output: output,
        inputs: inputs
    });
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

    for recipe in recipes {
        let recipe = recipe.as_hash().unwrap();
        let name   = String::from(recipe.get(&Yaml::from_str("name")).unwrap().clone().into_string().unwrap());

        // println!("{}",name);

        // Build all our inputs
        let mut inputs = Vec::new();
        for (input, qty) in recipe.get(&Yaml::from_str("inputs")).unwrap().as_hash().unwrap().iter() {

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
