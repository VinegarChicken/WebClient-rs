use std::ffi::{OsStr, OsString};
use std::fs;
use hyper::{Body, Request, Response, Client};
use hyper::body::{Buf, Bytes, HttpBody};
use hyper_tls::HttpsConnector;
use scraper::{Html, Selector};
use url::{Url, Host, Position};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use lewp_css::domain::at_rules::media::MediaType::print;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type UrlList = Result<(Vec<String>, Option<Vec<Response<Body>>>)>;

pub struct WebTool {

}

pub static mut BYTES_GLOBAL:Bytes = Bytes::new();

impl WebTool {
    pub fn new() -> Self{
        WebTool {

        }
    }
    pub async fn get_url(&self, url: String) -> Result<Response<Body>> {
        let tls = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(tls);
        let req = Request::get(url)
            .body(Body::empty())
            .unwrap();
        let mut res = client.request(req).await?;
        Ok(res)
    }

    pub async fn get_urls_deep(&self, url: String, level: i8) -> UrlList{
        let urlList = self.get_urls_base(url).await;

        match urlList{
            Ok(mut list) =>{
                let mut list = list.0;
                let mut level_2:Vec<String> = Vec::new();
                let mut responses:Vec<Response<Body>> = Vec::new();
                match level{
                    2 =>{
                        for newurl in list.clone(){
                            //  println!("{}", newurl);
                            let mut newList = (self.get_urls_base(newurl).await?);
                            level_2.append(&mut newList.0);
                            responses.push((newList.1).unwrap_or(Response::new(Body::from(""))));
                        }
                        return Ok((level_2, Some(responses)));
                    }
                    3 => {
                        for newurl in list.clone() {
                            let mut newList = (self.get_urls_base(newurl).await?);
                            list.append(&mut newList.0);
                            responses.push((newList.1).unwrap());
                        }
                    }
                    _ =>{
                        for url in list.clone(){
                            let resp = self.get_url(url).await?;
                            responses.push(resp);
                        }
                        return Ok((list, Some(responses)))
                    }
                }
                return Ok((list, Some(responses)));
            }
            Err(e)=>{
                println!("Error")
            }
        }
        Ok((vec![String::from("error")], None))
    }

    pub async fn get_urls_base(&self, url: String) -> Result<(Vec<String>, Option<Response<Body>>)>{
        let request = self.get_url(url.clone()).await;
        let mut urls:Vec<String> = Vec::new();

        return match request {
            Ok(mut resp) => unsafe {
                let bytes = hyper::body::to_bytes(resp.body_mut()).await?;
                let mut data = String::from_utf8(bytes.to_vec()).unwrap_or("".to_string());
                if data == ""{
                    return Ok((vec![String::from("error")], Some(resp)))
                }
                let document = Html::parse_document(data.as_str());
                for i in document.root_element().descendants() {
                    if let Some(el) = i.value().as_element() {
                        if let Some(mut href) = el.attr("href") {
                            if !href.contains("http") || !href.contains("https") {
                                let mut urlparse = Url::parse(url.as_str().clone()).unwrap();
                                let newurl = urlparse.join(href).unwrap();
                                let newurl = newurl.as_str().to_string();
                                if !urls.contains(&newurl) {
                                    urls.push(newurl);
                                }
                                continue
                            }
                            let mut link = href.to_string();
                            urls.push(link);
                        }
                        if let Some(mut src) = el.attr("src") {
                            if !src.contains("http") || !src.contains("https") {
                                let mut urlparse = Url::parse(url.as_str().clone()).unwrap();
                                let newurl = urlparse.join(src).unwrap();
                                let newurl = newurl.as_str().to_string();
                                if !urls.contains(&newurl) {
                                    urls.push(newurl);
                                }
                                continue
                            }
                            let mut link = src.to_string();
                            urls.push(link);
                        }
                    }
                }
                //recreate the response obj since you can't stream the bytes twice
                let resp = Response::builder()
                    .status(resp.status())
                    .body(Body::from(bytes))
                    .unwrap();

                Ok((urls, Some(resp)))
            }
            Err(e) => {
                Ok((vec![String::from("error")], None))
            }
        }
    }
    pub fn url_to_file_path(&self, url:String) -> String{
        let sep = std::path::MAIN_SEPARATOR.to_string();
        url.strip_prefix("/").unwrap().replace("/", sep.as_str()).to_string()
    }
    pub async fn download(&self, download_url:String, level: i8, output_directory: String) -> Result<()>{
        let mut urls = self.get_urls_deep(download_url.clone(), level).await?;
        let mut resps = urls.1.unwrap();
        let outputdir = Path::new(output_directory.as_str());
        fs::create_dir_all(output_directory.clone())?;
        let index = self.get_url(download_url.clone()).await?;
        let bytes = hyper::body::to_bytes(index).await?;
        let mut f = File::create(outputdir.join("index.html"));
        if let Ok(mut file) = f{
            file.write_all(bytes.as_ref());
        }
        else{
            return Ok(())
        }

        let mut currentIndex = 0;
        for url in urls.0{
            //println!("{:?}", url);
            let mut urlparse = Url::parse(&mut url.as_str());
            if let Ok(urlparse) = urlparse{
                let path = self.url_to_file_path(urlparse.path().to_string());
                if path == ""{
                    continue
                }
                println!("{}", path);
                let url_file_path = Path::new(&path);
                let bytes = hyper::body::to_bytes(&mut resps[currentIndex]).await?;
                let dir = outputdir.join(url_file_path);
                fs::create_dir_all(dir.parent().unwrap())?;
                // println!("{}", dir.to_str().unwrap());
                let mut f = File::create(dir);
                if let Ok(mut file) = f{
                    file.write_all(&bytes);
                }
                else{
                    //eprintln!("Failed to write {}", outputdir.join(url_file_path).to_str().unwrap());
                    continue
                }
            }
            else{
                continue
            }
            currentIndex+=1;
        }
        Ok(())

    }
}
