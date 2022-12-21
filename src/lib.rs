use std::process;

#[derive(Debug)]
pub enum ArgParseErr {
    NotEnoughArgs,
    InvalidBalance(&'static str),
    InvalidRisk(&'static str),
    InvalidStopLoss(&'static str),
}

pub struct Args {
    pub balance: f32,
    pub risk: f32,
    pub stop_loss: u32,
}
impl Args {
    pub fn parse(mut args: Vec<String>) -> Result<Args, ArgParseErr> {
        if args.len() != 4 {
            return Err(ArgParseErr::NotEnoughArgs);
        }

        let balance = args[1].parse().or(Err(ArgParseErr::InvalidBalance(
            "Please provide a valid balance, e.g. 10000",
        )))?;

        let risk: f32;
        if args[2].contains('%') {
            args[2].pop();
            risk = args[2].parse().or(Err(ArgParseErr::InvalidBalance(
                "Please provide a valid risk, e.g. 2%",
            )))?;
            if risk > 100.0 {
                return Err(ArgParseErr::InvalidRisk("Risk can't be more than 100%"));
            }
        } else {
            eprintln!("% sign required");
            process::exit(1);
        }

        let stop_loss: u32 = args[3].parse().or(Err(ArgParseErr::InvalidStopLoss(
            "Please provide a valid stop loss, e.g. 150(points)",
        )))?;

        Ok(Args {
            balance,
            risk,
            stop_loss,
        })
    }
}

pub fn calculate_lot_size(balance: f32, risk: f32, stop_loss: u32) -> f32 {
    let result = (balance * risk / 100.0) / stop_loss as f32;
    let result = (result * 100.0).floor() / 100.0;
    result
}
