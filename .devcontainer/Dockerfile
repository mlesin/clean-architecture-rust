# syntax=docker/dockerfile:1

# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.140.1/containers/ubuntu/.devcontainer/base.Dockerfile

# [Choice] Ubuntu version: bionic, focal
ARG VARIANT="focal"
FROM mcr.microsoft.com/vscode/devcontainers/base:0-${VARIANT}

# common packages
RUN apt-get update \
    && apt-get install --no-install-recommends -y \
    ca-certificates curl file \
    build-essential \
    autoconf automake autotools-dev libtool xutils-dev \
    pkg-config \
    libssl-dev \
    libpq-dev \
    bash-completion \
    && rm -rf /var/lib/apt/lists/*

# Create default user for container
# Copy the .env file to the container
COPY .env /tmp/.env
# Source to use it
RUN . /tmp/.env && rm /tmp/.env && \
    if getent passwd $USERNAME; then userdel -f $USERNAME; fi && \
    if getent passwd $USER_UID; then userdel -f $USER_UID; fi && \
    if getent group $GROUPNAME; then groupdel $GROUPNAME; fi && \
    if getent group $USER_GID; then groupdel $USER_GID; fi && \
    groupadd --gid $USER_GID $GROUPNAME && \
    useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s /bin/bash && \
    mkdir -p /etc/sudoers.d && \
    echo $USERNAME ALL=\(root\) NOPASSWD:ALL >/etc/sudoers.d/$USERNAME && \
    chmod 0440 /etc/sudoers.d/$USERNAME && \
    chown $USERNAME:$GROUPNAME $(getent passwd $USERNAME | cut -d: -f6) && \
    chmod 755 $(getent passwd $USERNAME | cut -d: -f6)


WORKDIR /workspace

ARG USERNAME
ARG GROUPNAME
ARG USER_UID
ARG USER_GID
ARG HTTP_PROXY

USER $USERNAME

# install toolchain
ENV RUSTUP_HOME=/home/$USERNAME/.local/rustup \
    CARGO_HOME=/home/$USERNAME/.local/cargo \
    PATH=/home/$USERNAME/.local/cargo/bin:$PATH

RUN mkdir -p ${CARGO_HOME} && mkdir -p ${RUSTUP_HOME}


RUN if [ -z "${HTTP_PROXY}" ] ; then \
    echo "Bypass proxy configuration."; \
    else \
    echo "Use proxy configuration (${HTTP_PROXY})"; \
    git config --global https.proxy ${HTTP_PROXY}; \
    git config --global http.proxy ${HTTP_PROXY}; \
    echo "[http]\nproxy = \"${HTTP_PROXY}\"" > ${CARGO_HOME}/config.toml; \
    fi

# --mount=type=cache,id=cargo,target=${CARGO_HOME}/registry,uid=${USER_UID},gid=${USER_GID}\
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -c rust-src -y

RUN mkdir -p ~/.local/share/bash-completion/completions && \
    rustup completions bash > ~/.local/share/bash-completion/completions/rustup && \
    rustup completions bash cargo > ~/.local/share/bash-completion/completions/cargo

# --mount=type=cache,id=cargo,target=${CARGO_HOME}/registry,uid=${USER_UID},gid=${USER_GID}\
RUN cargo install cargo-watch &&\
    cargo install diesel_cli --no-default-features --features 'postgres' &&\
    cargo install sqlx-cli --no-default-features --features rustls,postgres
