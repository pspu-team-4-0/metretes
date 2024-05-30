use anyhow::Ok;
use axum::Json;
use reqwest::multipart;

pub async fn log_in(username: String, password: String) -> anyhow::Result<super::models::Root> {
    let c = reqwest::Client::new();
    let body = multipart::Form::new()
        .text("login", username)
        .text("password", password);
    let response = c
        .post("https://api-uae-test.ujin.tech/api/auth/authenticate/")
        .multipart(body).send().await?;
    Ok(serde_json::from_str(&response.text().await?)?)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String
}

// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct LoginOutput {
//     pub Id: String,
//     LifeTime: u32,
//     StartedAt: String
//   }