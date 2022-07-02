use std::{net::IpAddr, path::PathBuf};

use anyhow::Error;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use clap::{arg, command, value_parser};
use posmtp::emailsend::{generate_smtpclient_pool, handle_emailmessage, healthy_check};
use tracing::info;
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "use config file like cfg.tmol "
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-i --ip <IpAddr> "Set ip addr to listen, default 0.0.0.0")
                .required(false)
                .value_parser(value_parser!(IpAddr)),
        )
        .arg(
            arg!(-p --port <Port> "Set port to listen")
                .required(true)
                .value_parser(value_parser!(u16)),
        )
        .get_matches();
    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        // let smtpclient_pool = generate_smtpclient_pool(config_path).await.unwrap();
        info!("{:?}", config_path);
        match matches.get_one::<IpAddr>("ip") {
            Some(ip_addr) => {
                if let Some(port) = matches.get_one::<u16>("port") {
                    info!("ip Address: {:?}, port: {:?}", ip_addr, port);
                    let mut ip_str = ip_addr.to_string();
                    ip_str.push_str(port.to_string().as_str());
                    let smtpclient_pool = generate_smtpclient_pool(config_path).await.unwrap();
                    let app = Router::new()
                        .route("/sendemail", post(handle_emailmessage))
                        .layer(Extension(smtpclient_pool))
                        .route("/healthy", get(healthy_check));
                    axum::Server::bind(&ip_str.parse().unwrap())
                        .serve(app.into_make_service())
                        .await?;
                }
            }
            None => {
                if let Some(port) = matches.get_one::<u16>("port") {
                    info!("Default listen in 0.0.0.0:{:?}", port);
                    let smtpclient_pool = generate_smtpclient_pool(config_path).await.unwrap();
                    let mut ip_str = "0.0.0.0:".to_string();
                    ip_str.push_str(port.to_string().as_str());

                    let app = Router::new()
                        .route("/sendemail", post(handle_emailmessage))
                        .layer(Extension(smtpclient_pool))
                        .route("/healthy", get(healthy_check));
                    axum::Server::bind(&ip_str.parse().unwrap())
                        .serve(app.into_make_service())
                        .await?;
                }
            }
        }
    }
    Ok(())
}
