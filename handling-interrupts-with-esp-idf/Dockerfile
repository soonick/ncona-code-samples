FROM ubuntu:latest

# Install ESP-IDF
RUN apt-get update && \
  apt-get install -y git wget flex bison gperf python3 python3-pip python3-venv \
  cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0

WORKDIR /
RUN git clone --depth=1 -b v5.3.1 --recurse-submodules --shallow-submodules https://github.com/espressif/esp-idf.git
WORKDIR /esp-idf
RUN ./install.sh esp32
ENV IDF_PATH="/esp-idf"
