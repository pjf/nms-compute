#[macro_use] extern crate prettytable;
extern crate yaml_rust;
extern crate csv;

mod resource;
mod recipe;

use resource::ResourceMap;
use recipe::{read_recipes, read_refinery};

use std::fs::File;
use std::io::Read;

use yaml_rust::{YamlLoader,Yaml};
use yaml_rust::yaml::{Hash,Array};

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;

// This struct used to have a lot more fields. :)
struct FormattingWidth {
    qty: usize,
}

fn main() {
    let resources   = ResourceMap::load();
    let mut recipes = read_recipes(&resources);
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

        let mut row = Vec::<Cell>::new();

        row.push(Cell::new(&recipe.name));

        let mut inputs = 0;

        // Print all the inputs. Also sum their values and total amount used.
        for input in recipe.inputs.iter() {

            row.push(
                Cell::new(
                    &format!("{qty:>qty_width$} × {input}",
                        qty=input.qty, input=input.resource.name,
                        qty_width=width.qty
                    )
                )
            );

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

        // Profits are styled to be right-aligned

        row.push(Cell::new(&format!("{:.*}",2,recipe.profit_ea())).style_spec("r"));
        row.push(Cell::new(&format!("{}",recipe.profit())).style_spec("r"));

        table.add_row(Row::new(row));
    }

    table.printstd();
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
