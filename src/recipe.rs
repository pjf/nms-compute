extern crate csv;

use yaml_rust::Yaml;
use std::fs::File;
use csv::StringRecord;
use std::error::Error;

use resource::{Resource,ResourceMap};
use yaml_array_from_file;

const RECIPES_FILE:   &'static str = "Recipes.yaml";
const REFINING_CSV:   &'static str = "Refinery.csv";

pub struct InputOutput<'a> {
    pub resource: &'a Resource,
    pub qty: u32,
}

// Inputs and outputs both use the same structure,
// but typing InputOutput every time is was once long and made
// it unclear which it is. So let's do some aliasing.

type Input<'a>  = InputOutput<'a>;
type Output<'a> = InputOutput<'a>;

pub struct Recipe<'a> {
    pub name: String,
    pub inputs: Vec<Input<'a>>,
    pub output: Output<'a>,
}

// I'm not sure *why* `impl` needs a lifetime here, but it does. :/
impl<'a> Recipe<'a> {
    pub fn output_value(&self) -> u32 {
        self.output.resource.value * self.output.qty
    }

    pub fn input_value(&self) -> u32 {
        let mut total = 0;
        for input in &self.inputs {
            total += input.resource.value * input.qty;
        }
        return total;
    }

    pub fn input_qty(&self) -> u32 {
        let mut total = 0;
        for input in &self.inputs {
            total += input.qty;
        }
        return total;
    }

    pub fn profit(&self) -> f64 {
        self.output_value() as f64 - self.input_value() as f64
    }

    pub fn profit_ea(&self) -> f64 {
        self.profit() / self.input_qty() as f64
    }

}

// Reads recipes from our recipes file and returns a vector of them.
pub fn read_recipes(resources: &ResourceMap) -> Vec<Recipe> {
    let recipes = yaml_array_from_file(RECIPES_FILE);

    let mut result = Vec::new();

    for recipe in recipes {
        let recipe = recipe.as_hash().unwrap();
        let name   = String::from(recipe.get(&Yaml::from_str("name")).unwrap().clone().into_string().unwrap());

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

// Reads our refinery recipes, which are from
// https://docs.google.com/spreadsheets/d/1m3D-ElN7ek3Y0f-1XDt0IW2l6HxfXi5n5Yr7VLwLbg4/edit#gid=1526138107
pub fn read_refinery(resources: &ResourceMap) -> Vec<Recipe> {
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

        // Skip blank lines.
        if name.is_empty() {
            continue;
        }

        let recipe = read_refinery_record(&resources, &record);

        match recipe {
            Ok(r)  => recipes.push(r),
            Err(e) => println!("Skipping {}: {}", name, e)
        }

    }

    return recipes;
}

fn read_refinery_record<'a>(resources: &'a ResourceMap, record: &StringRecord) -> Result<Recipe<'a>, Box<Error>> {

    let output = read_refinery_ingredient(resources, &record[2], &record[3])?;

    let mut inputs = Vec::new();

    // First input should always be there
    inputs.push(
        read_refinery_ingredient(resources, &record[7], &record[8])?
    );

    // 2nd input may be empty
    if ! record[9].is_empty() {
        inputs.push(
            read_refinery_ingredient(resources, &record[9], &record[10])?
        )
    }

    // 3rd input may be empty
    if ! record[11].is_empty() {
        inputs.push(
            read_refinery_ingredient(resources, &record[11], &record[12])?
        )
    }

    return Ok(Recipe {
        name: record[6].to_string(),
        output: output,
        inputs: inputs
    });
}

fn read_refinery_ingredient<'a>(resources: &'a ResourceMap, resource: &str, qty: &str) -> Result<InputOutput<'a>, Box<Error>> {
    let resource = resources.get(resource).ok_or(format!("Resource '{}' lookup failed", resource))?;
    let qty      = qty.parse()?;

    return Ok(InputOutput {
        resource,
        qty
    });
}
