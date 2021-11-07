use cfg_if::cfg_if;
use telegram_bot_raw::{HttpRequest, Request, SendMessage, Update};
use worker::wasm_bindgen::JsValue;
use worker::Request as WRequest;
use worker::*;

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

const BOT_TOKEN: &str = "2107975294:AAG6ycfnXr34JZSKxwUm3lLWQ7oJDYdocQU";

pub fn to_workers_request(one: HttpRequest) -> Result<WRequest> {
    match one.body {
        telegram_bot_raw::Body::Json(j) => {
            let mut hds = Headers::new();
            hds.set("Content-Type", "application/json").unwrap();
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
            let try_request = WRequest::new_with_init(&one.url.url(BOT_TOKEN), &init).unwrap();
            Ok(try_request)
        }
        _ => Err(Error::RustError("Not implemented yet".to_string())),
    }
}

pub async fn send_request(body: HttpRequest) -> Result<()> {
    let req = to_workers_request(body)?;
    let resp = Fetch::Request(req)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    console_log!("{}", resp);
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
