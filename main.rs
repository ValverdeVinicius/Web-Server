use std::io::{BufRead, Write};
use std::fs::read;

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();
    
    for mut stream in listener.incoming().flatten(){
			
				let mut rdr = std::io::BufReader::new(&mut stream);
				let mut l = String::new();
				rdr.read_line(&mut l).unwrap();
				
				match l.trim().split(' ').collect::<Vec<_>>().as_slice(){
					["GET", resource, "HTTP/1.1"] => {
					loop{
						let mut l = String::new();
						rdr.read_line(&mut l).unwrap();
						if l.trim().is_empty() { break; }
				}
				let mut p = std::path::PathBuf::new();
				p.push("htmldoc");
				p.push(resource.trim_start_matches("/"));
				if resource.ends_with('/'){ 
					p.push("index.html"); 
				}
				if let Ok(content) = read(&p) {
					stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
					stream.write_all(&content).unwrap();
				} 
				else {
					stream.write_all(b"HTTP/1.1 404 NOT FOUND\r\n\r\n").unwrap();
				}
			}
			_ => todo!()
		}
	}
}
