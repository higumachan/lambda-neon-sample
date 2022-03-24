FROM amazonlinux

WORKDIR /tmp

RUN yum -y install gcc-c++ && yum -y install findutils && yum -y install tar && yum -y install gzip  && yum -y install curl

RUN touch ~/.bashrc && chmod +x ~/.bashrc

RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.39.1/install.sh | bash

RUN source ~/.bashrc && nvm install 14

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /build
