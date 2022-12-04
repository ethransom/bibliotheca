mod station_feed;

use station_feed::StationFeed;

fn main() {
    let feed = StationFeed::try_from(include_str!("roseParkStationFeedResponse.xml"))
        .expect("couldn't parse feed");

    let ozone_data: Vec<(String, f64)> = feed
        .data
        .into_iter()
        .flat_map(|data| data.ozone.map(|o| (data.date, o)))
        .collect();

    println!("{ozone_data:?}")
}
