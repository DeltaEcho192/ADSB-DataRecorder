use serde::Deserialize;
use reqwest::Error;
mod database;

fn main()
{
    get_data();
}



#[tokio::main]
async fn get_data() -> Result<(), Error>
{
    let request_url = format!("http://192.168.1.119:8080/data.json");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let flight_data: Vec<database::Flight> = response.json().await?;
    if(flight_data.len() > 0)
    {
        println!("{:?}", flight_data[0].flight);
        database::insert(flight_data);
    }
    else{
        println!("{}","No Planes in the air");
    }
    
    Ok(())
}