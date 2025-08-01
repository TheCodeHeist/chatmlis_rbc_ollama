use axum::{Json, Router, routing::post};
use html2text::from_read;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{process::Stdio, sync::Arc};
use tokio::{
  process::{Child, Command},
  signal,
  sync::Mutex,
};
use tower_http::services::ServeDir;
use urlencoding::encode;

#[derive(Deserialize, Serialize, Debug)]
struct ChatMessage {
  role: String,
  content: String,
}

#[derive(Deserialize, Debug)]
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

async fn search_duckduckgo_and_extract_links(query: &str) -> Result<Vec<String>, reqwest::Error> {
  let encoded = encode(query);
  let body = reqwest::Client::new()
    .post("https://html.duckduckgo.com/html/")
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(format!("q={}", encoded))
    .send()
    .await?
    .text()
    .await?;

  let document = Html::parse_document(&body);
  let selector = Selector::parse("a.result__a").unwrap();

  let mut results = vec![];
  for element in document.select(&selector).take(3) {
    // let title = element.text().collect::<Vec<_>>().join(" ");
    let href = element.value().attr("href").unwrap_or("");
    // results.push(format!("- {} ({})", title.trim(), href));
    results.push(href.to_string());
  }

  Ok(results)
}

async fn fetch_page_text(url: &str) -> Option<String> {
  let client = reqwest::Client::builder()
    .user_agent("Mozilla/5.0 (rust-agent)")
    .build()
    .ok()?;

  let response = client.get(url).send().await.ok()?;
  let bytes = response.bytes().await.ok()?;

  // Parse HTML into readable plain text
  let plain_text = match from_read(&bytes[..], 80) {
    Ok(text) => text,
    Err(_) => return None,
  };

  let truncated = plain_text.chars().take(1000).collect::<String>(); // Keep only first 1000 chars to stay under context window

  Some(truncated)
}

async fn chat_handler(Json(mut req): Json<ChatRequest>) -> Json<serde_json::Value> {
  // println!("Received chat request: {:?}", req.messages);

  // // Make a DuckDuckGo search for context
  // let user_query = match req.messages.iter().find(|m| m.role == "user").iter().last() {
  //   Some(msg) => &msg.content,
  //   None => return Json(serde_json::json!({"response": "No user message found"})),
  // };

  // println!("After: {:?}", req.messages);

  // // let search_results = search_duckduckgo_and_extract_links(user_query).await.unwrap_or_default();
  // let mut combined_context = String::new();

  // for link in search_duckduckgo_and_extract_links(user_query)
  //   .await
  //   .unwrap_or_default()
  //   .into_iter()
  //   .take(3)
  // // Limit to first 3 links
  // {
  //   if let Some(text) = fetch_page_text(&link).await {
  //     combined_context.push_str(&format!("From {}:\n{}\n\n", link, text));
  //   }
  // }

  // req.messages.push(ChatMessage {
  //   role: "system".to_string(),
  //   content: format!("Search results: \n\n{}\n\n", combined_context),
  // });

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
      println!("{}", serde_json::to_string_pretty(&json).unwrap());

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
