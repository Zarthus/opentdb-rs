/// Example Sentence (Non Encoded): "Don't forget that π = 3.14 & doesn't equal 3."
pub enum Encoding {
    /// Default Encoding (HTML Codes) - example output:
    ///   Don&‌#039;t forget that &‌pi; = 3.14 &‌amp; doesn&‌#039;t equal 3.
    DefaultEncoding,
    /// Legacy URL Encoding - example output:
    ///   Don%27t+forget+that+%CF%80+%3D+3.14+%26+doesn%27t+equal+3.
    Legacy,
    /// URL Encoding (RFC 3986, https://www.ietf.org/rfc/rfc3986.txt) - example output:
    ///   Don%27t%20forget%20that%20%CF%80%20%3D%203.14%20%26%20doesn%27t%20equal%203.
    Url,
    /// Base64 Encoding - example output:
    ///   RG9uJ3QgZm9yZ2V0IHRoYXQgz4AgPSAzLjE0ICYgZG9lc24ndCBlcXVhbCAzLg==
    Base64
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::DefaultEncoding
    }
}

impl Encoding {
    pub fn value(&self) -> &str {
        match *self {
            Encoding::DefaultEncoding => { "default" },
            Encoding::Legacy => { "legacy" },
            Encoding::Url => { "url3986" },
            Encoding::Base64 => { "base64" },
        }
    }
}
