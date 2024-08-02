# Introduction to ESP32 development

## Upload image to board and connect to serial monitor

```
make upload
```

If you encounter errors uploading the image, you might need to run these commands on your system:

```
sudo adduser $USER dialout
sudo chmod a+rw /dev/ttyUSB0
```
