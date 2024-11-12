use reqwest::blocking::get;

use crate::apis::api::CertificateFetcher;
use crate::structs::certificate::Certificate;

pub struct CrtShFetcher;

impl CertificateFetcher for CrtShFetcher {
    fn fetch_certificates(website: &str) -> Result<Vec<Certificate>, reqwest::Error> {
        let url = format!("https://crt.sh/?q={}&output=json", website);
        let response = get(&url)?.json::<Vec<Certificate>>()?;
        Ok(response)
    }
}
