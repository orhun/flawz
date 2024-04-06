use nvd_cve::cache::{get_all, sync_blocking, CacheConfig};
use nvd_cve::client::{BlockingHttpClient, ReqwestBlockingClient};

pub fn main() {
    let mut config = CacheConfig::new();
    config.feeds = vec!["2024".to_string()];

    let client = ReqwestBlockingClient::new(&config.url, None, None, None);

    if let Err(error) = sync_blocking(&config, client) {
        eprintln!("Fatal Error while syncing feeds: {:?}", error);
        std::process::exit(1);
    }

    if let Ok(cves) = get_all(&config) {
        for cve in cves {
            println!("{:#?}", &cve);
        }
    }
}
