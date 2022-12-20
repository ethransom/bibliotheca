mod lifx;
mod openweathermap;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let openweathermap_token =
        std::env::var("OPENWEATHERMAP_TOKEN").expect("OPENWEATHERMAP_TOKEN env var must be set");
    let lifx_token = std::env::var("LIFX_TOKEN").expect("TOKEN env var must be set");

    let caqi: openweathermap::CAQI = openweathermap::fetch_caqi(&openweathermap_token)
        .await
        .try_into()
        .expect("couldn't parse caqi");

    let color = caqi.to_rgb();

    println!("AIR QUALITY IN SLC UTAH {:?}", caqi);

    lifx::put_light_color(&lifx_token, color).await;

    println!("LIGHTS SET TO {color}");
}
