mod lifx;

const SLC_LAT_LON: (f32, f32) = (40.76388, -111.89228);

fn main() {
    let openweathermap_token =
        std::env::var("OPENWEATHERMAP_TOKEN").expect("OPENWEATHERMAP_TOKEN env var must be set");
    let lifx_token = std::env::var("LIFX_TOKEN").expect("TOKEN env var must be set");

    let caqi: CAQI = openweathermap_fetch_caqi(&openweathermap_token)
        .try_into()
        .expect("couldn't parse caqi");

    let color = caqi.to_rgb();

    println!("AIR QUALITY IN SLC UTAH {:?}", caqi);

    lifx::put_light_color(&lifx_token, color);
}

fn openweathermap_fetch_caqi(api_token: &str) -> u64 {
    let (lat, lon) = SLC_LAT_LON;
    // fetch aqi from openweathermap.org
    let resp = reqwest::blocking::get(format!("http://api.openweathermap.org/data/2.5/air_pollution?lat={lat}&lon={lon}&appid={api_token}"))
        .expect("couldn't fetch aqi");
    println!("{:?}", resp);
    // read body of request
    let body = resp.text().expect("couldn't read body");
    println!("{:?}", body);

    openweathermap_parse_aqi_response(&body)
}

fn openweathermap_parse_aqi_response(body: &str) -> u64 {
    let json: serde_json::Value = serde_json::from_str(body).expect("couldn't parse json");

    json["list"][0]["main"]["aqi"]
        .as_u64()
        .expect("couldn't parse aqi")
}

#[test]
fn test_parse_openweathermap_aqi_response() {
    let body = "{\"coord\":{\"lon\":-111.8923,\"lat\":40.7639},\"list\":[{\"main\":{\"aqi\":3},\"components\":{\"co\":620.84,\"no\":27.27,\"no2\":64.43,\"o3\":0.01,\"so2\":5.54,\"pm2_5\":23.75,\"pm10\":35.68,\"nh3\":5.64},\"dt\":1670307748}]}";

    let aqi = openweathermap_parse_aqi_response(body);

    assert_eq!(aqi, 3);
}

/// Evidently this is the air quality scale used by openweathermap.org
/// https://en.wikipedia.org/wiki/Air_quality_index#CAQI
/// https://openweathermap.org/api/air-pollution
#[derive(Debug, PartialEq, Eq)]
enum CAQI {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

impl TryFrom<u64> for CAQI {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CAQI::VeryLow),
            2 => Ok(CAQI::Low),
            3 => Ok(CAQI::Medium),
            4 => Ok(CAQI::High),
            5 => Ok(CAQI::VeryHigh),
            _ => Err("index too high".to_owned()),
        }
    }
}

impl CAQI {
    fn to_rgb(&self) -> &'static str {
        match self {
            CAQI::VeryLow => "#79bc6a",  // olive
            CAQI::Low => "#bbcf4c",      // lime
            CAQI::Medium => "#eec20b",   // yellow
            CAQI::High => "#f29305",     // orange
            CAQI::VeryHigh => "#e8416f", // red
        }
    }
}
