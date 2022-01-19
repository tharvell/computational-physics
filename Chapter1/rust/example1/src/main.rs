use clap::Parser;
use plotters::prelude::*;
mod oiler;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Decay constant
    #[clap(short, long)]
    tau: f64,

    /// Starting amount
    #[clap(short, long)]
    nu_0: f64,
    
    /// Time steps 
    #[clap(long)]
    timesteps: u64,

    /// Step Size
    #[clap(short, long)]
    stepsize: f64,  
}

fn main() {
    let args = Args::parse();
    
    let c = oiler::euler_nu_decay(args.tau, args.nu_0, args.timesteps, args.stepsize);
    
    println!("{}", c);

    let root = BitMapBackend::new("plotters-doc-data/5.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);
}

