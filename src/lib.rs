#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![doc(html_root_url = "https://vtavernier.github.io/libsox-sys/")]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_init() {
        unsafe {
            sox_format_init();
            sox_format_quit();
        }
    }
}
