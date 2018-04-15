FROM debian:latest
MAINTAINER Adam Schwalm <adamschwalm@gmail.com>

RUN apt-get update

# install dependencies
RUN apt-get install -y make gcc file g++ patch wget cpio python unzip rsync bc bzip2 git
