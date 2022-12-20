const LIGHT_ID: &str = "d073d559d839";

pub async fn put_light_color(api_token: &str, color: &str) -> anyhow::Result<()> {
    let resp = reqwest::Client::new()
        .put(format!(
            "https://api.lifx.com/v1/lights/id:{LIGHT_ID}/state",
        ))
        .bearer_auth(api_token)
        .form(&[("power", "on"), ("color", color)])
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("request failed: {:?}", resp);
    }

    Ok(())
}
