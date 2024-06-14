use std::net::Ipv4Addr;
use clap::Parser;

#[derive(Parser)]
struct Args {
    pub beginning: Ipv4Addr,
    pub end: Ipv4Addr,
}

fn main() {

    let args = Args::parse();

    let beginning_raw: u32 = args.beginning.into();
    let end_raw: u32 = args.end.into();

    if !(beginning_raw <= end_raw) {
        eprintln!("Starting IP is not less than ending IP.");
        return;
    }

    let distance = end_raw - beginning_raw;

    let adjust = rand::random::<u32>() % distance;

    let random_ip: Ipv4Addr = (beginning_raw + adjust).into();

    println!("{}", random_ip)
}
