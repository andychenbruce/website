//
//  delta.cpp
//
//  Serial interface for Delta-Robot.
//

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>
#include <math.h>
#include <errno.h>
#include <unistd.h>
#include <termios.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/select.h>
#include "utils.cpp"
#include "serial.cpp"

static const char RobotPort[] = "/dev/cu.usbmodem144101";
//static const char RobotPort[] = "/dev/tty.DeltaX-SerialPort";

static SInt fd; // Device file descriptor

static void
sendLine(const char * const fmt, ...)
  __attribute__ ((format(printf, 1, 2)));

static void
sendLine(const char *fmt, ...)  // Send a formated string to the robot
{
  char s[0x1000];
  va_list args;
  va_start(args, fmt);
  vsprintf(s, fmt, args);
  Write(fd, s, strlen(s));
  Write(fd, "\n", 1);
}

static void
clearSerial(void)  // Clear buffered data.
{
  while (true) {
    SInt c = serialReadOneByte(fd, 1000000);    
    if (c == -1) {
      printf("serial empty\n");
      break;
    } else {
      printf("%c", c);
    }
  }
}

static void
gotoCommand(double x, double y, double z)
{
  double thirtyDegrees = 30*(M_PI/180.0);
  double oneTwentyDegrees = 120*(M_PI/180.0);
  double realX = x*cos(thirtyDegrees) + y*cos(oneTwentyDegrees);
  double realY = x*sin(thirtyDegrees) + y*sin(oneTwentyDegrees);
  sendLine("G01 X%f Y%f Z%f", realX, realY, z);
  usleep(1000000);
}

static void
arcCommand(double x, double y, double i, double j)
{
  double thirtyDegrees = 30*(M_PI/180.0);
  double oneTwentyDegrees = 120*(M_PI/180.0);
  double realX = x*cos(thirtyDegrees) + y*cos(oneTwentyDegrees);
  double realY = x*sin(thirtyDegrees) + y*sin(oneTwentyDegrees);
  double realI = i*cos(thirtyDegrees) + j*cos(oneTwentyDegrees);
  double realJ = i*sin(thirtyDegrees) + j*sin(oneTwentyDegrees);
  sendLine("G02 X%f Y%f I%f J%f", realX, realY, realI, realJ);
}

int
main(void)
{
  fd = serialPortOpen(RobotPort);
  clearSerial();
  sendLine("IsDelta");
  sendLine("IsDelta");
  clearSerial();
  sendLine("G28");
  usleep(2000000);
  double upH = -360;
  double downH = -385;
  gotoCommand(-20.0, 50.0, upH);
  gotoCommand(-20.0, 50.0, downH);
  gotoCommand(30.0, 50.0, downH);
  gotoCommand(30.0, 50.0, upH);
  gotoCommand(-20.0, -50.0, upH);
  gotoCommand(-20.0, -50.0, downH);
  gotoCommand(30.0, -50.0, downH);
  gotoCommand(30.0, -50.0, upH);
  gotoCommand(60.0, -70.0, downH);
  arcCommand(60.0, 70.0, 0.0, 70.0);
  for (;;) {
    SInt c = serialReadOneByte(fd, 1000000);
    if (c == -1) {
      printf("serial empty\n");
    } else {
      printf("%d = %c\n", c, c);
    }
  }
}
