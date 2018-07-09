#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate structopt;

use std::io;
use std::net::TcpListener;
use std::os::raw::c_int;
use std::os::unix::io::FromRawFd;

/// Easily add a `--port` flag to Structopt.
///
/// ## Usage
/// ```rust
/// extern crate clap_port_flag;
/// #[macro_use] extern crate structopt;
///
/// use structopt::StructOpt;
/// use clap_port_flag::Port;
///
/// #[derive(Debug, StructOpt)]
/// struct Cli {
///   #[structopt(flatten)]
///   port: Port,
/// }
/// #
/// # fn main() {}
/// ```
#[derive(StructOpt, Debug)]
pub struct Port {
  #[structopt(short = "H", long = "hostname", default_value = "127.0.0.1")]
  hostname: String,
  #[structopt(short = "p", long = "port", env = "PORT", group = "bind")]
  port: Option<u16>,
  #[structopt(long = "file-descriptor", env = "LISTEN_FD", group = "bind")]
  fd: Option<c_int>,
}

/// Create a TCP socket.
///
/// ## Panics
/// If a file descriptor Was passed directly, we call the unsafe
/// `TcpListener::from_raw_fd()` method, which may panic if a non-existent file
/// descriptor was passed.
impl Port {
  /// Create a TCP socket from the passed in port or file descriptor.
  pub fn bind(&self) -> std::io::Result<TcpListener> {
    match self {
      Self { fd: Some(fd), .. } => unsafe { Ok(TcpListener::from_raw_fd(*fd)) },
      Self {
        port: Some(port), ..
      } => TcpListener::bind((self.hostname.as_str(), *port)),
      _ => Err(io::Error::new(io::ErrorKind::Other, "No port supplied.")),
    }
  }
}
