struct Resource {
    name: &'static str,
    value: u32,
}

struct InputOutput<'a> {
    resource: &'a Resource,
    qty: u32,
}

// Inputs and outputs both use the same structure,
// but typing InputOutput every time is both long and makes
// it unclear which it is. So let's do some aliasing.

type Input<'a>  = InputOutput<'a>;
type Output<'a> = InputOutput<'a>;

struct Recipe<'a> {
    name: &'static str,
    inputs: Vec<Input<'a>>,
    output: Output<'a>,
}

fn main() {
    let fe               = Resource { name: "Ferrite Dust",       value:     14 };
    let fep              = Resource { name: "Pure Ferrite",       value:     28 };
    let fepp             = Resource { name: "Magnetised Ferrite", value:     82 };
    let carbon           = Resource { name: "Carbon",             value:     12 };
    let condensed_carbon = Resource { name: "Condensed Carbon",   value:     24 };
    let tritium          = Resource { name: "Tritium",            value:      6 };
    let dihydrogen       = Resource { name: "Di-hydrogen",        value:     34 };
    let frigate_fuel_50  = Resource { name: "Frigate Fuel (50t)", value: 50_000 };
    let copper           = Resource { name: "Copper",             value:    121 };
    let emeril           = Resource { name: "Emeril",             value:    348 };
    let chromatic_metal  = Resource { name: "Chromatic Metal",    value:    245 };
    let gamma_root       = Resource { name: "Gamma Root",         value:     16 };
    let coprite          = Resource { name: "Coprite",            value:      0 }; // TODO: Find value
    let cactus           = Resource { name: "Cactus Flesh",       value:     28 };
    let star_bulb        = Resource { name: "Star Bulbs",         value:      0 }; // TODO
    let frost_crystal    = Resource { name: "Frost Crystal",      value:     12 };
    let glass            = Resource { name: "Glass",              value:  18_000 };
    let living_glass     = Resource { name: "Living Glass",       value: 696_000 };
    let poly_fiber       = Resource { name: "Poly Fiber",         value: 200_000 };
    let lubricant        = Resource { name: "Lubricant",          value: 160_000 };
    let heat_capacitor   = Resource { name: "Heat Capacitor",     value: 240_000 };
    let circuit_board    = Resource { name: "Circuit Board",      value: 1_196_250 };

    let recipes = vec![
        Recipe {
            name: "Purify Ferrite",
            inputs: vec![ Input { resource: &fe, qty: 1 } ],
            output: Output { resource: &fep, qty: 1 }
        },
        Recipe {
            name: "Condense carbon",
            inputs: vec![ Input { resource: &carbon, qty: 2 } ],
            output: Output { resource: &condensed_carbon, qty: 1 }
        },
        Recipe {
            name: "Rocket Fuel",
            inputs: vec![
                Input { resource: &tritium,    qty: 50 },
                Input { resource: &dihydrogen, qty: 50 },
            ],
            output: Output { resource: &frigate_fuel_50, qty: 1 }
        },
        Recipe {
            name: "Copper -> Chromatic Metal",
            inputs: vec![ Input { resource: &copper, qty: 2 } ],
            output: Output { resource: &chromatic_metal, qty: 1 }
        },
        Recipe {
            name: "Emeril -> Chromatic Metal",
            inputs: vec![ Input { resource: &emeril, qty: 2 } ],
            output: Output { resource: &chromatic_metal, qty: 3 }
        },
        Recipe {
            name: "Poly Fiber",
            inputs: vec![
                Input { resource: &star_bulb, qty: 200 },
                Input { resource: &cactus, qty: 100 },
            ],
            output: Output { resource: &poly_fiber, qty: 1 }
        },
        Recipe {
            name: "Glass",
            inputs: vec![ Input { resource: &frost_crystal, qty: 50 } ],
            output: Output { resource: &glass, qty: 1 }
        },
        Recipe {
            name: "Living Glass",
            inputs: vec![
                Input { resource: &glass, qty: 5 },
                Input { resource: &lubricant, qty: 1 },
            ],
            output: Output { resource: &living_glass, qty: 1 }
        },
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
