use std::{io::Write, net::TcpStream};

use clap::Parser;

#[derive(Parser)]
#[clap(about = "simple YukiDB client")]
struct Opts {
    #[clap(short, long)]
    host: String,
    #[clap(short, long)]
    port: u16,
}

fn main() {
    let opts = Opts::parse();

    let serveraddr = format!("{}:{}", opts.host, opts.port);
    let mut conn = TcpStream::connect(serveraddr).expect("Failed to connect to server");

    conn.write(b"Hello, world!").unwrap();
}
