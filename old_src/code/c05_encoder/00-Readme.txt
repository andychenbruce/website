======================================================================
2021-03-25

~/src/encoderDriver/

make all

insmod encoderDriver.ko

grep encoderDriver /proc/devices

mknod /dev/encoder c 234 0

rmmod encoderDriver.ko

======================================================================
