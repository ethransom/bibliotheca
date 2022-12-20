mod lifx;
mod openweathermap;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let openweathermap_token =
        std::env::var("OPENWEATHERMAP_TOKEN").expect("OPENWEATHERMAP_TOKEN env var must be set");
    let lifx_token = std::env::var("LIFX_TOKEN").expect("TOKEN env var must be set");

    println!("FETCHING AIR QUALITY INDEX (CAQI) FOR SLC UTAH...");

    let caqi = match openweathermap::fetch_caqi(&openweathermap_token).await {
        Ok(caqi) => caqi,
        Err(err) => {
            eprintln!("error fetching CAQI: {:?}", err);
            return;
        }
    };

    let color = caqi.to_rgb();

    println!("AIR QUALITY IS: {:?}", caqi);

    println!("SETTING LIGHTS TO {color}...");

    if let Err(err) = lifx::put_light_color(&lifx_token, color).await {
        eprintln!("error setting light color: {:?}", err);
        return;
    }

    println!("LIGHTS SET");
}
