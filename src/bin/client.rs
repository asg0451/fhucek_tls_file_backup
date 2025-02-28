use std::fs::File;
use std::io::{self, Read};
use std::io::BufReader;
use std::net::{IpAddr, ToSocketAddrs};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use rustls::pki_types::{CertificateDer, UnixTime};
use tokio_rustls::rustls::pki_types::ServerName;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tokio::io::{copy, split, stdin as tokio_stdin, stdout as tokio_stdout, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;
use rustls::client::danger::{ServerCertVerifier, ServerCertVerified};

use file_backup_service::common;


const HOST_ADDR: &str = "127.0.0.1:4545";
const HOST_IP: &str = "127.0.0.1";
const CERT: &str = "/home/frank/certs/ripplein.space-dev.pem";



// const KEY: &str = "certs/key.pem";




// struct SkipServerVerification;

// impl SkipServerVerification {
//     fn new() -> Arc<Self> {
//         Arc::new(Self)
//     }
// }

// impl ServerCertVerifier for SkipServerVerification {
//     fn verify_server_cert(
//         &self,
//         end_entity: &CertificateDer<'_>,
//         intermediates: &[CertificateDer<'_>],
//         server_name: &ServerName<'_>,
//         ocsp_response: &[u8],
//         now: UnixTime
//     ) -> Result<ServerCertVerified, rustls::Error> {
//         Ok(ServerCertVerified::assertion())
//     }
// }


#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = HOST_ADDR
        .to_string()
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::AddrNotAvailable))?;

    
    let mut root_cert_store = rustls::RootCertStore::empty();
    // // let mut roots = rustls::RootCertStore::empty();
    // for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
    //     root_cert_store.add(cert).unwrap();
    // }
    // // let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
        // println!("Got a cert");
        root_cert_store.add(cert).unwrap();
    }


    // println!("OPENING CERT FILE {}", CERT);
    // let mut pem = BufReader::new(File::open(CERT)?);
    // for cert in rustls_pemfile::certs(&mut pem) {
    //     let cert = match cert {
    //         Ok(cert) => {println!("Got a cert"); cert },
    //         Err(_) => {println!("Err occurred "); break; }
    //     };

    //     root_cert_store.add(cert).unwrap();
    // }

    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();
    let tls_connector = TlsConnector::from(Arc::new(config));
    let sock_stream = TcpStream::connect(&addr).await?;
    let ip_addr = ServerName::try_from(HOST_IP).unwrap();
    let mut tls_stream = match tls_connector.connect(ip_addr, sock_stream).await {
        Ok(tls) => tls,
        Err(_e) => {
            println!("{}", _e);
            panic!("FAILED TO CONNECT") 
        }
    };

    let msg = String::from("HI SERVER FROM CLIENT");

    let (mut reader, mut writer) = split(tls_stream);
    writer.write_all(msg.as_bytes()).await?;
    
    writer.flush().await?;
    println!("heree");
    let mut dst = String::new();
    reader.read_to_string(&mut dst).await?;
    println!("RECEIVED FROM SERVER: {}",dst);
    // reader.shutdown().await?;
    // let mut dst = String::new();
    // tls_stream.read_to_string(&mut dst).await?;
    // let (mut stdin, mut stdout) = (tokio_stdin(), tokio_stdout());
    // let (mut reader, mut writer) = split(tls_stream);

    // tokio::select! {
    //     ret = copy(&mut reader, &mut stdout) => {
    //         ret?;
    //     },
    //     ret = copy(&mut stdin, &mut writer) => {
    //         ret?;
    //         writer.shutdown().await?
    //     }
    // }

    println!("ENDED");
    Ok(())
}