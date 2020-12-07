FROM rustlang/rust:nightly-slim

RUN apt-get update \
&& apt-get install \
    libssl-dev \
    lld \
    cmake \
    jupyter-notebook \
    pkg-config \
    git \
    -y \
&& rm -rf /var/lib/apt/lists/*

