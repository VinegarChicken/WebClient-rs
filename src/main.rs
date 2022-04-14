use clap::Parser;
use hyper::{Body, HeaderMap, Method};
use crate::WebClient;
use crate::Result;
mod client;
use client::*;
mod cli;
use cli::*;

fn https_check(url: String) -> String{
    let mut newurl = String::from("https://");
    if !url.contains(&"http://".to_string()) || !url.contains(&"https://".to_string()){
        newurl.push_str(url.as_str());
    }
    newurl
}

fn cmd_to_method(cmd: &Commands) -> Method {
    let mut method = Method::GET;
    match cmd {
        Commands::GET { .. } =>{
            method = Method::GET
        },
        Commands::PUT { .. } =>{
            method = Method::PUT
        },
        Commands::POST { .. } =>{
            method = Method::POST
        },
        Commands::DELETE { .. } =>{
            method = Method::DELETE
        }
        Commands::OPTIONS { .. } =>{
            method = Method::OPTIONS
        },
        Commands::HEAD { .. } =>{
            method = Method::HEAD
        },
        Commands::CONNECT { .. } =>{
            method = Method::CONNECT
        },
        Commands::PATCH { .. } =>{
            method = Method::PATCH
        },
        Commands::TRACE { .. } =>{
            method = Method::TRACE
        },
        _ => {}
    }
    method
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = WebClient::new();
    let cmd = Cli::parse();
    let map = HeaderMap::new();
    /*
    if let Some(json_path) = cmd.header_type_path{
        map = read_json_to_header_map(json_path).unwrap();
    }

     */
    let method = cmd_to_method(&cmd.command);
    match cmd.command {
        Commands::Download {outpath, url} => {
            let url = https_check(url);
            let resp = client.send_request(&url, Method::GET, Body::empty(), map.clone()).await;
            match resp {
                Err(e) => {
                    eprintln!("{}", e.to_string());
                }
                Ok(mut r) => {
                    let bytes = hyper::body::to_bytes(r.body_mut()).await?;
                    let f = std::fs::write(&outpath, bytes);
                    if f.is_ok(){
                        println!("Downloaded {} to {}", url, outpath);
                    }
                    else{
                        println!("{:?}", f.unwrap_err().to_string());
                    }
                }
            }
            return Ok(())
        },
        Commands::SiteDownload {url, outputdir, level} => {
            let url = https_check(url);
            let resp = client.download(&url, level, &outputdir).await;
            if let Err(e) = resp{
                eprintln!("{}", e.to_string());
            }
            else{
                println!("Downloaded {} to {}", url, outputdir);
            }
            return Ok(())
        }
        Commands::GET {url} | Commands::POST {url} | Commands::PUT {url} | Commands::CONNECT {url}  | Commands::PATCH {url} | Commands::DELETE {url} | Commands::OPTIONS {url} | Commands::HEAD {url} | Commands::TRACE {url} =>{
            let url = https_check(url);
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
            let mut resp = client.send_request(&url, method, Body::from(file_data), map).await?;
            let bytes = hyper::body::to_bytes(resp.body_mut()).await?;
            let data_string = String::from_utf8(bytes.to_vec());
            if let Err(e) = data_string{
                return Err(Box::from(e.to_string()))
            }
            let data_string = data_string.unwrap();
            println!("Response data: {}\n", data_string);
            println!("Response Info {:?}", resp);
        },

    }



    Ok(())
}

