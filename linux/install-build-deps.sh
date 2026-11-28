#!/usr/bin/env bash
set -euo pipefail

apt-get update
apt-get install -y \
  build-essential \
  libvulkan-dev \
  libwayland-dev \
  libxkbcommon-dev \
  libxkbcommon-x11-dev \
  libx11-dev \
  libxcb1-dev \
  libxcb-render0-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev \
  libfontconfig1-dev \
  libfreetype6-dev \
  libssl-dev \
  libgtk-3-dev \
  libgdk-pixbuf2.0-dev \
  libatk1.0-dev \
  libcairo2-dev \
  libpango1.0-dev \
  libglib2.0-dev \
  libsoup-3.0-dev \
  libwebkit2gtk-4.1-dev \
  libjavascriptcoregtk-4.1-dev \
  libxdo-dev \
  pkg-config \
  cmake
