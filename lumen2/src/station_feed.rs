use serde::Deserialize;
use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Deserialize)]
struct AirQualityData {
    state: String,
    site: Site,
}

#[derive(Debug, Deserialize)]
struct Site {
    name: String,
    // data: Option<Vec<Data>>,
}

#[derive(Debug, Deserialize)]
struct Data {
    date: String, // TODO: parse?
    ozone: f32,
    ozone_8hr_avg: f32,
    pm25: f32,
    pm25_24hr_avg: f32,
    nox: f32,
    no2: f32,
    temperature: f32,
    relative_humidity: i8,
    wind_speed: f32,
    wind_direction: i32,
    co: f32,
    solar_radiation: f32,
    so2: f32,
}

#[test]
fn test_station_feed_deserialization() {
    let fixture = include_str!("roseParkStationFeedResponse.xml");

    let data: AirQualityData = match from_str(fixture) {
        Err(e) => panic!("{e:?}"),
        Ok(d) => d,
    };

    assert_eq!(data.state, "Utah");
    assert_eq!(data.site.name, "Rose Park");
}
