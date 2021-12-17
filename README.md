# mihno
A simple TCP server honeypot written in Rust compatible with raw TCP and HTTP clients.

sha256 of mihno binary
27a4f56760afd4f042e1b09de78967edc4195f6d95bb03527173314345ff9caa

sha1 of mihno binary
fd92201da8f1dc965ca98d8003537666f2598659


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

In this example output, we can see the telnet data in the middle there. The first transaction is a GET / from a web browser,
the second a raw TCP (from our telnet above), and the third a curl doing a POST.

```
2021-12-17 17:13:13.744257569 UTC Listening for connections on port 3975
2021-12-17 17:13:19.679084778 UTC 72a6c16d-75ad-4c6a-bed1-c9c6ce143845  _--->_ start transaction
2021-12-17 17:13:19.679346530 UTC 192.168.1.133:59677 GET / HTTP/1.1
Host: mihno-host:3975
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:95.0) Gecko/20100101 Firefox/95.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
DNT: 1
Connection: keep-alive
Cookie: jenkins-timestamper-offset=21600000
Upgrade-Insecure-Requests: 1
Sec-GPC: 1
Cache-Control: max-age=0


2021-12-17 17:13:19.679460707 UTC Response sent _<---_ end transaction
2021-12-17 17:13:39.620908274 UTC de379029-0fd9-4e72-9874-9a3ba277b3a9  _--->_ start transaction
2021-12-17 17:13:39.620966387 UTC 127.0.0.1:49364 some funky tcp data sent in

2021-12-17 17:13:46.974986805 UTC Response sent _<---_ end transaction
2021-12-17 19:09:17.934566878 UTC 60bd9d55-9784-4846-a10c-0af23cd6c396  _--->_ start transaction
2021-12-17 19:09:17.934707002 UTC 127.0.0.1:59622 POST / HTTP/1.1
Host: localhost:3975
User-Agent: curl/7.74.0
Accept: */*
Content-Length: 27
Content-Type: application/x-www-form-urlencoded

honk honk message body post
2021-12-17 19:09:17.934791245 UTC Response sent _<---_ end transaction

```
