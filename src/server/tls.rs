// #![allow(unused_imports)]


use std::{io::{BufReader, self}, fs::File, path::{Path, PathBuf}, sync::Arc};

use rustls_pemfile::{rsa_private_keys, certs};
use tokio_rustls::{rustls::pki_types::{PrivateKeyDer, CertificateDer}, TlsAcceptor};

fn load_certs(path: &Path) -> io::Result<Vec<CertificateDer<'static>>> {
    certs(&mut BufReader::new(File::open(path)?)).collect()
}


fn load_keys(path: &Path) -> io::Result<PrivateKeyDer<'static>> {
    rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .next()
        .expect("Cannot read key")
        .map(Into::into)
}


pub fn load_tls(key_path: String, cert_path: String) -> io::Result<TlsAcceptor> {
    let key = load_keys(&PathBuf::from(key_path))?;
    let certs = load_certs(&PathBuf::from(cert_path))?;

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

    let acceptor = TlsAcceptor::from(Arc::new(config));
    Ok(acceptor)
}

