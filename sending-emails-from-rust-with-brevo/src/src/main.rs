use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Mail {
    pub api_key: String,
    pub to_address: String,
    pub to_name: String,
    pub from_address: String,
    pub from_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub mail: Mail,
}

impl Settings {
    pub fn new() -> Self {
        let s = match Config::builder()
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()
        {
            Ok(s) => s,
            Err(err) => panic!("Couldn't build configuration. Error: {}", err),
        };

        match s.try_deserialize() {
            Ok(s) => s,
            Err(err) => panic!("Couldn't deserialize configuration. Error: {}", err),
        }
    }
}

fn main() {
    let s = Settings::new();

    let body_str = format!(r#"{{
        "sender": {{
            "name": "{}",
            "email": "{}"
        }},
        "to": [
            {{
                "name": "{}",
                "email": "{}"
            }}
        ],
        "subject": "Testing brevo",
        "htmlContent": "<html><body>Hello, world!</body></html>"
    }}"#, s.mail.from_name, s.mail.from_address, s.mail.to_name, s.mail.to_address);
    let body: serde_json::Value = serde_json::from_str(&body_str).expect("Invalid JSON");

    let client = reqwest::blocking::Client::new();
    match client.post("https://api.brevo.com/v3/smtp/email")
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .header("api-key", s.mail.api_key)
            .json(&body)
            .send() {
        Ok(res) => {
            println!("Status: {}", res.status());
            match res.text() {
                Ok(rt) => println!("Response: {}", rt),
                Err(err) => panic!("Error: {}", err),
            }
        },
        Err(err) => panic!("Error sending the request. Error: {}", err),
    }
}
