use serde::Deserialize;
use serde_xml_rs::from_str;

impl AirQualityData {
    pub fn parse(str: &str) -> Result<AirQualityData, serde_xml_rs::Error> {
        from_str(str)
    }
}

#[derive(Debug, Deserialize)]
struct AirQualityData {
    state: String,
    site: Site,
}

#[derive(Debug, Deserialize)]
struct Site {
    name: String,
    data: Vec<Data>,
}

#[derive(Debug, Deserialize)]
struct Data {
    date: String, // TODO: parse date?
    // ozone: f64, // TODO: why can't these parse as floats?
    // ozone_8hr_avg: f32,
    // pm25: f32,
    // pm25_24hr_avg: f32,
    // nox: f32,
    // no2: f32,
    // temperature: f32,
    relative_humidity: i8,
    // wind_speed: f32,
    // wind_direction: i32,
    // co: f32,
    // solar_radiation: f32,
    // so2: f32,
}

#[test]
fn test_station_feed_deserialization() {
    let fixture = include_str!("roseParkStationFeedResponse.xml");

    let data = match AirQualityData::parse(fixture) {
        Err(e) => panic!("{e:?}"),
        Ok(d) => d,
    };

    assert_eq!(data.state, "Utah");
    assert_eq!(data.site.name, "Rose Park");

    assert_eq!(data.site.data.len(), 239);
    assert_eq!(data.site.data.first().unwrap().date, "12/04/2022 12:00:00");
}
