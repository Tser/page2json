// use reqwest::{ blocking::Client, Error};
use reqwest;


fn main(){

    let response = reqwest::blocking::get("https://www.python.org/ftp/python/").unwrap().text();

    // assert_eq!(response.status(), reqwest::StatusCode::OK);



    println!("响应数据：\n{:#?}", response);

    // Ok(());
}
