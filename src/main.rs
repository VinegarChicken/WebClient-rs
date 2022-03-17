
use hyper::{Body, Request, Response, Client};
use hyper::body::{Buf, HttpBody};
use hyper_tls::HttpsConnector;
use scraper::{Html, Selector};
use url::{Url, Host, Position};

mod sitedownloader;
use sitedownloader::*;





#[tokio::main]
async fn main() -> Result<()> {
    let downloader = WebTool::new();
    //let sites = downloader.download(String::from("https://checkpoint.cc"), 2,
     //                               String::from("C:\\Users\\m\\IdeaProjects\\HttpClient-rs\\Download")).await?;
    let t = downloader.download("https://checkpoint.cc".to_string(), 0, "C:\\Users\\m\\IdeaProjects\\HttpClient-rs\\Test".to_string()).await?;

    Ok(())
}

