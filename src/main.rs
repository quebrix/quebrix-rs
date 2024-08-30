use russelclient::russel::RusselClient;

mod russelclient;

#[tokio::main]
async fn main() {
    //initial app
    let russel = RusselClient::new("http://127.0.0.1:6022", "admin", "123456");
    //set value
    let _result = russel.set_value_async("dev", "test", "test value").await.unwrap();
    println!("set result: {:?}",_result);
    //get value
    let get_result = russel.get_raw_value_async("dev", "test").await.unwrap().unwrap();
    let get_decode_result = russel.get_decoded_value_async("dev", "test").await.unwrap().unwrap();
    println!("raw value: {:?}",get_result);
    println!("decode value: {:?}",get_decode_result);
    //delete value
    let delete_result = russel.delete_key_async("dev", "test").await.unwrap();
    println!("delete result: {:?}",delete_result);
    //set cluster
    let set_cluster_result = russel.set_cluster_async("prod").await.unwrap();
    println!("set cluster result: {:?}",set_cluster_result);
}