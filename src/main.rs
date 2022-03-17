use hyper::{Body, Request, Response, Client};
use hyper::body::{Buf, HttpBody};
use hyper_tls::HttpsConnector;
use scraper::{Html, Selector};
use url::{Url, Host, Position};

mod client;
use client::*;





#[tokio::main]
async fn main() -> Result<()> {
    let client = WebClient::new();
    //let t = client.download("https://checkpoint.cc".to_string(), 0,
      //                      "C:\\Users\\m\\IdeaProjects\\WebClient-rs\\Test".to_string()).await?;
    let mut req = client.get_url("https://checkpoint.cc/static/css/app.c31fb923.css".to_string()).await?;
    let bytes = hyper::body::to_bytes(req.body_mut()).await?;
    let mut css = String::from_utf8(bytes.to_vec()).unwrap_or("".to_string());
    Ok(())
}

