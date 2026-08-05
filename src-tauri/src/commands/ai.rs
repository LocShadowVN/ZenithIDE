use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AiMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct AiRequest {
    model: String,
    messages: Vec<AiMessage>,
}

#[derive(Deserialize)]
struct AiResponse {
    choices: Vec<AiChoice>,
}

#[derive(Deserialize)]
struct AiChoice {
    message: AiMessage,
}

#[tauri::command]
pub async fn ask_ai(
    api_url: String,
    api_key: String,
    model: String,
    prompt: String,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let req_body = AiRequest {
        model,
        messages: vec![AiMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let res = client
        .post(&api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&req_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("API Error: {}", res.status()));
    }

    let res_json = res.json::<AiResponse>().await.map_err(|e| e.to_string())?;

    res_json
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| "No response from AI".to_string())
}
