# Configuring ESP32 to act as Access Point

Before you running the examples, replace `<YOUR_SSID>` and `<YOUR_PASSWORD>` with your SSID and password in `minimal-https/main/main.cpp` and `production-https/main/main.cpp`.

To build the code, upload to ESP32 and start serial monitor:

```
make upload serial
```

The ESP32 will start a network named `my-esp32-ssid`. The password is `APassword`.

Go to your browser and try to visit any website. You will get a "Hello World" page.
