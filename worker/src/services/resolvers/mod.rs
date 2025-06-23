use std::sync::Arc;

use atrium_api::types::string::Did;
use atrium_common::resolver::Resolver as _;
use atrium_identity::{
    did::{CommonDidResolver, CommonDidResolverConfig, DEFAULT_PLC_DIRECTORY_URL},
    handle::{AtprotoHandleResolver, AtprotoHandleResolverConfig},
};
use atrium_oauth::DefaultHttpClient;
use dns_over_http::DnsOverHttps;
use worker::{console_log, kv::KvStore};

use crate::storage::kv::cached_resolver::KvStoreCachedResolver;

mod dns_over_http;

pub fn did_resolver(http_client: &Arc<DefaultHttpClient>, kv: &Arc<KvStore>) -> DidResolver {
    KvStoreCachedResolver::new(
        CommonDidResolver::new(CommonDidResolverConfig {
            plc_directory_url: DEFAULT_PLC_DIRECTORY_URL.to_string(),
            http_client: http_client.clone(),
        }),
        kv.clone(),
        "resolved::did",
    )
}

pub type DidResolver = KvStoreCachedResolver<CommonDidResolver<DefaultHttpClient>>;

impl DidResolver {
    pub async fn resolve_handle_for_did(&self, did: &Did) -> Option<String> {
        match self.resolve(did).await {
            Ok(did_doc) => {
                // also known as list is in priority order so take first
                did_doc
                    .also_known_as
                    .and_then(|akas| akas.first().map(|s| format!("@{}", s).replace("at://", "")))
            }
            Err(err) => {
                console_log!("Error resolving did: {err}");
                None
            }
        }
    }
}

pub fn handle_resolver(http_client: &Arc<DefaultHttpClient>, kv: &Arc<KvStore>) -> HandleResolver {
    KvStoreCachedResolver::new(
        AtprotoHandleResolver::new(AtprotoHandleResolverConfig {
            dns_txt_resolver: DnsOverHttps::new(),
            http_client: http_client.clone(),
        }),
        kv.clone(),
        "resolved:handle",
    )
}

pub type HandleResolver =
    KvStoreCachedResolver<AtprotoHandleResolver<DnsOverHttps, DefaultHttpClient>>;
