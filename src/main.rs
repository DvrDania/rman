use std::{env, process};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }

    // parse balance
    let balance: u32 = args[1]
        .parse()
        .expect("Please provide a valid number for balance");

    // parse risk
    let risk: f32;
    if args[2].contains('%') {
        args[2].pop();
        risk = args[2]
            .parse()
            .expect("Please provide a valid number for risk");
    } else {
        eprintln!("% sign required");
        process::exit(1);
    }

    // parse stop loss
    let stop_loss: u32 = args[3]
        .parse()
        .expect("Please provide a valid number for stop loss");

    println!("balance: {}", balance);
    println!("risk: {}%", risk);
    println!("stop loss: {}", stop_loss);
}
