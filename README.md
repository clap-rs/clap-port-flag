# clap-port-flag

[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Easily add a `--port` flag to CLIs using clap.

- [Documentation][8]
- [Crates.io][2]

## Usage

### Example: Base

With the following code in `src/main.rs`:

```rust,no_run
use clap::Parser;
use clap_port_flag::Port;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    port: Port,
}

fn main() {
    let args = Cli::parse();
    let _tcp_listener = args.port.bind().unwrap();
}
```

When you run the binary, it'll provide the following output:

```txt
my-cool-app 0.2.0
Alice Person <alice@person.com>
Application that does things over TCP.

USAGE:
    main [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --listen-fd <fd>         A previously opened network socket. [env: LISTEN_FD=]
    -a, --address <hostname>     The network address to listen to. [default: 127.0.0.1]
    -p, --port <port>            The network port to listen to. [env: PORT=]
```

### Example: Hyper

```rust,no_run
use clap_port_flag::Port;
use futures::prelude::*;
use hyper::service::service_fn;
use hyper::{Body, Response, Request};
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    port: Port,
}

async fn hello(_: Request<Body>) -> Result<Response<String>, std::convert::Infallible> {
    Ok(Response::new(String::from("Hello World!")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let listener = args.port.bind()?;
    let listener = tokio::net::TcpListener::from_std(listener)?;
    let addr = listener.local_addr()?;

    println!("Server listening on {}", addr);

    let (stream, _) = listener.accept().await?;
    if let Err(e) = hyper::server::conn::Http::new()
        .serve_connection(stream, service_fn(hello))
        .await
    {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
```

## Installation

```sh
$ cargo add clap-port-flag
```

## Further Reading

- [WhatWG URL spec](https://url.spec.whatwg.org/)
- [nodejs.org/api/url](https://nodejs.org/api/url.html)

## Acknowledgements

The original version of this crate was sketched out by
[@TeXitoi](https://github.com/TeXitoi) in
[rust-lang-nursery/cli-wg#37](https://github.com/rust-lang-nursery/cli-wg/issues/37).

## License

[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/clap-port-flag.svg?style=flat-square
[2]: https://crates.io/crates/clap-port-flag
[3]: https://img.shields.io/travis/rust-clique/clap-port-flag.svg?style=flat-square
[4]: https://travis-ci.org/rust-clique/clap-port-flag
[5]: https://img.shields.io/crates/d/clap-port-flag.svg?style=flat-square
[6]: https://crates.io/crates/clap-port-flag
[7]: https://docs.rs/clap-port-flag/badge.svg
[8]: https://docs.rs/clap-port-flag
