use crate::structs::certificate::Certificate;

pub trait CertificateFetcher {
    fn fetch_certificates(website: &str) -> Result<Vec<Certificate>, reqwest::Error>;
}
