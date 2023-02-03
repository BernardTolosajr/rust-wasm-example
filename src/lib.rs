use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use getrandom::getrandom;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HelloWorld) });
}}

struct  HelloWorld;
impl Context for HelloWorld {}

struct HttpBody;
impl Context for HttpBody {}

impl RootContext for HelloWorld {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpBody))
    }

    fn on_vm_start(&mut self, _: usize) -> bool {
        info!("Hello, World!");
        self.set_tick_period(Duration::from_secs(5));
        true
    }

    fn on_tick(&mut self) {
        cfg_if! {
            if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
                let now: DateTime<Utc> = self.get_current_time().into();
                info!("It's {}, there is no lucky number.", now);
            } else {
                let now: DateTime<Utc> = Utc::now();
                let mut buf = [0u8; 1];
                getrandom(&mut buf).unwrap();
                info!("It's {}, your lucky number is {}.", now, buf[0]);
            }
        }
    }
}

impl HttpContext for HttpBody {
    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        // If there is a Content-Length header and we change the length of
        // the body later, then clients will break. So remove it.
        // We must do this here, because once we exit this function we
        // can no longer modify the response headers.
        self.set_http_response_header("content-length", None);
        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        if !end_of_stream {
            // Wait -- we'll be called again when the complete body is buffered
            // at the host side.
            return Action::Pause;
        }

        // Replace the message body if it contains the text "secret".
        // Since we returned "Pause" previuously, this will return the whole body.
        if let Some(body_bytes) = self.get_http_response_body(0, body_size) {
            let body_str = String::from_utf8(body_bytes).unwrap();
            if body_str.contains("secret") {
                let new_body = format!("Original message body ({body_size} bytes) redacted.\n");
                self.set_http_response_body(0, body_size, &new_body.into_bytes());
            }
        }
        Action::Continue
    }
}
