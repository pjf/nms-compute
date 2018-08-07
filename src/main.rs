struct Resource {
    name: &'static str,
    value: u32,
}

struct InputOutput {
    resource: Resource,
    qty: u32,
}

// Inputs and outputs both use the same structure,
// but typing InputOutput every time is both long and makes
// it unclear which it is. So let's do some aliasing.

type Input  = InputOutput;
type Output = InputOutput;

struct Recipe {
    name: &'static str,
    inputs: Vec<Input>,
    output: Output,
}

fn main() {
    let carbon           = Resource { name: "Carbon",             value:     12 };
    let condensed_carbon = Resource { name: "Condensed Carbon",   value:     24 };
    let tritium          = Resource { name: "Tritium",            value:      6 };
    let dihydrogen       = Resource { name: "Di-hydrogen",        value:     34 };
    let frigate_fuel_50  = Resource { name: "Frigate Fuel (50t)", value: 50_000 };
    let copper           = Resource { name: "Copper",             value:    121 };
    let chromatic_metal  = Resource { name: "Chromatic Metal",    value:    245 };

    let recipes = vec![
        Recipe {
            name: "Condense carbon",
            inputs: vec![ Input { resource: carbon, qty: 2 } ],
            output: Output { resource: condensed_carbon, qty: 1 }
        },
        Recipe {
            name: "Rocket Fuel",
            inputs: vec![
                Input { resource: tritium,    qty: 50 },
                Input { resource: dihydrogen, qty: 50 },
            ],
            output: Output { resource: frigate_fuel_50, qty: 1 }
        },
        Recipe {
            name: "Copper -> Chromatic Metal",
            inputs: vec![ Input { resource: copper, qty: 2 } ],
            output: Output { resource: chromatic_metal, qty: 1 }
        }
    ];

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
