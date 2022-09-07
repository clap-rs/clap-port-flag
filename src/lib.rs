#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(test, deny(warnings))]

use std::io;
use std::net::TcpListener;
use std::os::raw::c_int;
use std::os::unix::io::FromRawFd;

/// Easily add a `--port` flag to clap.
///
/// ## Usage
/// ```rust
/// #[derive(Debug, clap::Parser)]
/// struct Cli {
///   #[clap(flatten)]
///   port: clap_port_flag::Port,
/// }
/// #
/// # fn main() {}
/// ```
#[derive(clap::Args, Debug)]
pub struct Port {
    /// The network address to listen to.
    #[clap(short = 'a', long = "address", default_value = "127.0.0.1")]
    address: String,
    /// The network port to listen to.
    #[clap(short = 'p', long = "port", env = "PORT", group = "bind")]
    port: Option<u16>,
    /// A previously opened network socket.
    #[clap(long = "listen-fd", env = "LISTEN_FD", group = "bind")]
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
    pub fn bind(&self) -> std::io::Result<TcpListener> {
        match self {
            Self { fd: Some(fd), .. } => unsafe { Ok(TcpListener::from_raw_fd(*fd)) },
            Self {
                port: Some(port), ..
            } => TcpListener::bind((self.address.as_str(), *port)),
            _ => Err(io::Error::new(io::ErrorKind::Other, "No port supplied.")),
        }
    }

    /// Create a TCP socket by calling to `.bind()`. If it fails, create a socket
    /// on `port`.
    ///
    /// Useful to create a default socket to listen to if none was passed.
    pub fn bind_or(&self, port: u16) -> std::io::Result<TcpListener> {
        self.bind()
            .or_else(|_| TcpListener::bind((self.address.as_str(), port)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use clap::Parser;

    #[derive(Debug, Parser)]
    struct Cli {
        #[clap(flatten)]
        port: Port,
    }

    #[test]
    fn test_cli() {
        let args = Cli::try_parse_from(&["test", "--address", "1.2.3.4", "--port", "1234"]);
        assert!(args.is_ok(), "Not ok: {:?}", args.unwrap_err());
        let args = args.unwrap();
        assert_eq!(args.port.address, "1.2.3.4");
        assert_eq!(args.port.port, Some(1234));
    }
}
