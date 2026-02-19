use dialoguer::{Input, theme::ColorfulTheme};
use rand::RngExt;

mod hamiltonian;
mod switch;
use switch::Switch;

fn main() {
    let (test_numbers, ports_number, load) = get_input();
    let mut rng = rand::rng();
    let mut my_switch = Switch::new(ports_number);

    for _ in 0..test_numbers {
        for input_port in 0..ports_number {
            let random_number = rng.random_range(0..100);
            if random_number < load {
                let output_port = rng.random_range(0..ports_number);
                my_switch.add_a_packet_in_queue(input_port, output_port);
            }
        }
    }
}

/// Get User Inputs (number of tests, number of ports, load percent)
fn get_input() -> (u32, usize, u8) {
    let test_numbers: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("1. test numbers: ")
        .interact_text()
        .unwrap();

    let ports_number: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("2. port numbers: ")
        .interact_text()
        .unwrap();

    let load: u8 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("3. load: [0-100]")
        .validate_with(|input: &u8| {
            if (0..=100).contains(input) {
                Ok(())
            } else {
                Err("load value should be between 0 and 100")
            }
        })
        .interact_text()
        .unwrap();

    (test_numbers, ports_number, load)
}
