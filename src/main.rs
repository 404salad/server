use std::{io::{BufRead, Write}};

// goal is to not use functions till needed to keep the flow as linear as possible
fn main() {
    let listener = std::net::TcpListener::bind("localhost:9999")
        .expect("could not bind to localhost 9999"); // todo: try alternate ports
    let www_path = std::path::PathBuf::from("www").canonicalize().expect("have a www/ directory to serve from");
    let not_found_page = r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>404 Not Found</title><style>body{font-family:Arial,sans-serif;text-align:center;padding:50px;background-color:#f2f2f2}h1{font-size:72px;margin-bottom:10px}p{font-size:24px;margin-top:0}a{color:#3498db;text-decoration:none}a:hover{text-decoration:underline}</style></head><body><h1>404</h1><p>Page not found :)</p><p><a href="\">Go back home</a></p></body></html>"#;

    loop {
        // 7.1.1.2.  Date
        //  Date: Tue, 15 Nov 1994 08:12:31 GMT
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
                    // just remove url params
                    let resource = resource.split_once('?').map(|(path, _)| path).unwrap_or(resource);

                    let mut path = std::path::PathBuf::new();
                    path.push("www");
                    path.push(resource.trim_start_matches("/"));
                    println!("path {path:?}");

                    let mut path = std::path::PathBuf::from(path.to_str().unwrap().replace("%20", " "));
                    println!("path replaced {path:?}");

                    match path.canonicalize() {
                        Ok(canon) => path = canon,
                        Err(_) => {
                            stream.write_all(b"HTTP/1.1 404 Not Found\r\n").unwrap();
                            stream.write_all(format!("Content-Length: {}\r\n", not_found_page.as_bytes().len()).as_bytes()).unwrap();
                            stream.write_all(b"Content-Type: text/html\r\n").unwrap(); 
                            stream.write_all(b"Connection: close\r\n\r\n").unwrap();
                            stream.write_all(not_found_page.as_bytes()).unwrap();
                            break;
                        }
                    }
                    if resource.ends_with('/') {
                        path.push("index.html");
                    }

                    println!("canonical path {path:?}");

                    if !path.starts_with(&www_path) {
                        stream.write_all(b"HTTP/1.1 403 Forbidden\r\n").unwrap();
                        stream.write_all(b"Connection: close\r\n\r\n").unwrap();
                        break;
                    }

                    if !path.exists() {
                        stream.write_all(b"HTTP/1.1 404 Not Found\r\n").unwrap();
                        stream.write_all(format!("Content-Length: {}\r\n", not_found_page.as_bytes().len()).as_bytes()).unwrap();
                        stream.write_all(b"Content-Type: text/html\r\n").unwrap(); 
                        stream.write_all(b"Connection: close\r\n\r\n").unwrap();
                        stream.write_all(not_found_page.as_bytes()).unwrap();
                        break;
                    }

                    let content = std::fs::read(&path).unwrap();
                    stream.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
                    stream.write_all(b"Connection: close\r\n").unwrap();
                    stream.write_all(format!("Content-Length: {}\r\n", content.len()).as_bytes()).unwrap();
                    stream.write_all(b"Content-Type: text/html\r\n").unwrap(); 
                    stream.write_all(b"\r\n").unwrap();
                    stream.write_all(&content).unwrap();
                },
                [_, _, "HTTP/1.1"] => {
                    stream.write_all(b"HTTP/1.1 405 Method Not Allowed\r\n").unwrap();
                    stream.write_all(b"Allow: GET\r\n").unwrap();
                    stream.write_all(b"Connection: close\r\n\r\n").unwrap();
                    break;
                },
                _ => {
                    stream.write_all(b"HTTP/1.1 505 HTTP Version Not Supported\r\n").unwrap();
                    stream.write_all(b"Connection: close\r\n\r\n").unwrap();
                    break;
                },
            }
        }

    }
}
