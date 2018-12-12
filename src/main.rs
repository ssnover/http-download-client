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

    let url_split = url.split("/");
    let url_pieces: Vec<&str> = url_split.collect();
    let hostname = url_pieces[0].clone();

    let mut resource_string: String = "".to_owned();
    let mut i = 0;
    for s in url_pieces {
        if i != 0 {
            resource_string.push_str("/");
            resource_string.push_str(s);
        }
        i += 1;
    }

    let mut request: String = "GET ".to_owned();
    request.push_str(&resource_string);
    request.push_str(" HTTP/1.1\r\n");
    request.push_str("Host: ");
    request.push_str(&hostname);
    request.push_str("\r\n\r\n");

    println!("Port: {}", port);
    println!("URL: {}", url);
    println!("Hostname: {}", hostname);
    println!("Resource: {}", resource_string);
    println!("HTTP Request: \n{}", request);
}
