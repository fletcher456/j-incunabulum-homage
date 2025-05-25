use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tiny_http::{Server, Response, Header, Method, Request};
use std::collections::VecDeque;

// Store messages in a thread-safe container
struct AppState {
    messages: Mutex<VecDeque<String>>,
}

fn main() {
    // Create a server listening on port 5000
    let server = Server::http("0.0.0.0:5000").unwrap();
    println!("Server running at http://0.0.0.0:5000");
    println!("Visit http://0.0.0.0:5000 in your browser");

    // Create shared state to store messages
    let state = Arc::new(AppState {
        messages: Mutex::new(VecDeque::new()),
    });

    // Handle incoming requests
    for mut request in server.incoming_requests() {
        println!("Received request: {} {}", request.method(), request.url());
        
        let method = request.method().clone();
        let url = request.url().to_string();
        
        let response = match (method, url.as_str()) {
            (Method::Post, "/submit") => {
                // We need to read the body for POST requests
                let content_length = request
                    .headers()
                    .iter()
                    .find(|h| h.field.equiv("Content-Length"))
                    .and_then(|h| h.value.as_str().parse::<usize>().ok())
                    .unwrap_or(0);
                
                let mut buffer = vec![0; content_length];
                if let Ok(_) = request.as_reader().read_exact(&mut buffer) {
                    // Parse the form data
                    let body = String::from_utf8_lossy(&buffer);
                    if let Some(message) = body.strip_prefix("message=") {
                        // URL decode the message
                        let message = url_decode(message);
                        
                        // Add to message queue
                        let mut messages = state.messages.lock().unwrap();
                        messages.push_front(message);
                        
                        // Keep only the last 10 messages
                        while messages.len() > 10 {
                            messages.pop_back();
                        }
                    }
                }
                
                // Redirect back to the main page
                let header = Header::from_bytes("Location", "/").unwrap();
                Response::from_string("").with_status_code(303).with_header(header)
            },
            (Method::Get, "/") | (Method::Get, "/hello_world.html") => {
                serve_html_with_messages(&state)
            },
            _ => {
                // Serve static files for other requests
                serve_static_file(&url)
            }
        };

        // Send the response
        if let Err(e) = request.respond(response) {
            println!("Error sending response: {:?}", e);
        }
    }
}

// No longer needed as we've inlined this functionality

// Simple URL decoder
fn url_decode(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let bytes = input.as_bytes();
    
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i+1..i+3]).unwrap_or("");
                if let Ok(val) = u8::from_str_radix(hex, 16) {
                    result.push(val as char);
                    i += 3;
                } else {
                    result.push('%');
                    i += 1;
                }
            },
            b'+' => {
                result.push(' ');
                i += 1;
            },
            b => {
                result.push(b as char);
                i += 1;
            }
        }
    }
    
    result
}

// Serve the HTML file with messages
fn serve_html_with_messages(state: &Arc<AppState>) -> Response<std::io::Cursor<Vec<u8>>> {
    match File::open("./static/hello_world.html") {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                // Generate HTML for messages
                let messages = state.messages.lock().unwrap();
                let messages_html = messages
                    .iter()
                    .rev() // Reverse the order so most recent is at the bottom
                    .map(|msg| format!("<div class=\"message\">{}</div>", html_escape(msg)))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                // Replace the placeholder with the messages
                let contents = contents.replace("$MESSAGES$", &messages_html);
                
                let header = Header::from_bytes("Content-Type", "text/html").unwrap();
                Response::from_string(contents).with_header(header)
            } else {
                Response::from_string("Error reading file").with_status_code(500)
            }
        },
        Err(_) => Response::from_string("File not found").with_status_code(404),
    }
}

// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#39;")
}

// Serve a static file
fn serve_static_file(url: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    // Remove the leading slash
    let file_path = if url.starts_with('/') {
        &url[1..]
    } else {
        url
    };

    // Try to read the file
    let path = Path::new(file_path);
    if path.exists() && path.is_file() {
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
        // File not found - return 404
        Response::from_string("File not found").with_status_code(404)
    }
}
