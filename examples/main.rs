extern crate clap_port_flag;

use clap_port_flag::Port;

use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    port: Port,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    let tcp_listener = args.port.bind_or(8080)?;
    println!("{:?}", tcp_listener);
    Ok(())
}
