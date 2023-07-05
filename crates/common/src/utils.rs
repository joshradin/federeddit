use std::fmt::Display;
use base64::Engine;

pub fn encode_base64<T : Display>(value: &T) -> String {
    base64::engine::general_purpose::STANDARD.encode(value.to_string())
}