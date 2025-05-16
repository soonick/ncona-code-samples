# Adding Configurations to ESP-IDF projects

## To generate sdkconfig file

```
make menuconfig
```

## Upload image to board and connect to serial monitor

```
make upload
```

## Troubleshooting

If you encounter errors connecting to the ESP32 board, you might need to run these commands on your system:

```
sudo adduser $USER dialout
sudo chmod a+rw /dev/ttyUSB0
```
