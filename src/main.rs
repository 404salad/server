use std::io::{BufRead, Write};

fn main() {
    let listener = std::net::TcpListener::bind("localhost:9999").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut reader = std::io::BufReader::new(&mut stream);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        match line.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    if line.trim().is_empty() {
                        break;
                    }
                    print!("{line}");
                }
                let mut path = std::path::PathBuf::new();
                path.push("www");
                path.push(resource.trim_start_matches("/"));
                if resource.ends_with('/') {
                    path.push("index.html");
                }
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                stream.write_all(&std::fs::read(path).unwrap()).unwrap();
            }
            _ => todo!(),
        }
    }
}
