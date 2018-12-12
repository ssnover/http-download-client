use std::env;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("", "port", "Set the port to read out on", "NUMBER");
    opts.optopt("", "url", "The URL containing the resource to download", "URL");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let port: u32 = match matches.opt_str("port") {
        Some(p) => { p.parse().expect("Invalid port number provided.") }
        None => { 80 }
    };
    let url = match matches.opt_str("url") {
        Some(u) => { u }
        None => { print_usage(&program, opts); return; }
    };

    println!("Port: {}", port);
    println!("URL: {}", url);
}
