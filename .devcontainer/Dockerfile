# https://github.com/devcontainers/images/blob/main/src/rust/.devcontainer/Dockerfile
ARG VARIANT="bookworm"
FROM mcr.microsoft.com/devcontainers/rust:${VARIANT}

# Rename the 'vscode' user.
# https://code.visualstudio.com/remote/advancedcontainers/add-nonroot-user#_creating-a-nonroot-user
ARG USERNAME=developer
RUN groupmod --new-name $USERNAME vscode &&\
  usermod --move-home --home /home/$USERNAME --login $USERNAME vscode &&\
  usermod --append --groups users $USERNAME &&\
  echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME &&\
  chmod 0440 /etc/sudoers.d/$USERNAME &&\
  rm /etc/sudoers.d/vscode

# Become the normal user and cd.
USER $USERNAME
WORKDIR /home/$USERNAME

# Install system dependencies
RUN sudo apt-get update && sudo apt-get install -y \
    g++ \
    npm \
    ffmpeg \
    tesseract-ocr \
    cmake \
    libavformat-dev \
    libavfilter-dev \
    libavdevice-dev \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    build-essential \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    trash-cli \
    at-spi2-core \
    libatspi2.0-0 \
    libcanberra-gtk-module \
    libcanberra-gtk3-module \
    libgtk-3-dev \
    x11-apps \
    xdg-utils \
    libtesseract-dev \
    libxdo-dev \
    libsdl2-dev \
    libclang-dev \
    libxtst-dev \
    libx11-dev \
    libxext-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libasound2-dev \
    libpulse-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Rust tools
RUN rustup component add clippy rustfmt

# Install Bun
RUN npm install -g bun

RUN sudo groupadd -g 109 render &&\
  sudo usermod -aG render,video developer &&\
  sudo mkdir -p /run/user/1000/dconf && sudo chown -R 1000:1000 /run/user/1000

# # Detect Bun's path and move it to $HOME/.bun/bin/bun
# RUN BUN_PATH=$(which bun) && \
#     mkdir -p $HOME/.bun/bin && \
#     cp "$BUN_PATH" $HOME/.bun/bin/bun && \
#     chmod +x $HOME/.bun/bin/bun
