mod j_interpreter;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware, get, post};
use actix_files as fs;
use j_interpreter::execute;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::VecDeque;
use log::info;

// Maximum number of history entries to keep
const MAX_HISTORY_SIZE: usize = 100;

// Structure to store command history
struct AppState {
    history: Mutex<VecDeque<HistoryEntry>>,
}

// Structure for a history entry
#[derive(Clone, Serialize, Deserialize)]
struct HistoryEntry {
    expression: String,
    result: String,
    is_error: bool,
}

// Structure for the evaluation request
#[derive(Deserialize)]
struct EvaluationRequest {
    expression: String,
}

// Structure for the evaluation response
#[derive(Serialize)]
struct EvaluationResponse {
    result: Option<String>,
    error: Option<String>,
}

// Structure for the history response
#[derive(Serialize)]
struct HistoryResponse {
    entries: Vec<HistoryEntry>,
}

// Handler for evaluating J expressions
async fn evaluate_expression(
    data: web::Json<EvaluationRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let expression = data.expression.trim();
    info!("Evaluating expression: {}", expression);
    
    // Execute the J expression
    let result = match execute(expression) {
        Ok(result) => {
            // Add to history
            let history_entry = HistoryEntry {
                expression: expression.to_string(),
                result: result.clone(),
                is_error: false,
            };
            
            add_to_history(app_state.as_ref(), history_entry);
            
            EvaluationResponse {
                result: Some(result),
                error: None,
            }
        }
        Err(err) => {
            let error_msg = err.to_string();
            
            // Add to history
            let history_entry = HistoryEntry {
                expression: expression.to_string(),
                result: error_msg.clone(),
                is_error: true,
            };
            
            add_to_history(app_state.as_ref(), history_entry);
            
            EvaluationResponse {
                result: None,
                error: Some(error_msg),
            }
        }
    };
    
    HttpResponse::Ok().json(result)
}

// Handler for getting command history
async fn get_history(app_state: web::Data<AppState>) -> impl Responder {
    let history_lock = app_state.history.lock().unwrap();
    let entries: Vec<HistoryEntry> = history_lock.iter().cloned().collect();
    
    HttpResponse::Ok().json(HistoryResponse { entries })
}

// Add an entry to the command history
fn add_to_history(app_state: &AppState, entry: HistoryEntry) {
    let mut history = app_state.history.lock().unwrap();
    
    // Add the new entry to the front
    history.push_front(entry);
    
    // Trim the history if it exceeds the maximum size
    while history.len() > MAX_HISTORY_SIZE {
        history.pop_back();
    }
}

fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Create app state
    let app_state = web::Data::new(AppState {
        history: Mutex::new(VecDeque::new()),
    });
    
    info!("Starting J Web REPL server on http://0.0.0.0:5000");
    
    // Start the HTTP server - use a basic approach that works with older actix versions
    actix_web::rt::System::new().block_on(async {
        HttpServer::new(move || {
            App::new()
                .app_data(app_state.clone())
                .service(web::resource("/evaluate").route(web::post().to(evaluate_expression)))
                .service(web::resource("/history").route(web::get().to(get_history)))
                .service(Files::new("/static", "./static").index_file("index.html"))
                .service(Files::new("/", "./static").index_file("index.html"))
        })
        .bind("0.0.0.0:5000")?
        .run()
        .await
    })
}