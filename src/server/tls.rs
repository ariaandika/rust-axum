// #![allow(unused_imports)]


use std::sync::Arc;

use rustls::{ServerConfig, pki_types::CertificateDer};
use rustls_pemfile::private_key;
use tokio_rustls::TlsAcceptor;

pub fn load_tls(key_dir: String, cert_dir: String) -> Result<TlsAcceptor, Box<dyn std::error::Error>> {
    
    let cert_buf = std::io::BufReader::new(std::fs::File::open(cert_dir)?);
    let cert = CertificateDer::from(cert_buf.buffer().to_vec());

    let mut key_buf = std::io::BufReader::new(std::fs::File::open(key_dir)?);
    let keys = private_key(&mut key_buf)?.unwrap();
    
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], keys)?;
    Ok(TlsAcceptor::from(Arc::new(config)))
}
