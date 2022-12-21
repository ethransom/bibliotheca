use anyhow::{bail, Context, Error, Result};
use strum::IntoEnumIterator;

mod lifx;
mod openweathermap;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("error: {:?}", err);
        std::process::exit(1);
    }
}

async fn try_main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        return set_to_current_aqi()
            .await
            .context("could not set light to current AQI");
    } else if args.len() == 2 {
        match args[1].as_str() {
            "cycle" => cycle_colors().await.context("couldn't cycle colors"),
            arg => bail!("unknown argument: {}", arg),
        }
    } else {
        bail!("too many arguments");
    }
}

async fn set_to_current_aqi() -> Result<(), Error> {
    let openweathermap_token = openweathermap_token()?;
    let lifx_token = lifx_token()?;

    println!("FETCHING AIR QUALITY INDEX (CAQI) FOR SLC UTAH...");

    let caqi = match openweathermap::fetch_caqi(&openweathermap_token).await {
        Ok(caqi) => caqi,
        Err(err) => bail!(err.context("error fetching CAQI")),
    };

    let color = caqi.to_rgb();

    println!("AIR QUALITY IS: {:?}", caqi);

    println!("SETTING LIGHTS TO {color}...");

    if let Err(err) = lifx::put_light_color(&lifx_token, color).await {
        bail!(err.context("error setting light color"));
    }

    println!("LIGHTS SET");

    Ok(())
}

async fn cycle_colors() -> Result<()> {
    let lifx_token = lifx_token()?;

    for caqi in openweathermap::CAQI::iter().cycle() {
        let color = caqi.to_rgb();
        println!("Air Quality {:?} is color {:?}", caqi, color);
        lifx::put_light_color(&lifx_token, color)
            .await
            .context("error setting light color")?;
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    unreachable!();
}

fn openweathermap_token() -> Result<String> {
    std::env::var("OPENWEATHERMAP_TOKEN").context("OPENWEATHERMAP_TOKEN env var must be set")
}

fn lifx_token() -> Result<String> {
    std::env::var("LIFX_TOKEN").context("TOKEN env var must be set")
}
