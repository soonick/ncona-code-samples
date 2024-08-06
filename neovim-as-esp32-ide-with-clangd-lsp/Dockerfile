FROM ubuntu:latest

# Install ESP-IDF
RUN apt-get update && \
  apt-get install -y git wget flex bison gperf python3 python3-pip python3-venv \
  cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0

WORKDIR /
RUN git clone -b v5.2.2 --recursive https://github.com/espressif/esp-idf.git
WORKDIR /esp-idf
RUN ./install.sh esp32
ENV IDF_PATH="/esp-idf"

# Install Neovim
RUN apt-get update && \
  apt-get install git build-essential libncurses-dev libncurses5-dev npm golang \
      libgtk2.0-dev libatk1.0-dev libcairo2-dev libx11-dev libxpm-dev libxt-dev \
      curl default-jre ninja-build gettext make cmake unzip python3-dev ripgrep \
      clang-format tree -y

RUN git clone --depth=1 -b stable https://github.com/neovim/neovim /nvim
WORKDIR /nvim
RUN make CMAKE_BUILD_TYPE=RelWithDebInfo
RUN make install
