use anyhow::bail;
use worker::*;

mod lifx;
mod openweathermap;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(scheduled)]
pub async fn main(_worker: ScheduledEvent, _env: worker::Env, _ctx: worker::ScheduleContext) {
    try_scheduled_main(_env)
        .await
        .expect("scheduled main failed");
}

async fn try_scheduled_main(env: worker::Env) -> anyhow::Result<(), anyhow::Error> {
    let lifx_token = if let Ok(token) = env.var("LIFX_TOKEN") {
        token.to_string()
    } else {
        bail!("LIFX_TOKEN was not set");
    };
    let openweathermap_token = if let Ok(token) = env.var("OPENWEATHERMAP_TOKEN") {
        token.to_string()
    } else {
        bail!("OPENWEATHERMAP_TOKEN was not set");
    };
    console_log!("FETCHING AIR QUALITY INDEX (CAQI) FOR SLC UTAH...");
    let caqi = openweathermap::fetch_caqi(&openweathermap_token).await?;
    console_log!("AIR QUALITY IS: {:?}", caqi);
    let color = caqi.to_rgb();
    console_log!("SETTING LIGHTS TO {color}...");
    lifx::put_light_color(&lifx_token, color).await?;
    console_log!("LIGHTS SET");
    Ok(())
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .post_async("/light/:color", |_req, ctx| async move {
            let color = if let Some(color) = ctx.param("color") {
                color
            } else {
                return Response::error("Bad Request", 400);
            };

            let api_token = if let Ok(token) = ctx.env.var("LIFX_TOKEN") {
                token.to_string()
            } else {
                return Response::error("Internal Server Error", 500);
            };

            if let Err(err) = lifx::put_light_color(&api_token, color).await {
                return Response::error(err.to_string(), 500);
            }

            Response::from_html(format!("Light set to <em>{color}</em>"))
        })
        .get_async("/aqi", |_, ctx| async move {
            let api_token = if let Ok(token) = ctx.env.var("OPENWEATHERMAP_TOKEN") {
                token.to_string()
            } else {
                return Response::error("Internal Server Error", 500);
            };

            match openweathermap::fetch_caqi(&api_token).await {
                Ok(caqi) => Response::from_html(format!("Air quality is: <em>{caqi:?}</em>")),
                Err(err) => Response::error(err.to_string(), 500),
            }
        })
        .get_async("/refresh", |_, ctx| async move {
            match try_scheduled_main(ctx.env).await {
                Ok(()) => Response::from_html("Set light"),
                Err(err) => Response::error(err.to_string(), 500),
            }
        })
        .run(req, env)
        .await
}
