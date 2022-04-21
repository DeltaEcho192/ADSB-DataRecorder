use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Flight {
    pub hex: String,
    pub flight: String,
    pub lat: f64,
    pub lon: f64,
    pub altitude: i32,
    pub track: i32,
    pub speed: i32,
}

pub fn insert(flight_data:Vec<Flight>)
{
    let opts = OptsBuilder::new()
    .ip_or_hostname(Some("localhost"))
    .user(Some("root"))
    .pass(Some("xxmaster"))
    .db_name(Some("flight_data"));
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
   
    conn.exec_batch(
        r"INSERT INTO test_data (hex, flight, lat,lon,altitude,track,speed)
          VALUES (:hex, :flight, :lat,:lon,:altitude,:track,:speed)",
        flight_data.iter().map(|p| params! {
            "hex" => &p.hex,
            "flight" => &p.flight,
            "lat" => p.lat,
            "lon" => p.lon,
            "altitude" => p.altitude,
            "track" => p.track,
            "speed" => p.speed,
        })
    ).unwrap();
} 
