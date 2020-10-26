use std::collections::HashMap;

use url::Url;

pub(crate) fn parse_url(url: &str) -> Result<(String, String, String), anyhow::Error> {
    let parsed = Url::parse(url)?;
    if parsed.scheme() != "hzip" {
        return Err(anyhow!("Protocol not found"));
    }

    let api_host = parsed.host().unwrap().to_string()
        + ":".to_string().as_str()
        + parsed.port().unwrap().to_string().as_str();

    let hash_query: HashMap<_, _> = parsed.query_pairs().into_owned().collect();

    Ok((
        api_host,
        hash_query["password"].clone(),
        hash_query["archive"].clone(),
    ))
}
