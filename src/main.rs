use clap::Parser;
use hyper::{Body, HeaderMap, Method};
use crate::WebClient;
use crate::Result;
mod client;
use client::*;
mod cli;
use cli::*;




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
        Commands::Download {outpath} => {
            let resp = client.send_request(&cmd.url, Method::GET, Body::empty(), map.clone()).await;
            match resp {
                Err(e) => {
                    eprintln!("{}", e.to_string());
                }
                Ok(mut r) => {
                    let bytes = hyper::body::to_bytes(r.body_mut()).await?;
                    std::fs::write(outpath, bytes);
                }
            }
            return Ok(())
        },
        Commands::SiteDownload {outputdir, level} => {
            let resp = client.download(&cmd.url, level, outputdir).await;
            if let Err(e) = resp{
                eprintln!("{}", e.to_string());
            }
            return Ok(())
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
        let bytes = std::fs::read(path);
        if let Ok(b) = bytes{
            file_data = b;
        }
        else{
            let e = bytes.unwrap_err();
            return Err(Box::from(e.to_string()))
        }

    }

    let mut resp = client.send_request(&cmd.url, method, Body::from(file_data), map).await?;
    let bytes = hyper::body::to_bytes(resp.body_mut()).await?;
    let data_string = String::from_utf8(bytes.to_vec());
    if let Err(e) = data_string{
        return Err(Box::from(e.to_string()))
    }
    let data_string = data_string.unwrap();
    if cmd.content{
        println!("\nResponse Content: {}", data_string);
    }
    if cmd.info{
        println!("\nResponse Info {:?}", resp);
    }
    Ok(())
}

