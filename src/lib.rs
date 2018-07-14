#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate structopt;

use std::io;
use std::net::{TcpListener, UdpSocket};
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
  /// The network address to listen to.
  #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
  address: String,
  /// The network port to listen to.
  #[structopt(short = "p", long = "port", env = "PORT", group = "bind")]
  port: Option<u16>,
  /// A previously opened network socket.
  #[structopt(long = "listen-fd", env = "LISTEN_FD", group = "bind")]
  fd: Option<c_int>,
}

/// Create a TCP socket.
///
/// ## Panics
/// If a file descriptor was passed directly, we call the unsafe
/// `TcpListener::from_raw_fd()` method, which may panic if a non-existent file
/// descriptor was passed.
impl Port {
  /// Create a TCP socket from the passed in port or file descriptor.
  pub fn tcp_bind(&self) -> std::io::Result<TcpListener> {
    match self {
      Self { fd: Some(fd), .. } => unsafe { Ok(TcpListener::from_raw_fd(*fd)) },
      Self {
        port: Some(port), ..
      } => TcpListener::bind((self.address.as_str(), *port)),
      _ => Err(io::Error::new(io::ErrorKind::Other, "No port supplied.")),
    }
  }

  /// Create a TCP socket by calling to `.tcp_bind()`. If it fails, create a socket
  /// on `port`.
  ///
  /// Useful to create a default socket to listen to if none was passed.
  pub fn tcp_bind_or(&self, port: u16) -> std::io::Result<TcpListener> {
    self
      .tcp_bind()
      .or_else(|_| TcpListener::bind((self.address.as_str(), port)))
  }

/// Create a UDP socket.
///
/// ## Panics
/// If a file descriptor was passed directly, we call the unsafe
/// `UdpSocket::from_raw_fd()` method, which may panic if a non-existent file
/// descriptor was passed.
  pub fn udp_bind(&self) -> std::io::Result<UdpSocket> {
    match self {
      Self { fd: Some(fd), .. } => unsafe { Ok(UdpSocket::from_raw_fd(*fd)) },
      Self {
        port: Some(port), ..
      } => UdpSocket::bind((self.address.as_str(), *port)),
      _ => Err(io::Error::new(io::ErrorKind::Other, "No port supplied.")),
    }
  }

  /// Create a UDP socket by calling to `.udp_bind()`. If it fails, create a socket
  /// on `port`.
  ///
  /// Useful to create a default socket to listen to if none was passed.
  pub fn udp_bind_or(&self, port: u16) -> std::io::Result<UdpSocket> {
    self
      .udp_bind()
      .or_else(|_| UdpSocket::bind((self.address.as_str(), port)))
  } 
}
