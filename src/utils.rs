//! Utility functions.
use cfg_if::cfg_if;
use telegram_bot_raw::HttpRequest;
use worker::wasm_bindgen::JsValue;
use worker::{Request as WRequest, *};

use crate::constants::BOT_TOKEN;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

/// Convert [`HttpRequest`](telegram_bot_raw::requests::_base::http::
/// HttpRequest) to worker's [`Request`](worker::request).
pub fn to_workers_request(one: HttpRequest) -> Result<WRequest> {
    let bot_token = BOT_TOKEN.to_string();
    match one.body {
        telegram_bot_raw::Body::Json(j) => {
            let mut hds = Headers::new();
            hds.set("Content-Type", "application/json")?;
            let init = RequestInit {
                method: match one.method {
                    telegram_bot_raw::Method::Get => Method::Get,
                    telegram_bot_raw::Method::Post => Method::Post,
                },
                body: Some(JsValue::from_str(&j)),
                cf: CfProperties::default(),
                headers: hds,
                redirect: RequestRedirect::Follow,
            };
            let try_request = WRequest::new_with_init(&one.url.url(bot_token.trim()), &init)?;
            Ok(try_request)
        }
        // FormData: blocked by cloudflare/workers-rs#79
        _ => Err(Error::RustError("Not implemented yet".to_string())),
    }
}

/// Send a request finalized with `telegram_bot_raw`'s
/// [`HttpRequest`](telegram_bot_raw::requests::_base::HttpRequest).
pub async fn send_request(body: HttpRequest) -> Result<()> {
    let req = to_workers_request(body)?;
    Fetch::Request(req).send().await?.text().await?;
    Ok(())
}

/// Send a raw request finalized with [`wasm_bindgen::JsValue`].
pub async fn send_raw_request(url: &str, body: JsValue) -> Result<()> {
    let mut hds = Headers::new();
    hds.set("Content-Type", "application/json")?;
    let init = RequestInit {
        method: Method::Post,
        body: Some(body),
        cf: CfProperties::default(),
        headers: hds,
        redirect: RequestRedirect::Follow,
    };

    let req = WRequest::new_with_init(url, &init)?;
    Fetch::Request(req).send().await?.text().await?;

    Ok(())
}

/// Escapes Markdown text. See [`teloxide::utils::markdown::escape`].
pub fn escape(s: &str) -> String {
    s.replace("_", r"\_")
        .replace("~", r"\~")
        .replace("`", r"\`")
        .replace(">", r"\>")
        .replace("#", r"\#")
        .replace("+", r"\+")
        .replace("-", r"\-")
        .replace("=", r"\=")
        .replace("|", r"\|")
        .replace("{", r"\{")
        .replace("}", r"\}")
        .replace(".", r"\.")
        .replace("!", r"\!")
}
