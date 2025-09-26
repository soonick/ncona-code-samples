use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Utc;
use hex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use url::form_urlencoded;
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator};

pub const PUBLIC_URL_EXPIRATION_SECS: &'static u16 = &3_600; // 1 hour
pub const BUCKET_NAME: &'static str = "";
pub const OBJECT_PATH: &'static str = "";

#[derive(Serialize)]
struct SignBlobRequest {
    payload: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignBlobResponse {
    signed_blob: String,
}

async fn get_service_account_email() -> Result<String, ()> {
    let key_path = match env::var("GOOGLE_APPLICATION_CREDENTIALS") {
        Ok(v) => v,
        Err(e) => {
            println!("Couldn't get GOOGLE_APPLICATION_CREDENTIALS env variable. {}", e);
            return Err(());
        }
    };
    let sa_key = match read_service_account_key(key_path).await {
        Ok(k) => k,
        Err(e) => {
            println!("Couldn't read the key. {}", e);
            return Err(());
        }
    };
    Ok(sa_key.client_email)
}

async fn get_oauth2_token() -> Result<String, ()> {
    let key_path = match env::var("GOOGLE_APPLICATION_CREDENTIALS") {
        Ok(kp) => kp,
        Err(e) => {
            println!("Error getting GOOGLE_APPLICATION_CREDENTIALS: {}", e);
            return Err(());
        }
    };
    let sa_key = match read_service_account_key(key_path.clone()).await {
        Ok(k) => k,
        Err(e) => {
            println!("Error getting key from {}: {}", key_path, e);
            return Err(());
        }
    };
    let auth = match ServiceAccountAuthenticator::builder(sa_key).build().await {
        Ok(a) => a,
        Err(e) => {
            println!("Failed to build ServiceAccountAuthenticator: {}", e);
            return Err(());
        }
    };
    let token_response = match auth
        .token(&["https://www.googleapis.com/auth/cloud-platform"])
        .await
    {
        Ok(r) => r,
        Err(e) => {
            println!("Failed to get token from authenticator: {}", e);
            return Err(());
        }
    };
    let token = match token_response.token() {
        Some(t) => t.to_string(),
        None => {
            println!("Token response doesn't contain token field");
            return Err(());
        }
    };

    Ok(token)
}

#[tokio::main]
async fn main() {
    let now = Utc::now();
    let now_iso = now.format("%Y%m%dT%H%M%SZ");

    let email = match get_service_account_email().await {
        Ok(e) => e,
        Err(_) => {
            panic!("Couldn't get service account e-mail");
        }
    };
    let token = match get_oauth2_token().await {
        Ok(t) => t,
        Err(_) => {
            panic!("Couldn't get oauth token");
        }
    };

    let credential_scope = format!("{}/auto/storage/goog4_request", now.format("%Y%m%d"));
    let goog_credential = format!("{}/{}", email, credential_scope);
    let encoded_credential: String =
        form_urlencoded::byte_serialize(goog_credential.as_bytes()).collect();
    let canonical_query_string = format!(
        "X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential={}&X-Goog-Date={}&X-Goog-Expires={}&X-Goog-SignedHeaders=host",
        encoded_credential,
        now_iso,
        PUBLIC_URL_EXPIRATION_SECS,
    );

    let path_to_resource = format!("/{}/{}", BUCKET_NAME, OBJECT_PATH);
    let canonical_request = format!(
        "GET\n{}\n{}\nhost:storage.googleapis.com\n\nhost\nUNSIGNED-PAYLOAD",
        path_to_resource, canonical_query_string,
    );

    let string_to_sign = format!(
        "GOOG4-RSA-SHA256\n{}\n{}\n{}",
        now_iso,
        credential_scope,
        hex::encode(Sha256::digest(canonical_request))
    );

    // Use IAM Credentials API to sign
    let sign_url = format!(
        "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/{}:signBlob",
        email
    );

    let client = Client::new();
    let sign_resp = match client
        .post(&sign_url)
        .bearer_auth(&token)
        .json(&SignBlobRequest {
            payload: BASE64_STANDARD.encode(string_to_sign.as_bytes()),
        })
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            panic!("Sign blob request failed: {}", e);
        }
    };

    if !sign_resp.status().is_success() {
        panic!(
            "Sign blob request failed with status: {}. Response: {:?}",
            sign_resp.status(),
            sign_resp.text().await
        );
    }

    let sign_resp: SignBlobResponse = match sign_resp.json().await {
        Ok(r) => r,
        Err(e) => {
            panic!("Converting sign blob response to json failed: {}", e);
        }
    };

    let signature = match BASE64_STANDARD.decode(&sign_resp.signed_blob) {
        Ok(d) => hex::encode(d),
        Err(e) => {
            panic!("Base64 decoding of signed blob failed: {}", e);
        }
    };

    // Construct the final signed URL
    let url = format!(
        "https://storage.googleapis.com/{}/{}?{}&X-Goog-Signature={}",
        BUCKET_NAME, OBJECT_PATH, canonical_query_string, signature
    );

    println!("{}", url);
}
