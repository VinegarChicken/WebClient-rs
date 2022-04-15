use clap::{Subcommand, Parser};

/*
pub fn read_json_to_header_map(path: String) -> Result<HeaderMap, &'static str>{
    let map = HeaderMap::new();
    todo
    Ok(map)
}
 */

#[derive(Debug, Parser, Clone)]
#[clap(name = "WebClient-rs", about = "A Http Client", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    /*
    #[clap(short, long)]
    pub logging:bool,
    #[clap(long, help = "Use request headers from specified json")]
    pub header_type_path: Option<String>,
    Todo
     */
    #[clap(short, long, help = "File to upload, if any. Use with correct type of request.")]
    pub file_path: Option<String>
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    //#[clap()]
    #[clap(name = "Download", about = "Download file", long_about = None, alias = "download")]
    Download {
        url: String,
        #[clap(short, long, help = "Output File Path")]
        outpath: Option<String>,
    },
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

