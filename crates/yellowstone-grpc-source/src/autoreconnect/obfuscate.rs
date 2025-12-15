use std::borrow::Cow;
use url::Url;

/// obfuscate urls with api token like http://mango.rpcpool.com/a991fba00fagbad
pub fn url_obfuscate_api_token(url: &str) -> Cow<'_, str> {
    if let Ok(mut parsed) = Url::parse(url) {
        if parsed.path() == "/" {
            Cow::Borrowed(url)
        } else {
            parsed.set_path("omitted-secret");
            Cow::Owned(parsed.to_string())
        }
    } else {
        Cow::Borrowed(url)
    }
}

#[test]
fn test_obfuscate_path() {
    let url_mango = "http://mango.rpcpool.com/121sdfsdf21";
    let obfuscated = url_obfuscate_api_token(url_mango);
    assert_eq!(obfuscated, "http://mango.rpcpool.com/omitted-secret");
}

#[test]
fn test_obfuscate_nopath() {
    let url_localhost = "http://127.0.0.1";
    let obfuscated = url_obfuscate_api_token(url_localhost);
    assert_eq!(obfuscated, "http://127.0.0.1");
}

#[test]
fn test_obfuscate_invalid() {
    let url_localhost = "::::invalid";
    let obfuscated = url_obfuscate_api_token(url_localhost);
    assert_eq!(obfuscated, "::::invalid");
}
