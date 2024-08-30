use reqwest::{header::{self, HeaderName}, Error, StatusCode};
use serde_json::json;
use base64::encode;
use serde::Deserialize;
use urlencoding::encode as urlencode;

#[derive(Debug,Deserialize)]
//responsebody
pub struct ApiResponse<T> {
    pub is_success: bool,
    pub data: T,
    pub decoded_data:Option<String>
}

// Main struct
pub struct RusselClient<'a> {
    connection_string: &'a str,  // http://127.0.0.1:6022 Or your serverIP:Port if you want request from out of server
    username: &'a str,           // admin
    password: &'a str            // 123456
}

impl<'a> RusselClient<'a> {
    pub fn new(connection_string: &'a str, username: &'a str, password: &'a str) -> Self {
        RusselClient {
            connection_string,
            username,
            password,
        }
    }

    pub async fn set_value_async(&self, _cluster: &str, _key: &str, _value: &str) -> Result<bool, Error> {
        let url = format!("{}/api/set", self.connection_string);
        let request_body = json!({
            "cluster": _cluster,
            "key": _key,
            "value": _value
        });
        let client = reqwest::Client::new();
        let authorization_header = make_auth(&self.username,&self.password);
        let response = client
            .post(&url)
            .json(&request_body)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, authorization_header)
            .send()
            .await?;
    
        if response.status().is_success() {
            let body = response.text().await?;
            let api_response:ApiResponse<String> = serde_json::from_str(&body).unwrap();
            if api_response.is_success {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
    }

    pub async fn get_raw_value_async(&self, _cluster: &str, _key: &str) -> Result<Option<Vec<u8>>, Error> {
        let encoded_cluster = urlencode(_cluster).into_owned();
        let encoded_key = urlencode(_key).into_owned();
        let url = format!("{}/api/get/{}/{}",self.connection_string,encoded_cluster,encoded_key);
        let authorization_header = make_auth(&self.username,&self.password);
        let client = reqwest::Client::new();
        let response = client.get(url)
        .header(header::AUTHORIZATION, authorization_header)
        .send().await?;
        if response.status().is_success(){
            let body = response.text().await?;
            let api_response:ApiResponse<Vec<u8>> = serde_json::from_str(&body).unwrap();
            return Ok(Option::Some(api_response.data));
        }else {
            return Ok(None);
        }
    }

    pub async fn get_decoded_value_async(&self, _cluster: &str, _key: &str) -> Result<Option<String>, Error> {
        let encoded_cluster = urlencode(_cluster).into_owned();
        let encoded_key = urlencode(_key).into_owned();
        let url = format!("{}/api/get/{}/{}",self.connection_string,encoded_cluster,encoded_key);
        let authorization_header = make_auth(&self.username,&self.password);
        let client = reqwest::Client::new();
        let response = client.get(url)
        .header(header::AUTHORIZATION, authorization_header)
        .send().await?;
        if response.status().is_success(){
            let body = response.text().await?;
            let api_response:ApiResponse<Vec<u8>> = serde_json::from_str(&body).unwrap();
            return Ok(Option::Some(decode_value(api_response.data)));
        }else {
            return Ok(None);
        }
    }

    pub async fn set_cluster_async(&self, _cluster: &str) -> Result<bool, Error> {

        let encoded_cluster = urlencode(_cluster).into_owned();
        let url = format!("{}/api/set_cluster/{}",self.connection_string,encoded_cluster);
        let client = reqwest::Client::new();
        let authorization_header = make_auth(&self.username,&self.password);
        let response = client
            .post(&url)
            .header(header::AUTHORIZATION, authorization_header)
            .send()
            .await?;
    
        if response.status().is_success() {
          
            let body = response.text().await?;
            let api_response:ApiResponse<String> = serde_json::from_str(&body).unwrap();
            if api_response.is_success {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
    }

    pub async fn delete_key_async(&self, _cluster: &str, _key: &str) -> Result<bool, Error> {

        let encoded_cluster = urlencode(_cluster).into_owned();
        let encoded_key = urlencode(_key).into_owned();
        let url = format!("{}/api/delete/{}/{}",self.connection_string,encoded_cluster,encoded_key);
        let client = reqwest::Client::new();
        let authorization_header = make_auth(&self.username,&self.password);
        let response = client
            .delete(&url)
            .header(header::AUTHORIZATION, authorization_header)
            .send()
            .await?;
    
        if response.status().is_success() {
          
            let body = response.text().await?;
            let api_response:ApiResponse<String> = serde_json::from_str(&body).unwrap();
            if api_response.is_success {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
    }
}


//auth maker
fn make_auth(username:&str,password:&str) -> String{
    let creds = format!("{}:{}", username, password);
    let byte_cred = creds.as_bytes();
    let main_cred = encode(byte_cred);
    format!("{}", main_cred)
}

pub fn decode_value(value:Vec<u8>) -> String{
    String::from_utf8_lossy(&value).to_string()
}