use std::env;
use std::str::FromStr;

fn main() {
    // OpenSSL >= 1.1.0 can be initialized concurrently and is initialized correctly by libcurl.
    // <= 1.0.2 need locking callbacks, which are provided by openssl_sys::init().
    match env::var("DEP_OPENSSL_VERSION") {
        Ok(ver) => {
            let ver = u32::from_str(&ver).unwrap();
            if ver < 110 {
                println!("cargo:rustc-cfg=need_openssl_init");
            }
        }
        Err(_) => {},
    };

    // The system libcurl should have the default certificate paths configured.
    // The system OpenSSL should know where its certificates are.
    if env::var_os("DEP_CURL_STATIC").is_some() && env::var_os("DEP_OPENSSL_VENDORED").is_some() {
        println!("cargo:rustc-cfg=need_openssl_probe");
    }
}
