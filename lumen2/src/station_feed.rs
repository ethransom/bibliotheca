use serde::Deserialize;
use serde_xml_rs::from_str;

impl AirQualityData {
    pub fn parse(str: &str) -> std::result::Result<Vec<StationFeedDatapoint>, serde_xml_rs::Error> {
        Ok(from_str::<AirQualityData>(str)?
            .site
            .data
            .into_iter()
            .map(StationFeedDatapoint::from)
            .collect::<Vec<StationFeedDatapoint>>())
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
    data: Vec<ParseData>,
}

#[derive(Debug, Deserialize)]
struct ParseData {
    date: String,  // TODO: parse date?
    ozone: String, // TODO: why can't these parse as floats?
    ozone_8hr_avg: String,
    pm25: String,
    pm25_24hr_avg: String,
    nox: String,
    no2: String,
    temperature: String,
    relative_humidity: String,
    wind_speed: String,
    wind_direction: String,
    co: String,
    solar_radiation: String,
    so2: String,
}

type Result<T> = std::result::Result<T, std::num::ParseFloatError>;

pub struct StationFeedDatapoint {
    date: String, // TODO: parse date?
    ozone: Result<f64>,
    ozone_8hr_avg: Result<f32>,
    pm25: Result<f32>,
    pm25_24hr_avg: Result<f32>,
    nox: Result<f32>,
    no2: Result<f32>,
    temperature: Result<f32>,
    relative_humidity: Result<f32>,
    wind_speed: Result<f32>,
    wind_direction: Result<f32>,
    co: Result<f32>,
    solar_radiation: Result<f32>,
    so2: Result<f32>,
}

impl std::convert::From<ParseData> for StationFeedDatapoint {
    fn from(value: ParseData) -> Self {
        StationFeedDatapoint {
            date: value.date,
            ozone: value.ozone.parse(),
            ozone_8hr_avg: value.ozone_8hr_avg.parse(),
            pm25: value.pm25.parse(),
            pm25_24hr_avg: value.pm25_24hr_avg.parse(),
            nox: value.nox.parse(),
            no2: value.no2.parse(),
            temperature: value.temperature.parse(),
            relative_humidity: value.relative_humidity.parse(),
            wind_speed: value.wind_speed.parse(),
            wind_direction: value.wind_direction.parse(),
            co: value.co.parse(),
            solar_radiation: value.solar_radiation.parse(),
            so2: value.so2.parse(),
        }
    }
}

#[test]
fn test_station_feed_deserialization() {
    let fixture = include_str!("roseParkStationFeedResponse.xml");

    let data = match AirQualityData::parse(fixture) {
        Err(e) => panic!("{e:?}"),
        Ok(d) => d,
    };

    assert_eq!(data.len(), 239);
    assert_eq!(data.first().unwrap().date, "12/04/2022 12:00:00");
}
