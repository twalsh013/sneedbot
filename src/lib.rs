//use reqwest;
//use std::io::Read;


pub mod imagegrabber {
    use rand::Rng;
    use serde_json::{Result, Value};
    use std::io;
    use std::fs::File;
    use io::Cursor;

    //use serde::{Serialize, Deserialize};
    //use anyhow;

    //#[derive(Serialize, Deserialize, Debug)]
    //struct Image {
    //    data: Value,
    //}

    pub async fn pick_a_pic(tag: &String) -> Result<String> {//, reqwest::Error> {
        //let test: &str = "no";
        let client = reqwest::Client::new();
        let page: u32 =  rand::thread_rng().gen_range(1, 20);
        let ret: Result<String> = Ok("good".to_owned());

        let mut get_path = String::from("https://danbooru.donmai.us/posts.json?tags=");
        get_path.push_str(tag);
        get_path.push_str(" -loli");
        let pagestr = format!("&page={}",page);
        get_path.push_str(&pagestr);
        

        let res = client
        .get(get_path)
        .send()
        .await.unwrap().text().await.unwrap();

        //let res = res.text().await?;

        //let resp = reqwest::get(get_path)
        //.await?.text().await?;

        //let mut json: Value = serde_json::from_str(&res)?;
        let pics: Vec<Value> = serde_json::from_str(&res)?;

        let count = pics.len();
        if count < 1 {
            return Ok("none".to_owned())
        }

        let index = rand::thread_rng().gen_range(0, count-1);

       // let pic: Value = serde_json::from_str(&pics[count].data)?;
        
        let url: String = pics[index]["file_url"].as_str().unwrap().to_string();
        

        println!{"url = {}",url};
        let mut path = String::from("downloaded");

        if url.contains("png") {
            path.push_str(".png");
        }
        else if url.contains("jpg") {
            path.push_str(".jpg");
        }
        else if url.contains("jpeg") {
            path.push_str(".jpeg");
        }

        let rtmp = reqwest::get(url).await.unwrap();
        let mut content = Cursor::new(rtmp.bytes().await.unwrap());
        //let btmp = rtmp.text_with_charset("utf-8").await.unwrap();//.as_bytes();
        //let resp = btmp.as_bytes();
        //println!("{}",btmp);
        let mut out = File::create(path).expect("failed to create file");

        //let tmp = str::from_utf8(resp).unwrap();
        //let mut contents = tmp.as_bytes();        
        
        io::copy(&mut content, &mut out).expect("failed to copy content");

        ret
    }
}