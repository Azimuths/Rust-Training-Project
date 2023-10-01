use std::io;
use rand::Rng;

struct Market {
    roll: i32,
    modifier: i32,
    multiplier: i32,
    values: Vec<i32>,
    up: bool,
    set_count: i32,
    base_value: i32,
}

impl Market {
    fn new() -> Self {
        Market {
            roll: rand::thread_rng().gen_range(1..=20),
            modifier: 0,
            multiplier: 1,
            values: Vec::new(),
            up: false,
            set_count: 0,
            base_value: 50,
        }
    }

    fn get_modifier(&mut self) {
        // ... (same as before)
    }

    fn get_multiplier(&mut self) {
        // ... (same as before)
    }

    fn update_base_value(&mut self) {
        if self.set_count >= 3 {
            self.set_count = 0;
            self.base_value += 10;
        }
    }

    fn calculate(&mut self) {
        loop {
            self.get_modifier();
            if self.modifier != 0 {
                self.values.push(self.modifier);
            }
            self.get_multiplier();

            println!("You rolled a {}", self.roll);
            let gain_loss = self.modifier * self.multiplier;
            println!(
                "That resulted in a gain or loss of {} electrum, multiplier being {}",
                gain_loss, self.multiplier
            );
            self.update_base_value();
            println!("Base value: {}", self.base_value);

            let mut input = String::new();
            println!("What do we do next? (Type 'exit' to quit)");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            if input == "exit" {
                break;
            }
            self.roll = rand::thread_rng().gen_range(1..=20);
        }
    }
}

fn main() {
    println!("Welcome to the electrum calculator!");
    let mut market = Market::new();
    market.calculate();
}
