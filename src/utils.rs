use cfg_if::cfg_if;
use telegram_bot_raw::{HttpRequest};
use worker::wasm_bindgen::JsValue;
use worker::Request as WRequest;
use worker::*;

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

pub fn to_workers_request(one: HttpRequest) -> Result<WRequest> {
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
            let try_request = WRequest::new_with_init(&one.url.url(BOT_TOKEN), &init)?;
            Ok(try_request)
        }
        // FormData: blocked by cloudflare/workers-rs#79
        _ => Err(Error::RustError("Not implemented yet".to_string())),
    }
}

pub async fn send_request(body: HttpRequest) -> Result<()> {
    let req = to_workers_request(body)?;
    let _resp = Fetch::Request(req).send().await?.text().await?;
    Ok(())
}

pub async fn send_raw_request(url: &str, body: JsValue) -> Result<()> {
    console_log!("{:?}", body);
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

    let resp = Fetch::Request(req).send().await?.text().await?;
    console_log!("{}", resp,);
    Ok(())
}

// https://docs.rs/teloxide/0.5.3/src/teloxide/utils/markdown.rs.html#91-110
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
