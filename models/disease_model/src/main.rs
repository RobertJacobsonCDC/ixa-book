mod people;

use ixa::Context;

static POPULATION: u64 = 1000;

fn main() {
    let mut context = Context::new();
    people::init(&mut context);
    context.execute();

    println!("Simulation finished executing");
}
