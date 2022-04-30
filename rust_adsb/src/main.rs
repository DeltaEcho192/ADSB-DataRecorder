
use reqwest::Error;
use std::time::Duration;
use std::thread::sleep;
mod database;

fn main()
{
    loop
    {
        let callback = get_data();
        match callback {
            Ok(f) =>
            {
                println!("Successful Request");
            },
            Err(e)=>{
                println!("Error has occured: {}",e);
            }   
        }
        sleep(Duration::from_millis(5000));
    }
    
    
}



#[tokio::main]
async fn get_data() -> Result<(), Error>
{
    
    let request_url = format!("http://192.168.1.119:8080/data.json");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let flight_data: Vec<database::Flight> = response.json().await?;
    if flight_data.len() > 0 
    {
        println!("{}", "Aircraft Being inserted");
        database::insert(flight_data);
    }
    else{
        println!("{}","No Planes in the air");
    }
    Ok(())
}