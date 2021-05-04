mod name_generator;
#[macro_use]
mod number_generator;
use crate::name_generator::name::generate_name;
use crate::name_generator::{
    configuration::{self, config_model::GeneratorConfiguration},
    loader::loader,
};

fn main() {
    let config: GeneratorConfiguration = configuration::reader::read("./config.json").unwrap();
    let firstname = loader(&config, "firstname");
    let lastname = loader(&config, "lastname");

    for _i in 0..10 {
        println!("{} {}", generate_name(&firstname), generate_name(&lastname));
    }
}
