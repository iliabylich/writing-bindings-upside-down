FROM rust:1.57

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update
RUN apt-get upgrade
RUN apt install -y lsb-release wget software-properties-common

RUN wget https://apt.llvm.org/llvm.sh
RUN chmod +x llvm.sh
RUN ./llvm.sh 13

RUN apt-get install zsh git less nano

ENV CC=clang-13
ENV CXX=clang++-13
ENV AR=llvm-ar-13

RUN mkdir -p /Users/ilyabylich/Work

RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
RUN chsh -s $(which zsh)
