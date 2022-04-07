use clap::{Subcommand, Parser};

/*
pub fn read_json_to_header_map(path: String) -> Result<HeaderMap, &'static str>{
    //TODO
    Ok(HeaderMap::new())
}
 */
#[derive(Debug, Parser)]
#[clap(name = "WebClient-rs", about = "A Http Client", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    #[clap(short, long)]
    pub logging:bool,
    #[clap(short, long, required = true)]
    pub url: String,
    #[clap(short, long, help = "Print response content to terminal")]
    pub content: bool,
    #[clap(short, long, help = "Print response information to terminal")]
    pub info: bool,
    #[clap(long, help = "Use request headers from specified json")]
    pub header_type_path: Option<String>,
    #[clap(short, long, help = "File to upload, if any. Use with correct type of request.")]
    pub file_path: Option<String>
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    //#[clap()]
    #[clap(name = "Download", about = "Download file", long_about = None, alias = "download")]
    Download {
        #[clap(short, long, help = "Output File Path")]
        outpath: String,
    },
    SiteDownload{
        #[clap(short, long, required = true, help = "Output directory for Site files. Will create if it doesn't exist.")]
        outputdir: String,
        #[clap(long, help = "Download level", default_value = "0")]
        level: i8
    },
    #[clap(name = "GET", about = "Send GET request", long_about = None, alias="get")]
    GET{

    },
    #[clap(name = "PUT", about = "Send PUT request", long_about = None, alias="put")]
    PUT{

    },
    #[clap(name = "POST", about = "Send POST request", long_about = None, alias="post")]
    POST{

    },
    #[clap(name = "DELETE", about = "Send DELETE request", long_about = None, alias="delete")]
    DELETE{

    },
    #[clap(name = "OPTIONS", about = "Send OPTIONS request", long_about = None, alias="options")]
    OPTIONS{

    },
    #[clap(name = "HEAD", about = "Send HEAD request", long_about = None, alias="head")]
    HEAD{

    },
    #[clap(name = "CONNECT", about = "Send CONNECT request", long_about = None, alias="connect")]
    CONNECT{

    },
    #[clap(name = "PATCH", about = "Send PATCH request", long_about = None, alias="patch")]
    PATCH{

    },
    #[clap(name = "TRACE", about = "Send TRACE request", long_about = None, alias="trace")]
    TRACE{

    },
}

