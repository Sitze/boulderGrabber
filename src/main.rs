use influxdb::{Client, Query, Timestamp};
use influxdb::InfluxDbWriteable;
use chrono::{DateTime, Utc, Local};

#[derive(InfluxDbWriteable)]
struct WeatherReading {
    time: DateTime<Local>,
    //time: DateTime<Local>,
    humidity: i32,
    #[tag] wind_direction: String,
}

#[tokio::main]
async fn main() {
    // Connect to db `test` on `http://localhost:8086`
    let client = Client::new("http://localhost:8086", "test_measurement_db");

    let localTime = chrono::offset::Local::now() + chrono::Duration::hours(2);

    // Let's write some data into a measurement called `weather`
    let weather_reading = WeatherReading {
        time: localTime,
        //time: chrono::offset::Local::now(),
        //time: chrono::offset::Utc::now(),
        humidity: 32,
        wind_direction: String::from("north"),
    };

    let write_result = client
        .query(&weather_reading.into_query("weather"))
        .await;
    assert!(write_result.is_ok(), "Write result was not okay");

    // Let's see if the data we wrote is there
    let read_query = Query::raw_read_query("SELECT * FROM weather");

    let read_result = client.query(&read_query).await;
    assert!(read_result.is_ok(), "Read result was not ok");
    println!("{}", read_result.unwrap());
}