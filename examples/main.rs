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

fn main() -> Result<(), std::io::Error> {
  let args = Cli::from_args();
  let tcp_listener = args.port.tcp_bind_or(8080)?;
  println!("TCP bind(or): {:?}", tcp_listener);

  let udp_listener = args.port.udp_bind_or(4242)?;
  println!("UDP bind(or): {:?}", udp_listener);
  Ok(())
}
