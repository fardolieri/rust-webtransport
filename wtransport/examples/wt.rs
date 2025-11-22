use anyhow::Result;
use tracing::error;
use tracing::info;
use tracing::info_span;
use tracing::Instrument;
use webtransport::WebTransportServer;
use wtransport::tls::Sha256DigestFmt;
use wtransport::Identity;

const WEBTRANSPORT_PORT: u16 = 5000;

#[tokio::main]
async fn main() -> Result<()> {
    utils::init_logging();

    let identity = Identity::self_signed(["localhost", "127.0.0.1", "::1"]).unwrap();
    let cert_digest = identity.certificate_chain().as_slice()[0].hash();
    let cert_digest_hash = cert_digest.fmt(Sha256DigestFmt::BytesArray);
    println!("cert_digest_hash: {cert_digest_hash}");

    let webtransport_server = WebTransportServer::new(identity)?;

    tokio::select! {
        result = webtransport_server.serve() => {
            error!("WebTransport server: {:?}", result);
        }
    }

    Ok(())
}

mod webtransport {
    use super::*;
    use std::time::Duration;
    use wtransport::endpoint::endpoint_side::Server;
    use wtransport::endpoint::IncomingSession;
    use wtransport::Endpoint;
    use wtransport::ServerConfig;

    pub struct WebTransportServer {
        endpoint: Endpoint<Server>,
    }

    impl WebTransportServer {
        pub fn new(identity: Identity) -> Result<Self> {
            let config = ServerConfig::builder()
                .with_bind_default(WEBTRANSPORT_PORT)
                .with_identity(identity)
                .keep_alive_interval(Some(Duration::from_secs(3)))
                .build();

            let endpoint = Endpoint::server(config)?;

            Ok(Self { endpoint })
        }

        pub fn local_port(&self) -> u16 {
            self.endpoint.local_addr().unwrap().port()
        }

        pub async fn serve(self) -> Result<()> {
            info!("Server running on port {}", self.local_port());

            for id in 0.. {
                let incoming_session = self.endpoint.accept().await;

                tokio::spawn(
                    Self::handle_incoming_session(incoming_session)
                        .instrument(info_span!("Connection", id)),
                );
            }

            Ok(())
        }

        async fn handle_incoming_session(incoming_session: IncomingSession) {
            async fn handle_incoming_session_impl(incoming_session: IncomingSession) -> Result<()> {
                let mut buffer = vec![0; 65536].into_boxed_slice();

                info!("Waiting for session request...");

                let session_request = incoming_session.await?;

                info!(
                    "New session: Authority: '{}', Path: '{}'",
                    session_request.authority(),
                    session_request.path()
                );

                let connection = session_request.accept().await?;

                info!("Waiting for data from client...");

                loop {
                    tokio::select! {
                        stream = connection.accept_bi() => {
                            let mut stream = stream?;
                            info!("Accepted BI stream");

                            let Some(bytes_read) = stream.1.read(&mut buffer).await? else {
                                continue;
                            };

                            let str_data = std::str::from_utf8(&buffer[..bytes_read])?;

                            info!("Received (bi) '{str_data}' from client");

                            stream.0.write_all(b"ACK").await?;
                        }
                        stream = connection.accept_uni() => {
                            let mut stream = stream?;
                            info!("Accepted UNI stream");

                            let Some(bytes_read) = stream.read(&mut buffer).await? else {
                                continue;
                            };

                            let str_data = std::str::from_utf8(&buffer[..bytes_read])?;

                            info!("Received (uni) '{str_data}' from client");

                            let mut stream = connection.open_uni().await?.await?;
                            stream.write_all(b"ACK").await?;
                        }
                        dgram = connection.receive_datagram() => {
                            let dgram = dgram?;
                            let str_data = std::str::from_utf8(&dgram)?;

                            info!("Received (dgram) '{str_data}' from client");

                            connection.send_datagram(b"ACK")?;
                        }
                    }
                }
            }

            let result = handle_incoming_session_impl(incoming_session).await;
            info!("Result: {:?}", result);
        }
    }
}

mod utils {
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::EnvFilter;

    pub fn init_logging() {
        let env_filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy();

        tracing_subscriber::fmt()
            .with_target(true)
            .with_level(true)
            .with_env_filter(env_filter)
            .init();
    }
}
