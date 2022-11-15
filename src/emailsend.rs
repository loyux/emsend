use std::path::PathBuf;

use anyhow::Error;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use lettre::transport::smtp::AsyncSmtpTransport;
use lettre::{
    transport::smtp::{authentication::Credentials, PoolConfig},
    Message,
};
use lettre::{AsyncTransport, Tokio1Executor};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use toml;
use tracing::info;

#[derive(Serialize, Deserialize)]
pub struct EmailMessage {
    ///邮件主题
    pub subject: String,
    ///邮件内容
    pub body: String,
    ///邮件收件人
    pub reciever: String,
    ///邮件发件人，与smtp一致
    pub sender: String,
    ///api鉴权
    pub tokens: String,
}

///用户名密码创建与注册

///根据用户名密码创建token，并存储token的hash值
pub async fn get_access_token() -> String {}

pub async fn handle_emailmessage(
    Json(email_message): Json<EmailMessage>,
    async_pools_sender: Extension<AsyncSmtpTransport<Tokio1Executor>>,
) -> impl IntoResponse {
    if email_message.tokens == "tokens" {
        let mut return_response = String::new();
        //一个具有多个所有者的可变引用
        let recv = email_message.reciever.clone();
        // let rep = RefCell::new(return_response);

        let email = Message::builder()
            .from(email_message.sender.parse().unwrap())
            .reply_to(email_message.sender.parse().unwrap())
            .to(email_message.reciever.parse().unwrap())
            .subject(email_message.subject)
            .body(email_message.body)
            .unwrap();
        let result = tokio::spawn(async move {
            let result = async_pools_sender.send(email).await;
            match result {
                Ok(_) => {
                    info!("Send email to {} successful", recv);
                    let infomsg = format!("Send email to {} successful", recv);

                    return_response.push_str(&infomsg);
                    return_response
                }
                Err(e) => {
                    info!("Send email error: {}", e);
                    let dd = format!("{:#?}", e);
                    return_response.push_str(dd.as_str());
                    return_response
                }
            }
        })
        .await;
        match result {
            Ok(resp) => (StatusCode::ACCEPTED, resp),
            Err(e) => {
                let dd = format!("{:#?}", e);
                (StatusCode::ACCEPTED, dd)
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "Token is not effective")
    }
}

///Generate a pool of smtp_async_client
pub async fn generate_smtpclient_pool(
    cfg_path: &PathBuf,
) -> Result<AsyncSmtpTransport<Tokio1Executor>, Error> {
    info!("Read smtp config {:?} successful!", &cfg_path);
    let values: Value = toml::from_str(
        std::fs::read_to_string(cfg_path.as_os_str())
            .unwrap()
            .as_str(),
    )
    .to_owned()
    .unwrap();

    // let valueParse: &dyn Fn(&str, &str) -> &str = &|x: &'static str, y: &'static str| -> &str {
    //     values
    //         .get(x)
    //         .expect("can't find values")
    //         .get(y)
    //         .expect("can't find values of the config file")
    //         .as_str()
    //         .unwrap()
    // };

    let valueParse = |x: &'static str, y: &'static str| -> &str {
        values
            .get(x)
            .expect("can't find values")
            .get(y)
            .expect("can't find values of the config file")
            .as_str()
            .unwrap()
    };
    let token = valueParse("smtpserver", "token").into();
    let user = valueParse("smtpserver", "user").into();
    info!("Starting connecting {}", &user);
    let pools = PoolConfig::new().min_idle(0).max_size(10);
    let async_pools_sender: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.163.com")
            .unwrap()
            .credentials(Credentials::new(user, token))
            .pool_config(pools)
            .build();
    // async_sender.send(email).await.unwrap();
    info!("build pool complete");
    Ok(async_pools_sender)
}

pub async fn generate_smtpclient_pool_use_config(
    user: &str,
    token: &str,
) -> Result<AsyncSmtpTransport<Tokio1Executor>, Error> {
    // info!("Read smtp config {:?} successful!", &cfg_path);
    // let values: Value = toml::from_str(
    // std::fs::read_to_string(cfg_path.as_os_str())
    // .unwrap()
    // .as_str(),
    // )
    // .unwrap();
    info!("Starting connecting {}", &user);
    let pools = PoolConfig::new().min_idle(0).max_size(10);
    let async_pools_sender: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.163.com")
            .unwrap()
            .credentials(Credentials::new(user.to_string(), token.to_string()))
            .pool_config(pools)
            .build();
    // async_sender.send(email).await.unwrap();
    info!("build pool complete");
    Ok(async_pools_sender)
}

pub async fn healthy_check() -> impl IntoResponse {
    info!("do healthy_check, Healthy");
    "Healthy\n"
}

#[test]
fn test_cfgfile() {
    let values: Value = toml::from_str(
        std::fs::read_to_string(std::path::Path::new("smtp_server.toml"))
            .unwrap()
            .as_str(),
    )
    .unwrap();
    //111
    let token = values
        .get("smtpserver")
        .unwrap()
        .get("token")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let user = values
        .get("smtpserver")
        .unwrap()
        .get("user")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    println!("{}   {}", token, user);
    //这样取得的值有转义符号
    // let tokens = values["smtpserver"].get("token").unwrap().to_string();
    // println!("{:?}   {:?}", user, token);
}
