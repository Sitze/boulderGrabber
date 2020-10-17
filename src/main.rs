use influxdb::{Client, Query, Timestamp};
use influxdb::InfluxDbWriteable;
use chrono::{DateTime, Utc, Local};
use scraper::{Html, Selector};

#[derive(InfluxDbWriteable)]
struct VisitorCount {
    time: DateTime<Local>,
    visitors: i32,
}

fn get_amount_of_visitors(html_source: &str) -> Result<String, String>{
    // Parse the document
    let full_doc_body = Html::parse_document(&html_source);

    let counter_class = Selector::parse(".actcounter-content").unwrap();

    let mut inner_doc_body: String = "".to_string();

    // Extract inner html
    for number_of_visitors in full_doc_body.select(&counter_class){
        inner_doc_body = number_of_visitors.inner_html();
        //DEBUG println!("First loop: {}", inner_doc_body);
    }

    // Selector for the counter value
    let counter_value_span = Selector::parse("span").unwrap();
    let mut current_visitors: String = "".to_string();

    let fragment = Html::parse_fragment(&inner_doc_body);

    for visitors in fragment.select(&counter_value_span){
        current_visitors = visitors.inner_html();
        //DEBUG println!("Second loop: {}", current_visitors);
    }

    if current_visitors == "" {
        return Err("Could not get visitor number".to_string());
    }

    // Return the visitor number
    Ok(current_visitors)
}

#[tokio::main]
async fn main() {
    // Link to grab the information from
    let mut request = reqwest::get("https://www.boulderado.de/boulderadoweb/gym-clientcounter/index.php?mode=get&token=eyJhbGciOiJIUzI1NiIsICJ0eXAiOiJKV1QifQ.eyJjdXN0b21lciI6IkRBVkRlZ2dlbmRvcmYifQ.yzloAfvmNFb-HqzYFGoMJc4vsDUE58cV8409hn0Thf4").await.unwrap().text().await.unwrap();
    
    // Get current visitor count as string
    let current_visitors_string = get_amount_of_visitors(&request).unwrap();

    // Convert the visitor count to i32
    let current_visitors : i32 = current_visitors_string.parse().unwrap();

    // Connect to db `test` on `http://localhost:8086`
    let client = Client::new("http://localhost:8086", "test_measurement_db");

    // Get timestamp
    let localTime = chrono::offset::Local::now() + chrono::Duration::hours(0);

    // Let's write some data into a measurement called `visitors`
    let mut visitor_reading = VisitorCount {
        time: localTime,
       visitors: current_visitors,
    };

    // Write visitor count into 'visitors' measurement table
    let write_result = client
        .query(&visitor_reading.into_query("visitors"))
        .await;
    assert!(write_result.is_ok(), "Write result was not okay");

    // Let's see if the data we wrote is there
    //let read_query = Query::raw_read_query("SELECT * FROM weather");

    //let read_result = client.query(&read_query).await;
    //assert!(read_result.is_ok(), "Read result was not ok");
    //println!("{}", read_result.unwrap());
}
