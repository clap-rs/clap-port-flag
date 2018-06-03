extern crate clap_port_flag;
#[macro_use]
extern crate structopt;

use clap_port_flag::Port;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
  #[structopt(flatten)]
  port: Port,
}

fn main() {
  let args = Cli::from_args();
  let tcp_listener = args.port.bind().unwrap();
  println!("{:?}", tcp_listener);
}
