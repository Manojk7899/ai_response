use std::env;

#[tokio::main]
async fn main() {
        dotenvy::dotenv().ok(); // loads .env if present

        let api_key=env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
        let models=env::var("GEMINI_MODELS").expect("GEMINI_MODELS not set");
        let prompt = "Write a poem about the beauty of nature.";
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{models}:generateContent?key={api_key}");
        let body=serde_json::json!({
            "contents":[
                {
                    "parts": [
                        {
                            "text":prompt
                        }
                    ]
                }
            ]
        });
        let client=reqwest::Client::new();
        let response=client.post(&url).json(&body).send().await.unwrap();
        println!("Status: {}", response.status());
        let raw_text=response.text().await.unwrap();
        println!("{}",raw_text);

}