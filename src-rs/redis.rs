use reqwest::{self, StatusCode};
use std::{env::var, format };

pub async fn get_key(key: String) -> String {
   let redis_url = var("REDIS_URL").unwrap();
   let command = format!("{redis_url}/get/{key}");
   let redis_token = var("REDIS_TOKEN").unwrap();
   let result = reqwest::Client::new()
       .get(command)
       .header("Authorization", format!("Bearer {redis_token}"))
       .send()
       .await;
   match result {
       Ok(response)=>{
            match response.text().await {
                Ok(text)=>{
                    text
                },
                Err(error)=>{
                    println!("Error while getting response body {error}");
                    "".to_string()
                }
            }
       },
       Err(error)=>{
           println!("Error while sending http request {error}");
           "".to_string()
       }
   }
}

pub async fn setx_key(key: String, value: String, expiration: String) -> bool {
    let redis_url = var("REDIS_URL").unwrap();
    let command = format!("{redis_url}/setx/{key}/{expiration}/{value}");
    let redis_token = var("REDIS_TOKEN").unwrap();
    let result = reqwest::Client::new()
        .get(command)
        .header("Authorization", format!("Bearer {redis_token}"))
        .send()
        .await;
    match result{
        Ok(response)=>{
            match response.status() {
                StatusCode::OK=>{
                    true
                },
                _=>{
                    false
                }
            }
        },
        Err(err)=>{
            println!("Error while sending http request {err}");
            false
        }
    }
}
