use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Flight {
    hex: String,
    flight: String,
    lat: f64,
    lon: f64,
    altitude: i32,
    track: i32,
    speed: i32
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("http://192.168.1.119:8080/data.json");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let flight_data: Vec<Flight> = response.json().await?;
    println!("{:?}", flight_data[1].flight);
    Ok(())
}