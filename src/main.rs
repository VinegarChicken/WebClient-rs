use std::io::Bytes;
use hyper::{Body, Request, Response, Client, Method, HeaderMap};
use hyper::body::{Buf, HttpBody};
use hyper_tls::HttpsConnector;
use scraper::{Html, Selector};
use url::{Url, Host, Position};

mod client;
use client::*;
mod cli;
use cli::*;
use clap::Parser;

/*
let bytes = hyper::body::to_bytes(req.body_mut()).await?;
    let mut css = String::from_utf8(bytes.to_vec()).unwrap_or("".to_string());
 */
/*
//let t = client.download("https://checkpoint.cc".to_string(), 0,
    //                      "C:\\Users\\m\\IdeaProjects\\WebClient-rs\\Test".to_string()).await?;
    let mut req = client.get_url("https://httpbin.org/post".to_string(), Method::POST,Body::empty()).await?;
    let bytes = hyper::body::to_bytes(req.body_mut()).await?;
    let mut css = String::from_utf8(bytes.to_vec()).unwrap_or("".to_string());
    println!("{}", css);
 */



#[tokio::main]
async fn main() -> Result<()> {
    let client = WebClient::new();
    let cmd = Cli::parse();
    let mut method = Method::GET;
    let mut map = HeaderMap::new();
    if let Some(json_path) = cmd.header_type_path{
        map = read_json_to_header_map(json_path).unwrap();
    }
    match cmd.command {
        /*
        Commands::Req {type_req, content, info, header_type_path } =>{
            let mut method = Method::GET;
            let headers = read_json_to_header_map(header_type_path.unwrap_or("".to_string()));
            let mut map = HeaderMap::new();
            if let Ok(h) = headers{
                map = h;
            }

            if upload && path.is_some(){
                let file_data = std::fs::read(path.unwrap());
                let resp = client.send_request(cmd.url, method, Body::from(file_data.unwrap_or(Vec::new())), map).await?;
            }
            else if let Some(p) = path{
                let mut resp = client.send_request(cmd.url, method, Body::empty(), map).await?;
                let bytes = hyper::body::to_bytes(resp.body_mut()).await?;
                std::fs::write(p, bytes);
            }
            else{
                let resp = client.send_request(cmd.url, method, Body::empty(), map).await?;
            }

        },

         */
        Commands::Download {outpath} => {
            let mut resp = client.send_request(cmd.url.clone(), Method::GET, Body::empty(), map.clone()).await?;
            let bytes = hyper::body::to_bytes(resp.body_mut()).await?;
            std::fs::write(outpath, bytes);
        },
        Commands::SiteDownload {outputdir} => {
            println!("Url: {}", outputdir)
        }
        Commands::GET {} =>{
            method = Method::GET
        },
        Commands::PUT {} =>{
            method = Method::PUT
        },
        Commands::POST {} =>{
            method = Method::POST
        },
        Commands::DELETE {} =>{
            method = Method::DELETE
        }
        Commands::OPTIONS {} =>{
            method = Method::OPTIONS
        },
        Commands::HEAD {} =>{
            method = Method::HEAD
        },
        Commands::CONNECT {} =>{
            method = Method::CONNECT
        },
        Commands::PATCH {} =>{
            method = Method::PATCH
        },
        Commands::TRACE {} =>{
            method = Method::TRACE
        },
    }
    let mut file_data:Vec<u8> = Vec::new();
    if let Some(path) = cmd.file_path{
        let mut bytes = std::fs::read(path);
        file_data = bytes.unwrap_or(Vec::new());
        /*
        if let Err(e) = bytes{
            eprintln!("{}", e.to_string())
        }
         */
    }

    let mut resp = client.send_request(cmd.url, method, Body::from(file_data), map).await?;
    let bytes = hyper::body::to_bytes(resp.body_mut()).await?;
    let mut data_string = String::from_utf8(bytes.to_vec()).unwrap_or("".to_string());
    if cmd.content{
        println!("\nResponse Content: {}", data_string);
    }
    if cmd.info{
        println!("\nResponse Info {:?}", resp);
    }


    Ok(())
}

