use std::fs;
use hyper::{Body, Request, Response, Client, Method, HeaderMap};
use hyper_tls::HttpsConnector;
use scraper::Html;
use url::Url;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type UrlList = Result<(Vec<String>, Option<Vec<Response<Body>>>)>;



pub struct WebClient {

}
/*
TODO
Breadth first or depth first?
A* search algorithm
 */

impl WebClient {
    pub fn new() -> Self{
        WebClient {

        }
    }
    pub async fn send_request(&self, url: &String, m: Method, b: Body, header_map: HeaderMap) -> Result<Response<Body>> {
        let tls = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(tls);
        let req = Request::builder()
            .method(m)
            .uri(url)
            .body(b)?;
        let mut parts = req.into_parts();
        parts.0.headers = header_map;
        let req = Request::from_parts(parts.0, parts.1);
        let res = client.request(req).await;
        return match res {
            Ok(e) => {
                Ok(e)
            },
            Err(e) => {
                Err(Box::from(e.to_string()))
            }
        }
    }

    pub async fn get_urls_deep(&self, url: &String, level: i8) -> UrlList{
        let url_list = self.get_urls_base(url).await;

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
                            let mut new_list = self.get_urls_base(&newurl).await?;
                            level_2.append(&mut new_list.0);
                            responses.push((new_list.1).unwrap_or(Response::new(Body::from(""))));
                        }
                        return Ok((level_2, Some(responses)));
                    }
                    3 => {
                        for newurl in list.clone() {
                            println!("Searching Url lv{}: {}", level, newurl);
                            let mut new_list = self.get_urls_base(&newurl).await?;
                            //new_list.0 = new_list.0.into_iter().unique().collect();
                            list.append(&mut new_list.0);
                            responses.push((new_list.1).unwrap_or(Response::new(Body::from(""))));
                        }
                    }
                    _ =>{
                        for url in list.clone().iter(){
                            println!("Searching Url lv{}: {}", level, url);
                            let resp = self.send_request(url, Method::GET, Body::empty(), HeaderMap::new()).await?;
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

    pub async fn get_urls_base(&self, url: &String) -> Result<(Vec<String>, Option<Response<Body>>)>{
        let request = self.send_request(url, Method::GET, Body::empty(), HeaderMap::new()).await;
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
    pub async fn download(&self, download_url:&String, level: i8, output_directory: String) -> Result<()>{
        let urls = self.get_urls_deep(download_url, level).await?;
        let mut resps = urls.1.unwrap();
        let outputdir = Path::new(output_directory.as_str());
        fs::create_dir_all(output_directory.clone())?;
        let index = self.send_request(download_url, Method::GET, Body::empty(), HeaderMap::new()).await?;
        let bytes = hyper::body::to_bytes(index).await?;
        let f = File::create(outputdir.join("index.html"));
        if let Ok(mut file) = f{
            file.write_all(bytes.as_ref());
        }
        else{
            let e = f.unwrap_err();
            return Err(Box::from(e.to_string()));
        }
        let mut current_index = 0;

        for url in urls.0{
            //println!("{:?}", url);
            let urlparse = Url::parse(&mut url.as_str());
            if let Ok(urlparse) = urlparse{
                let mut path = self.url_to_file_path(urlparse.path().to_string());
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
                println!("urlparse error");
                current_index +=1;
                continue
            }
            current_index +=1;
        }
        Ok(())

    }
}