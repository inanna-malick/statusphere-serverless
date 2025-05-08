use atrium_identity::handle::DnsTxtResolver;
use reqwest_wasm::header::ACCEPT;
use serde::{Deserialize, Serialize};

pub struct DnsOverHttps(reqwest_wasm::Client);

impl DnsOverHttps {
    pub fn new() -> Self {
        Self(reqwest_wasm::Client::new())
    }
}

impl DnsTxtResolver for DnsOverHttps {
    async fn resolve(
        &self,
        query: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let url = format!("https://one.one.one.one/dns-query?name={query}&type=TXT");

        let resp = self
            .0
            .get(url)
            .header(ACCEPT, "application/dns-json")
            .send()
            .await?;

        let resp = resp.json::<PartialResp>().await?;

        Ok(resp
            .answer
            .into_iter()
            .map(|x| {
                // TXT-records are (*sometimes*) quoted, but downstream
                // does not handle quotes well
                x.data.trim_matches('"').to_string()
            })
            .collect())
    }
}

// not modelling errors or misc metadata, just the happy path. PRs are welcome
#[derive(Serialize, Deserialize, Clone, Debug)]
struct PartialResp {
    #[serde(rename = "Answer")]
    answer: Vec<PartialRespElem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PartialRespElem {
    data: String,
}
