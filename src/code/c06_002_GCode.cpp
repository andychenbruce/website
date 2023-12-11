//
//  serial.cpp
//

static SInt
serialPortOpen(const char *devicePath)
{
  int fd;
  struct termios options;
  fd = open(devicePath, O_RDWR | O_NOCTTY | O_NDELAY);
  if (fd < 0) {
    fatal("Device \"%s\" cannot be opened: %s", devicePath, syserr());
  }
  fcntl(fd, F_SETFL, FNDELAY);       // Open the device in nonblocking mode
  if (tcgetattr(fd, &options) < 0) { // Get the current options of the port
    fatal("tcgetattr failed: %s", syserr());
  }
  bzero(&options, sizeof(options));  // Clear all the options
  cfsetispeed(&options, B115200);    // Set the baud rate at 115200 bauds
  cfsetospeed(&options, B115200);
  
  options.c_cflag |= ( CLOCAL | // Ignore modem control lines
                       CREAD  | // Enable receive
                       CS8    | // 8 data, 1 stop, no parity, no flow control
                       PARMRK); // Mark framing errors
  
  options.c_iflag |= ( IGNPAR | IGNBRK );
  options.c_oflag = 0;
  options.c_cc[VTIME]=0;    // Timer unused
  options.c_cc[VMIN]=0;     // At least on character before satisfy reading
  cfmakeraw(&options);
  if (tcsetattr(fd, TCSANOW, &options) < 0) {  // Activate the settings
    fatal("tcsetattr failed, %s: %s", devicePath, syserr());
  }
  return fd;
}

static SInt
serialReadOneByte(SInt fd, UInt usecs)
{
  if (! select1(usecs, fd)) {
    return -1;  // Timeout
  }
  UInt8 c;
  Read(fd, &c, 1);
  return (SInt) c;
}
