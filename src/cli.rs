use std::str::FromStr;
use clap::{Subcommand, Parser};
use hyper::header::HeaderName;
use hyper::http::HeaderValue;
use crate::HeaderMap;
use crate::Result;
use serde_json;
use serde_json::{json, Value};




#[derive(Debug, Parser, Clone)]
#[clap(name = "WebClient-rs", about = "A Http Client", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    #[clap(short, long, help = "File to upload, if any. Use with correct type of request.")]
    pub file_path: Option<String>,
    #[clap(long, help = "Disables default behavior of following requests. When downloading a website, requests are not followed regardless.")]
    pub no_redirect: bool,
    #[clap(short, long, help = "Only print response information instead of response content")]
    pub info: bool,
    #[clap(short, long, help = "Write headers to json file")]
    pub dump_headers: Option<String>,
    #[clap(short, long, help = "Json file to read header data from.", long_help = "")]
    pub json_header_path: Option<String>
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    //#[clap()]
    #[clap(name = "Download", about = "Download file", long_about = None, alias = "d")]
    Download {
        url: String,
        #[clap(short, long, help = "Output File Path")]
        outpath: Option<String>,
    },
    #[clap(name = "site-download", about = "Download website", long_about = None, alias="sd")]
    SiteDownload{
        url: String,
        #[clap(short, long, help = "Output directory for Site files. Will create if it doesn't exist.")]
        outputdir: Option<String>,
        #[clap(long, help = "Download level", default_value = "0")]
        level: i8
    },
    #[clap(name = "GET", about = "Send GET request", long_about = None, alias="get")]
    GET{
        url: String,
    },
    #[clap(name = "PUT", about = "Send PUT request", long_about = None, alias="put")]
    PUT{
        url: String
    },
    #[clap(name = "POST", about = "Send POST request", long_about = None, alias="post")]
    POST{
        url: String
    },
    #[clap(name = "DELETE", about = "Send DELETE request", long_about = None, alias="delete")]
    DELETE{
        url: String
    },
    #[clap(name = "OPTIONS", about = "Send OPTIONS request", long_about = None, alias="options")]
    OPTIONS{
        url: String
    },
    #[clap(name = "HEAD", about = "Send HEAD request", long_about = None, alias="head")]
    HEAD{
        url: String
    },
    #[clap(name = "CONNECT", about = "Send CONNECT request", long_about = None, alias="connect")]
    CONNECT{
        url: String
    },
    #[clap(name = "PATCH", about = "Send PATCH request", long_about = None, alias="patch")]
    PATCH{
        url: String
    },
    #[clap(name = "TRACE", about = "Send TRACE request", long_about = None, alias="trace")]
    TRACE{
        url: String
    },
}