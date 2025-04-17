use std::sync::Arc;

use atrium_identity::{
    did::{CommonDidResolver, CommonDidResolverConfig, DEFAULT_PLC_DIRECTORY_URL},
    handle::{AtprotoHandleResolver, AtprotoHandleResolverConfig},
};
use atrium_oauth::DefaultHttpClient;
use dns_over_http::DnsOverHttps;
use worker::kv::KvStore;

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
