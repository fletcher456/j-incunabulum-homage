use std::fs::File;
use std::io::Read;
use serde_json;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tiny_http::{Server, Response, Header, Method, Request};
use std::collections::VecDeque;

// Import our modular J interpreter modules
mod j_array;
mod tokenizer;
mod parser;
mod custom_parser;
mod lalr_parser;
mod lalr_parser_test;
mod semantic_analyzer;
mod evaluator;
mod interpreter;
mod visualizer;
mod test_suite;

use interpreter::{JInterpreter, format_result};
use visualizer::ParseTreeVisualizer;
use lalr_parser::LalrParser;
use custom_parser::CustomParser;
use tokenizer::JTokenizer;
use semantic_analyzer::JSemanticAnalyzer;
use evaluator::JEvaluator;

// Store messages and J interpreter state in a thread-safe container
struct AppState {
    messages: Mutex<VecDeque<String>>,
    j_interpreter: JInterpreter,
}

fn main() {
    // Create a server listening on port 5000
    let server = Server::http("0.0.0.0:5000").unwrap();
    println!("Server running at http://0.0.0.0:5000");
    println!("Visit http://0.0.0.0:5000 in your browser");

    // Create shared state with J interpreter
    let state = Arc::new(AppState {
        messages: Mutex::new(VecDeque::new()),
        j_interpreter: JInterpreter::new(),
    });

    // Handle incoming requests
    for mut request in server.incoming_requests() {
        println!("Received request: {} {}", request.method(), request.url());
        
        let method = request.method().clone();
        let url = request.url().to_string();
        
        let response = match (method, url.as_str()) {
            // J REPL evaluation endpoint
            (Method::Post, "/j_eval") => {
                // Read the POST body
                let content_length = request
                    .headers()
                    .iter()
                    .find(|h| h.field.equiv("Content-Length"))
                    .and_then(|h| h.value.as_str().parse::<usize>().ok())
                    .unwrap_or(0);
                
                let mut buffer = vec![0; content_length];
                if let Ok(_) = request.as_reader().read_exact(&mut buffer) {
                    let body = String::from_utf8_lossy(&buffer);
                    println!("Received body: {}", body);
                    
                    // Parse JSON request to get parser choice  
                    let request_data: serde_json::Value = serde_json::from_str(&body).unwrap_or_else(|_| {
                        // Fallback for legacy form data
                        if let Some(expression) = body.strip_prefix("expression=") {
                            serde_json::json!({"expression": url_decode(expression), "parser": "lalrpop"})
                        } else {
                            serde_json::json!({"expression": body.trim(), "parser": "lalrpop"})
                        }
                    });
                    
                    let expression = request_data["expression"].as_str().unwrap_or("");
                    let parser_choice = request_data["parser"].as_str().unwrap_or("lalrpop");
                    
                    if expression.is_empty() {
                        let error_response = r#"{"error": "No expression provided"}"#;
                        let header = Header::from_bytes("Content-Type", "application/json").unwrap();
                        return Ok(Response::from_string(error_response).with_header(header));
                    }
                        
                        println!("Evaluating: {} (using {} parser)", expression, parser_choice);
                        
                        // Use appropriate parser with manual pipeline
                        let tokenizer = JTokenizer::new();
                        let semantic_analyzer = JSemanticAnalyzer::new();
                        let evaluator = JEvaluator::new();
                        let visualizer = ParseTreeVisualizer::new();
                        
                        let formatted_result = match tokenizer.tokenize(&expression) {
                            Ok(tokens) => {
                                let ast_result = match parser_choice {
                                    "custom" => {
                                        let custom_parser = CustomParser::new();
                                        custom_parser.parse(tokens)
                                    }
                                    _ => {
                                        let lalr_parser = LalrParser::new();
                                        lalr_parser.parse(tokens)
                                    }
                                };
                                
                                match ast_result {
                                    Ok(ast) => {
                                        let parse_tree_text = format!("{} Parse Tree:\n{}", 
                                            if parser_choice == "custom" { "Custom" } else { "LALRPOP" },
                                            visualizer.visualize(&ast));
                                        
                                        println!("Expression: {}", expression);
                                        println!("{}", parse_tree_text);
                                        
                                        match semantic_analyzer.analyze(ast) {
                                            Ok(resolved_ast) => {
                                                match evaluator.evaluate(&resolved_ast) {
                                                    Ok(result_array) => {
                                                        println!("Result: {}\n", result_array);
                                                        format!("{}", result_array)
                                                    }
                                                    Err(eval_err) => {
                                                        let error_text = format!("Evaluation Error: {}", eval_err);
                                                        println!("{}\n", error_text);
                                                        error_text
                                                    }
                                                }
                                            }
                                            Err(semantic_err) => {
                                                let error_text = format!("Semantic Error: {}", semantic_err);
                                                println!("{}\n", error_text);
                                                error_text
                                            }
                                        }
                                    }
                                    Err(parse_err) => {
                                        let error_text = format!("{} Parse Error: {}", 
                                            if parser_choice == "custom" { "Custom" } else { "LALRPOP" },
                                            parse_err);
                                        println!("Expression: {}", expression);
                                        println!("{}\n", error_text);
                                        error_text
                                    }
                                }
                            }
                            Err(token_err) => {
                                let error_text = format!("Token Error: {}", token_err);
                                println!("Expression: {}", expression);
                                println!("{}\n", error_text);
                                error_text
                            }
                        };
                        
                        // Return JSON response
                        let json_response = format!(
                            "{{\"result\": \"{}\"}}",
                            formatted_result.replace('"', "\\\"").replace('\n', "\\n")
                        );
                        
                        let header = Header::from_bytes("Content-Type", "application/json").unwrap();
                        Response::from_string(json_response).with_header(header)
                    } else {
                        let error_response = "{\"result\": \"Error: Invalid request format\"}";
                        let header = Header::from_bytes("Content-Type", "application/json").unwrap();
                        Response::from_string(error_response).with_header(header).with_status_code(400)
                    }
                } else {
                    let error_response = "{\"result\": \"Error: Could not read request body\"}";
                    let header = Header::from_bytes("Content-Type", "application/json").unwrap();
                    Response::from_string(error_response).with_header(header).with_status_code(400)
                }
            },
            // Original message submission (kept for backward compatibility)
            (Method::Post, "/submit") => {
                // Read the POST body
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
                        messages.push_front(format!("<div class=\"message\">{}</div>", html_escape(&message)));
                        
                        // Keep only the last 10 messages
                        while messages.len() > 10 {
                            messages.pop_back();
                        }
                    }
                }
                
                // Redirect to the J REPL page
                let header = Header::from_bytes("Location", "/").unwrap();
                Response::from_string("").with_status_code(303).with_header(header)
            },
            (Method::Get, "/") => {
                // Default to J REPL interface
                serve_j_repl_with_messages(&state)
            },
            (Method::Get, "/hello_world.html") => {
                // Original chat interface (kept for backward compatibility)
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

// Serve the J REPL page with messages
fn serve_j_repl_with_messages(state: &Arc<AppState>) -> Response<std::io::Cursor<Vec<u8>>> {
    match File::open("./static/j_repl.html") {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                // Generate HTML for messages
                let messages = state.messages.lock().unwrap();
                let messages_html = format!(
                    "<div class=\"message-container\">{}</div>",
                    messages
                        .iter()
                        .rev() // Reverse the order so most recent is at the bottom
                        .map(|msg| msg.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                
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

// Serve the original HTML file with messages (for backward compatibility)
fn serve_html_with_messages(state: &Arc<AppState>) -> Response<std::io::Cursor<Vec<u8>>> {
    match File::open("./static/hello_world.html") {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                // Generate HTML for messages
                let messages = state.messages.lock().unwrap();
                let messages_html = format!(
                    "<div class=\"message-container\">{}</div>",
                    messages
                        .iter()
                        .rev() // Reverse the order so most recent is at the bottom
                        .map(|msg| if msg.contains("class=\"message") { 
                            msg.to_string() 
                        } else { 
                            format!("<div class=\"message\">{}</div>", html_escape(msg)) 
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                
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

    // Try to read the file from the static directory
    let static_path = format!("./static/{}", file_path);
    let path = Path::new(&static_path);
    
    println!("Looking for static file at: {}", static_path);
    
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
            Err(e) => {
                println!("Error opening file: {:?}", e);
                Response::from_string("Error opening file").with_status_code(500)
            },
        }
    } else {
        // File not found - return 404
        println!("File not found: {}", static_path);
        Response::from_string("File not found").with_status_code(404)
    }
}
