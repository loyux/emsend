# 介绍
利用163 smtp服务器，通过http api进行发送邮件，通过配置文件启动http服务;

1. 将其嵌入到容器中作为容器本地服务进行邮件发送

## 通过配置文件运行
使用以下配置文件

example: cfg.toml
```
user = "邮件账户xxx@xxx.com"
token = "邮箱smtp生成的授权码"
ipaddr = "127.0.0.1"
port = 1234
```
运行 postmp run --config /vdb/posmtp/cfg.toml

## 通过命令行启动
cargo run -- args --user xxxxxxxx@xx.com --token xxxxxxxxx 
可选增加 --ipaddr 0.0.0.0 --port 1234

## post请求结构体

```
{
subject:主题
body:内容
reciever:接收者
sender: 发送者，一般为smtp服务器所使用邮箱
}
Json>
{"subject":"容器状态检查","body":"状态健康","reciever":"lito0210@outlook.com", "sender": "loyurs@163.com"}
```
