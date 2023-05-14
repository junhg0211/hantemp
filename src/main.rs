use reqwest::{get, StatusCode};
use serde_json::Value;

#[tokio::main]
async fn main() {
    let url = "https://api.hangang.msub.kr/";

    let response = get(url).await
        .unwrap();

    if response.status() != StatusCode::OK {
        panic!("Error: {}", response.status());
    }

    let content = response.text().await.unwrap();
    let value: Value = serde_json::from_str(&content)
        .expect("JSON parsing error");

    let temperature = &value["temp"]
        .as_str().unwrap()
        .parse::<f32>().unwrap();
    println!("현재 한강의 온도는 {:?}°C입니다.", temperature);

    let station = &value["station"]
        .as_str().unwrap();
    let time = &value["time"]
        .as_str().unwrap();
    println!("{}에서 {}에 측정함", station, time);
}
