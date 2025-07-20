use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, process::Stdio, sync::Arc};
use tokio::{
  process::{Child, Command},
  signal,
  sync::Mutex,
};
use tower_http::services::ServeDir;

#[derive(Deserialize, Serialize)]
struct ChatMessage {
  role: String,
  content: String,
}

#[derive(Deserialize)]
struct ChatRequest {
  messages: Vec<ChatMessage>,
}

// #[derive(Serialize)]
// struct ChatResponse {
//   response: String,
// }

#[tokio::main]
async fn main() {
  // Shared child process for Ollama
  let ollama_process: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));

  // Launch Ollama and store the handle
  {
    let mut process = ollama_process.lock().await;
    *process = Some(launch_ollama_model().await);
  }

  // Clone the process handle for shutdown task
  let shutdown_process = ollama_process.clone();

  // Spawn shutdown handler
  tokio::spawn(async move {
    signal::ctrl_c()
      .await
      .expect("Failed to listen for shutdown signal");
    println!("\n🛑 Shutting down...");

    let mut proc_lock = shutdown_process.lock().await;
    if let Some(child) = proc_lock.as_mut() {
      match child.kill().await {
        Ok(_) => println!("✅ Ollama stopped."),
        Err(e) => eprintln!("❌ Failed to kill Ollama: {}", e),
      }
    }
    std::process::exit(0);
  });

  // Setup routes
  let frontend = ServeDir::new("client/build").not_found_service(ServeDir::new("client/build"));
  let api = Router::new().route("/chat", post(chat_handler));
  let app = Router::new().nest("/api", api).fallback_service(frontend);

  println!("🚀 Server running at http://127.0.0.1:3000/");

  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .expect("Failed to bind address");

  axum::serve(listener, app.into_make_service())
    .await
    .expect("Failed to start server");
}

async fn chat_handler(Json(req): Json<ChatRequest>) -> Json<serde_json::Value> {
  let res = reqwest::Client::new()
    .post("http://localhost:11434/api/chat")
    .json(&serde_json::json!({
        "model": "chatmlis-rbc",
        "messages": req.messages,
        "stream": false
    }))
    .send()
    .await;

  match res {
    Ok(resp) => {
      let json: serde_json::Value = resp.json().await.unwrap();
      // Debugging output
      // println!("{}", serde_json::to_string_pretty(&json).unwrap());
      // Json(ChatResponse {
      //   response: json["response"].as_str().unwrap_or("Error").to_string(),
      // })

      Json(json)
    }
    Err(_) => Json(serde_json::json!({
      "response": "Failed to connect to Ollama"
    })),
  }
}

async fn launch_ollama_model() -> Child {
  // Optional: check if it's already running
  if reqwest::get("http://localhost:11434").await.is_ok() {
    println!("✅ Ollama is already running.");
    return Command::new("true").spawn().unwrap(); // dummy no-op
  }

  println!("🔧 Launching Ollama with chatmlis-rbc...");

  Command::new("ollama")
    .arg("serve")
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .expect("❌ Failed to start Ollama")
}
