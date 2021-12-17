# mihno
A simple TCP server honeypot written in Rust compatible with raw TCP and HTTP clients.

As usual for rust, `cargo build --release` to compile it.

Raw TCP clients like telnet and netcat can send in data, as well as HTTP clients
like cURL etc.

```
$ telnet mihno-host 3975
Trying ::1...
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
funky data and stuff
HTTP/1.1 200 OK
Content-Type: text/html; charset=UTF-8

<html><body><h1>H A R V E S T E D </h1></br></br><p>performance +</br></br>a2c727ee8f206913df426b6bd29d7727bf19a10229466edfc349812388c911bd</p></body></html>
Connection closed by foreign host.
```


The output goes to STDOUT, so redirect to where you need etc.

```
2021-12-17 16:14:55.063733301 UTC 56cbcac2-2fba-40e7-af7d-83de10c61cbd  _--->_ start transaction
2021-12-17 16:14:55.063855867 UTC GET / HTTP/1.1
Host: mihno-host:3975
User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
Sec-GPC: 1
Cache-Control: max-age=0


2021-12-17 16:14:55.063947049 UTC Response sent _<---_ end transaction
2021-12-17 16:47:10.334472197 UTC d163ceab-6aca-4a76-9e87-e39c8189cc71  _--->_ start transaction
2021-12-17 16:47:10.334686574 UTC POST / HTTP/1.1
Host: localhost:3975
User-Agent: curl/7.74.0
Accept: */*
Content-Length: 9
Content-Type: application/x-www-form-urlencoded

My client body
2021-12-17 16:47:10.334781644 UTC Response sent _<---_ end transaction
2021-12-17 16:49:15.120140610 UTC bb7b0e52-57ab-47c2-b4ea-d8266f618917  _--->_ start transaction
2021-12-17 16:49:15.120350379 UTC funky data and stuff

2021-12-17 16:49:19.602318623 UTC Response sent _<---_ end transaction

```
