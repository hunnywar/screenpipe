# Use the official Rust image as the base
FROM rust:latest

# Install system dependencies
RUN apt-get update && apt-get install -y \
    g++ \
    npm \
    ffmpeg \
    tesseract-ocr \
    cmake \
    libavformat-dev \
    libavfilter-dev \
    libavdevice-dev \
    libssl-dev \
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

# Install Node.js (LTS version)
RUN npm install -g bun
