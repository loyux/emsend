# 介绍

使用smtp服务器，写了一个http api进行发送邮件，通过配置文件启动http服务

## 配置文件

使用以下配置文件
example: cfg.toml

```
[smtpserver]
user = "邮件账户xxx@xxx.com"
token = "邮箱smtp生成的授权码"
```

运行 posmtp --config cfg.toml --port 3000

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
