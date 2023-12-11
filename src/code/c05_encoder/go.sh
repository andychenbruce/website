
make all

rmmod encoderDriver.ko

insmod encoderDriver.ko

mknod /dev/encoder c 234 0
