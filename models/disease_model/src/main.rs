mod people;
mod transmission_manager;

use ixa::Context;

static POPULATION: u64 = 1000;
static FORCE_OF_INFECTION: f64 = 0.1;
static MAX_TIME: f64 = 300.0;

fn main() {
    let mut context = Context::new();
    people::init(&mut context);
    context.execute();

    println!("Simulation finished executing");
}
