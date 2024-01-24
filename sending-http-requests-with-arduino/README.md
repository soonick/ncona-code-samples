# Sending HTTP Requests from Arduino

Replace `NETWORK_NAME` and `NETWORK_PASSWORD` in `ArduinoHttp/ArduinoHttp.ino` with the values for your WiFi network.

Connect the Arduino board to the computer and run:

```bash
sudo chmod a+rw /dev/ttyACM0 && arduino-cli compile --fqbn arduino:renesas_uno:unor4wifi ArduinoHttp && arduino-cli upload -p /dev/ttyACM0 --fqbn arduino:renesas_uno:unor4wifi ArduinoHttp
```

Open a serial monitor:

```bash
sudo chmod a+rw /dev/ttyACM0 && sudo stty 9600 -F /dev/ttyACM0 raw -echo && sudo cat /dev/ttyACM0
```
