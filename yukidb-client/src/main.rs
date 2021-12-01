use std::{
    io::{stdin, BufRead, BufReader, BufWriter, Write, stdout},
    net::TcpStream,
};

use clap::Parser;

#[derive(Parser)]
#[clap(about = "simple YukiDB client")]
struct Opts {
    #[clap(short, long, default_value = "localhost")]
    host: String,
    #[clap(short, long)]
    port: u16,
}

fn handle_connection(mut read_stream: BufReader<TcpStream>, mut write_stream: BufWriter<TcpStream>) {
    let mut stdin = BufReader::new(stdin()).lines();

    print!("> ");
    stdout().flush().unwrap();
    while let Ok(command) = stdin.next().unwrap() {
        write_stream.write_all((command + "\n").as_bytes()).unwrap();
        write_stream.flush().unwrap();

        let mut result = String::new();
        read_stream.read_line(&mut result).unwrap();
        print!("{}> ", result);
        stdout().flush().unwrap();
    }
}

fn main() {
    let opts = Opts::parse();

    let serveraddr = format!("{}:{}", opts.host, opts.port);
    let conn = TcpStream::connect(serveraddr).expect("Failed to connect to server");

    let mut read_stream = BufReader::new(conn.try_clone().unwrap());

    let mut response = String::new();
    read_stream.read_line(&mut response).unwrap();

    print!("{}", response);
    stdout().flush().unwrap();

    if response == "The connections are full.\n" {
        return;
    }

    handle_connection(read_stream, BufWriter::new(conn));
}
