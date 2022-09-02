FROM ubuntu
LABEL maintainer="carefuldata@protonmail.com"
LABEL version="0.1"
LABEL description="M I H N O honeypot container"
RUN apt-get update && apt-get install -y apt-transport-https aptitude && rm -rf /usr/local/share/man
COPY target/release/mihno /usr/local/sbin/
COPY ./mihno.toml /etc/
EXPOSE 3975
CMD /usr/local/sbin/mihno
