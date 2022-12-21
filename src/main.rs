use rman::{ArgParseErr, Args};
use std::{env, process};

fn main() {
    let args = match Args::parse(env::args().collect()) {
        Ok(args) => args,
        Err(err) => match err {
            ArgParseErr::NotEnoughArgs => {
                eprintln!("Not enough arguments");
                process::exit(1);
            }
            ArgParseErr::InvalidBalance(msg) => {
                eprintln!("{msg}");
                process::exit(1);
            }
            ArgParseErr::InvalidRisk(msg) => {
                eprintln!("{msg}");
                process::exit(1);
            }
            ArgParseErr::InvalidStopLoss(msg) => {
                eprintln!("{msg}");
                process::exit(1);
            }
        },
    };

    let lot_size = rman::calculate_lot_size(args.balance, args.risk, args.stop_loss);

    println!("balance: {}", args.balance);
    println!("risk: {}%", args.risk);
    println!("stop loss: {}", args.stop_loss);
    println!("_______________________________");
    println!("lot size for this trade: {lot_size}");
}
