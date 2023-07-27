FROM node:18

RUN apt-get update && apt-get install -y \
    build-essential \
    zip \
    unzip \
    git \
    curl \
    libzip-dev \
    clang

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y