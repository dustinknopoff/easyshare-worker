use maud::html;
use tracing_subscriber::fmt::{format::Pretty, time::UtcTime};
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use uuid::Uuid;
use worker::*;

use crate::ui::layout;
use crate::ui::file_upload;
mod ui;
mod utils;

const WORKER_URL: &str = "http://localhost:8787";

#[event(start)]
pub fn start() {
    utils::set_panic_hook();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false) // Only partially supported across JavaScript runtimes
        .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    tracing::info!(request=?req, "Handling request");

    let router = Router::new();

    router
        .get("/", |_, _| {
            Response::from_html(
                layout::layout("EasyShare", file_upload::form()).into_string(),
            )
        })
        .get_async("/obj/:key", |_req, ctx| async move {
            let Some(id) = ctx.param("key") else {
                return Response::error("key required", 404);
            };

            let bucket = ctx.bucket("EASYSHARE_BUCKET")?;

            let Some(object) = bucket.get(id).execute().await? else {
                return Response::error("No object found", 404);
            };

            let r2_headers = object.http_metadata();
            let headers = Headers::new();
            let mut headers = write_http_headers(headers, r2_headers)?;
            headers.set("etag", &object.http_etag())?;

            Ok(Response::from_stream(object.body().unwrap().stream()?)?
            .with_headers(headers)
            .with_status(200))
            
        })
        .get_async("/share/:id", |_req, ctx| async move {
            let Some(id) = ctx.param("id") else {
                return Response::error("ID required", 404);
            };

            let bucket = ctx.bucket("EASYSHARE_BUCKET")?;

            let folder: Vec<String> = bucket
                .list()
                .prefix(id)
                .execute()
                .await?
                .objects()
                .iter()
                .map(|object| format!("{WORKER_URL}/obj/{}", object.key()))
                .collect();

            Response::from_html(
                layout::layout(
                    "EasyShare",
                    html!(
                        @if folder.is_empty() {
                            p {
                                "No share found or is expired."
                            }
                        }
                        ul {
                            @for object in folder {
                                li {
                                    (object)
                                }
                            }
                        }
                    ),
                )
                .into_string(),
            )
        })
        .run(req, env)
        .await
}

fn write_http_headers(mut headers: Headers, r2_metadata: HttpMetadata) -> Result<Headers> {
    if let Some(content_type) = r2_metadata.content_type {
        headers.set("content-type", &content_type)?;
    }
    if let Some(content_language) = r2_metadata.content_language {
        headers.set("content-language", &content_language)?;
    }
    if let Some(content_disposition) = r2_metadata.content_disposition {
        headers.set("content-disposition", &content_disposition)?;
    }
    if let Some(content_encoding) = r2_metadata.content_encoding {
        headers.set("content-encoding", &content_encoding)?;
    }
    if let Some(cache_control) = r2_metadata.cache_control {
        headers.set("cache-control", &cache_control)?;
    }
    if let Some(cache_expiry) = r2_metadata.cache_expiry {
        headers.set("cache-expiry", &format!("max-age={}", cache_expiry.as_millis()))?;
    }
    Ok(headers)
}