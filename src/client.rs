use std::fs;
use hyper::{Body, Request, Response, Client, Method, HeaderMap, StatusCode};
use hyper_tls::HttpsConnector;
use scraper::Html;
use url::Url;
use std::path::Path;
use std::str::FromStr;
use hyper::body::HttpBody;
use hyper::header::{HeaderName, HeaderValue};
use serde_json::Value;
use indicatif::{ProgressBar, ProgressStyle};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type UrlList = Result<(Vec<String>, Option<Vec<Response<Body>>>)>;

pub struct WebClient {

}


impl WebClient {
    pub fn new() -> Self{
        WebClient {

        }
    }
    pub async fn send_request(&self, url: &String, m: Method, bytes: Vec<u8>, header_map: HeaderMap, follow_redirects: bool, print_progress: bool) -> Result<Response<Body>> {
        let tls = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(tls);
        let req = Request::builder()
            .method(m.clone())
            .uri(url)
            .body(Body::from(bytes.clone()))?;
        let mut parts = req.into_parts();
        parts.0.headers = header_map;
        let req = Request::from_parts(parts.0, parts.1);
        let res = client.request(req).await;
        let mut respbytes = Vec::new();

        return match res {
            Ok(mut r) => {
                let mut size:f64 = 0.0;
                if follow_redirects{
                    if (300..399).contains(&r.status().as_u16()){
                        let location = r.headers().get("location").unwrap().to_str().unwrap().to_string();
                        let newreq = Request::builder()
                            .method(m.clone())
                            .uri(location.clone())
                            .body(Body::from(bytes.clone()))?;
                        let newresp = client.request(newreq).await?;
                        r = newresp;
                        println!("Redirected to {}", location);
                    }
                }
                if print_progress {
                    let length = r.size_hint().upper().unwrap_or(0) as f64;
                    let pb = ProgressBar::new(length as u64);
                    pb.set_style(
                        ProgressStyle::with_template(
                            "[{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})",
                        )
                            .unwrap()
                            .progress_chars("#>-"),
                    );
                    while let Some(next) = r.data().await {
                        let chunk = next?;
                        respbytes.append(&mut chunk.to_vec());
                        size += chunk.len() as f64;
                        pb.set_position(size as u64);
                    }
                    pb.finish();
                    let response = r.into_parts();
                    r = Response::from_parts(response.0, Body::from(respbytes));
                }
                Ok(r)
            },
            Err(e) => {
                Err(Box::from(e.to_string()))
            }
        }
    }

    pub async fn get_urls_deep(&self, url: &String, headers: HeaderMap, level: i8) -> UrlList{
        let url_list = self.get_urls_base(url, headers.clone()).await;

        match url_list {
            Ok(list) =>{
                let mut list = list.0;
                let mut level_2:Vec<String> = Vec::new();
                let mut responses:Vec<Response<Body>> = Vec::new();
                match level{
                    2 =>{
                        for newurl in list.clone(){
                            //  println!("{}", newurl);
                            println!("Searching Url lv{}: {}", level, newurl);
                            let mut new_list = self.get_urls_base(&newurl, headers.clone()).await?;
                            level_2.append(&mut new_list.0);
                            responses.push((new_list.1).unwrap_or(Response::new(Body::from(""))));
                        }
                        return Ok((level_2, Some(responses)));
                    }
                    3 => {
                        for newurl in list.clone() {
                            println!("Searching Url lv{}: {}", level, newurl);
                            let mut new_list = self.get_urls_base(&newurl, headers.clone()).await?;
                            //new_list.0 = new_list.0.into_iter().unique().collect();
                            list.append(&mut new_list.0);
                            responses.push((new_list.1).unwrap_or(Response::new(Body::from(""))));
                        }
                    }
                    _ =>{
                        for url in list.clone().iter(){
                            println!("Searching Url lv{}: {}", level, url);
                            let resp = self.send_request(url, Method::GET, Vec::new(), HeaderMap::new(), false, false).await?;
                            responses.push(resp);
                        }
                        return Ok((list, Some(responses)))
                    }
                }
                return Ok((list, Some(responses)));
            }
            Err(e)=>{
                return Err(e)
            }
        }
    }

    pub async fn get_urls_base(&self, url: &String, headers: HeaderMap) -> Result<(Vec<String>, Option<Response<Body>>)>{
        let request = self.send_request(url, Method::GET, Vec::new(), headers, false, false).await;
        let mut urls:Vec<String> = Vec::new();

        return match request {
            Ok(mut resp) => {
                let bytes = hyper::body::to_bytes(resp.body_mut()).await?;
                let data = String::from_utf8(bytes.to_vec());
                if let Err(e) = data{
                    return Err(Box::new(e))
                }
                let data:String = data.unwrap();
                let document = Html::parse_document(data.as_str());
                for i in document.root_element().descendants() {
                    if let Some(el) = i.value().as_element() {
                        if let Some(href) = el.attr("href") {
                            if !href.contains("http") || !href.contains("https") {
                                let urlparse = Url::parse(url.as_str().clone()).unwrap();
                                let newurl = urlparse.join(href).unwrap();
                                let newurl = newurl.as_str().to_string();
                                if !urls.contains(&newurl) {
                                    urls.push(newurl);
                                }
                                continue
                            }
                        }
                        if let Some(src) = el.attr("src") {
                            if !src.contains("http") || !src.contains("https") {
                                println!("{}", src);
                                let urlparse = Url::parse(url.as_str().clone()).unwrap();
                                let newurl = urlparse.join(src).unwrap();
                                let newurl = newurl.as_str().to_string();
                                if !urls.contains(&newurl) {
                                    urls.push(newurl);
                                }
                                continue
                            }
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
            Err(e)=>{
                return Err(e)
            }
        }
    }
    pub fn url_to_file_path(&self, url:String) -> String{
        let sep = std::path::MAIN_SEPARATOR.to_string();
        url.strip_prefix("/").unwrap().replace("/", sep.as_str()).to_string()
    }
    pub async fn download(&self, download_url:&String, headers: HeaderMap, level: i8, output_directory: &String) -> Result<()>{
        let urls = self.get_urls_deep(download_url, headers, level).await?;
        let mut resps = urls.1.unwrap();
        let outputdir = Path::new(output_directory.as_str());
        let create_dirs = fs::create_dir_all(output_directory.clone());
        if create_dirs.is_err(){
            return Err(Box::from(create_dirs.unwrap_err().to_string()))
        }
        let mut current_index = 0;

        for url in urls.0{
            let urlparse = Url::parse(&mut url.as_str());
            if let Ok(urlparse) = urlparse{
                let mut path = self.url_to_file_path(urlparse.path().to_string());
                if Path::new(&path).file_name().is_none(){
                    path.push_str("index.html")
                }
                if Path::new(&path).extension().is_none(){
                    path.push_str(".html");
                }
                let url_file_path = Path::new(&path);
                if path == ""{
                    println!("Empty path");
                    current_index +=1;
                    continue
                }

                let bytes = hyper::body::to_bytes(&mut resps[current_index]).await?;
                let dir = outputdir.join(url_file_path);
                fs::create_dir_all(dir.parent().unwrap())?;
                println!("{}", dir.to_str().unwrap());
                let f = std::fs::write(dir, bytes);
                if let Err(e) = f{
                    eprintln!("Failed to write {} Error: {}", outputdir.join(url_file_path).to_str().unwrap(), e.to_string());
                    current_index +=1;
                    continue
                }
            }
            else{
                println!("{:?}", urlparse.unwrap_err().to_string());
                current_index +=1;
                continue
            }
            current_index +=1;
        }
        Ok(())

    }
}