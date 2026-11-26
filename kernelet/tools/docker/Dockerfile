# syntax=docker/dockerfile:1
# SPDX-License-Identifier: MPL-2.0

ARG BASE_VERSION
FROM asterinas/dev:${BASE_VERSION}

# QEMU and rustup come from asterinas/dev.
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        bc bison build-essential ca-certificates curl findutils flex \
        gzip libelf-dev patch perl python3 tar util-linux xz-utils \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

ADD --checksum=sha256:b1a2562be56e42afb3f8489d4c2a7ac472ac23098f1ef1c1e40da601f54625eb \
    --chmod=0644 https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.12.tar.xz /opt/kernelet/linux/linux-6.12.tar.xz
ENV LINUX_MIRROR=file:///opt/kernelet/linux

RUN --mount=type=bind,target=/src --mount=type=tmpfs,target=/tmp/kernelet-fetch \
    cd /src/kernelet && rustup show active-toolchain && make crates BUILD=/tmp/kernelet-fetch

WORKDIR /workspace/kernelet
