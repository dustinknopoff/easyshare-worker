use maud::html;
use tracing_subscriber::fmt::{format::Pretty, time::UtcTime};
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use uuid::Uuid;
use worker::*;

use crate::ui::file_upload;
use crate::ui::layout;
mod ui;
mod utils;


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
            Response::from_html(layout::layout("EasyShare", file_upload::form()).into_string())
        })
        .post_async("/upload", |mut req, ctx| async move {
            let form_data = req.form_data().await?;
            let bucket = ctx.bucket("EASYSHARE_BUCKET")?;
            let worker_url = ctx.var("WORKER_URL")?.to_string();

            let Some(files) = form_data.get_all("files") else {
                return Response::error("No files uploaded", 404);
            };

            let prefix = Uuid::new_v4();

            for file in files {
                if let FormEntry::File(file) = file {
                    bucket
                        .put(format!("{}/{}", prefix, file.name()), file.bytes().await?)
                        .execute()
                        .await?;
                };
            }

            let url = format!("{worker_url}/share/{prefix}");

            Response::from_html(html!(
                div id="success-response" {
                    p { "Your files have been uploaded. Click the link below to view what you'll be sharing."}
                    a href={(url)} {
                        "View Files"
                    }
                    p { "Or share the URL "
                        a href={(url)} {
                            (url) 
                        }
                    }
                    (layout::shortcut())
                }
            ).into_string())
        })
        .get_async("/obj/:key/:file_name", |_req, ctx| async move {
            let Some(id) = ctx.param("key") else {
                return Response::error("key required", 404);
            };
            let Some(file_name) = ctx.param("file_name") else {
                return Response::error("file_name required", 404);
            };

            let bucket = ctx.bucket("EASYSHARE_BUCKET")?;

            let Some(object) = bucket.get(format!("{}/{}", urlencoding::decode(id).unwrap(), urlencoding::decode(file_name).unwrap())).execute().await? else {
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
        .get_async("/share/:id", |req, ctx| async move {
            let Some(id) = ctx.param("id") else {
                return Response::error("ID required", 404);
            };

            let bucket = ctx.bucket("EASYSHARE_BUCKET")?;
            let worker_url = ctx.var("WORKER_URL")?.to_string();

            let folder: Vec<Object> = bucket
                .list()
                .prefix(id)
                .execute()
                .await?
                .objects();

            let html = Response::from_html(
                layout::layout(
                    "EasyShare",
                    html!(
                        @if folder.is_empty() {
                            p {
                                "No share found or is expired."
                            }
                        }
                        ul {
                            @for object in folder.iter() {
                                li {
                                    a download href={(worker_url) "/obj/" (object.key())} {
                                        "Click to download " (object.key())
                                    }
                                }
                            }
                        }
                        (layout::shortcut())
                    ),
                )
                .into_string(),
            );

            match req.headers().get("Accept") {
                Ok(Some(accept_header)) if accept_header.contains("application/json") => {
                    let response: Vec<String> = folder.iter().map(|object| format!("{worker_url}/obj/{}", object.key())).collect();
                    Response::from_json(&response)
                },
                _ => html, 
            }
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
        headers.set(
            "cache-expiry",
            &format!("max-age={}", cache_expiry.as_millis()),
        )?;
    }
    Ok(headers)
}
