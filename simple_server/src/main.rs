use std::fs::File;
use std::io::Read;
use std::path::Path;
use tiny_http::{Server, Response, Header};

fn main() {
    // Create a server listening on port 5000
    let server = Server::http("0.0.0.0:5000").unwrap();
    println!("Server running at http://0.0.0.0:5000");
    println!("Visit http://0.0.0.0:5000 in your browser");

    // Handle incoming requests
    for request in server.incoming_requests() {
        println!("Received request: {} {}", request.method(), request.url());

        // Default to serving hello_world.html for any request
        let file_path = if request.url() == "/" {
            "./static/hello_world.html"
        } else {
            // Remove the leading slash
            &request.url()[1..]
        };

        // Try to read the file
        let path = Path::new(file_path);
        let response = if path.exists() && path.is_file() {
            match File::open(path) {
                Ok(mut file) => {
                    let mut contents = Vec::new();
                    if file.read_to_end(&mut contents).is_ok() {
                        // Determine content type based on file extension
                        let content_type = match path.extension().and_then(|ext| ext.to_str()) {
                            Some("html") => "text/html",
                            Some("css") => "text/css",
                            Some("js") => "application/javascript",
                            _ => "application/octet-stream",
                        };

                        let header = Header::from_bytes("Content-Type", content_type).unwrap();
                        Response::from_data(contents).with_header(header)
                    } else {
                        Response::from_string("Error reading file").with_status_code(500)
                    }
                }
                Err(_) => Response::from_string("Error opening file").with_status_code(500),
            }
        } else {
            // If file doesn't exist, default to hello_world.html
            match File::open("./static/hello_world.html") {
                Ok(mut file) => {
                    let mut contents = Vec::new();
                    if file.read_to_end(&mut contents).is_ok() {
                        let header = Header::from_bytes("Content-Type", "text/html").unwrap();
                        Response::from_data(contents).with_header(header)
                    } else {
                        Response::from_string("Error reading file").with_status_code(500)
                    }
                }
                Err(_) => Response::from_string("File not found").with_status_code(404),
            }
        };

        // Send the response
        if let Err(e) = request.respond(response) {
            println!("Error sending response: {:?}", e);
        }
    }
}
