use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct OAIChoises {
    text: String,
}

#[derive(Deserialize, Debug)]
struct OAIResponse {
    choices: Vec<OAIChoises>,
}

#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    model: String,
    max_tokens: u32,
    temperature: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/completions";
    
    let preamble = "Eu sou um robô de pesquisa";

    let oai_token: String = String::from("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");
    let auth_header_val = format!("Bearer {}", oai_token);

    println!("{esc}c", esc = 27 as char);
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut user_text = String::new();

        stdin().read_line(&mut user_text)
            .expect("Failed to read line");
        println!("");

        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, user_text),
            model: String::from("text-curie-001"), //text-davinci-002
            max_tokens: 60,
            temperature: 0,
        };

        let body = Body::from(serde_json::to_vec(&oai_request)?);
        
        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();
        
        let res = client.request(req).await?;

        let body = hyper::body::aggregate(res).await?;

        let json: OAIResponse = serde_json::from_reader(body.reader())?;

        // Print the json
        println!("🤖: {}", json.choices[0].text);
    }
}