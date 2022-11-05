pub mod emailsend;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};
use toml::Value;
use tracing::info;

use crate::emailsend::{
    generate_smtpclient_pool, generate_smtpclient_pool_use_config, handle_emailmessage,
    healthy_check,
};

#[derive(Deserialize, Serialize, Debug)]
struct ServerConfig {
    user: String,
    token: String,
    ipaddr: String,
    port: u16,
}

#[derive(Parser)]
#[clap(author="loyu", version="0.0.1", about="use to start a smtp server", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    ///通过命令行参数运行
    Args {
        ///smtp 用户
        #[clap(value_parser, long)]
        user: String,
        ///smtp 密码
        #[clap(value_parser, long)]
        token: String,
        #[clap(value_parser, long, default_value = "3000")]
        ///监听端口，默认3000
        port: u16,
        ///监听地址，默认127.0.0.1
        #[clap(value_parser, long, default_value = "127.0.0.1")]
        ipaddr: IpAddr,
    },
    ///通过配置文件运行
    Run {
        #[clap(value_parser, long)]
        config: PathBuf,
    },
}

pub async fn cli_run() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        SubCommands::Args {
            user,
            port,
            ipaddr,
            token,
        } => {
            info!(
                "user: {:?}, ip Address: {:?}, port: {:?}",
                user, port, ipaddr
            );
            let mut ip_str = ipaddr.to_string();
            ip_str.push_str(port.to_string().as_str());
            info!("{ip_str}");
            let socket = SocketAddr::new(
                IpAddr::V4(Ipv4Addr::from_str(ipaddr.to_string().as_str()).unwrap()),
                port.to_owned(),
            );
            let smtpclient_pool = generate_smtpclient_pool_use_config(user, token).await?;
            let app = Router::new()
                .route("/sendemail", post(handle_emailmessage))
                .layer(Extension(smtpclient_pool))
                .route("/healthy", get(healthy_check));
            info!("Starting to serve");
            axum::Server::bind(&socket)
                .serve(app.into_make_service())
                .await?;
        }
        SubCommands::Run { config } => {
            println!("{:?}", config);
            let values: ServerConfig = toml::from_str(
                std::fs::read_to_string(config.as_os_str())
                    .map_err(|_| tracing::error!("file not exist, please check the path"))
                    .unwrap()
                    .as_str(),
            )
            .unwrap();
            let smtpclient_pool =
                generate_smtpclient_pool_use_config(&values.user, &values.token).await?;

            let socketv4 = SocketAddr::new(
                IpAddr::V4(
                    Ipv4Addr::from_str(values.ipaddr.to_string().as_str())
                        .map_err(|err| tracing::error!("{:#?}", err))
                        .unwrap(),
                ),
                values.port.to_owned(),
            );
            info!(
                "Starting to server, listen in {}: {}",
                values.ipaddr, values.port
            );
            let app = Router::new()
                .route("/sendemail", post(handle_emailmessage))
                .layer(Extension(smtpclient_pool))
                .route("/healthy", get(healthy_check));
            axum::Server::bind(&socketv4)
                .serve(app.into_make_service())
                .await?;
        }
    }
    Ok(())
}
