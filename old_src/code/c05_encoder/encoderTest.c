//
//  encoderTest.c
//
//  Test program for encoder device driver
//

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <errno.h>
#include <fcntl.h>
#include <unistd.h>

#include "encoderDriver.h"

//====================================================================

static const char device_name[] = "/dev/encoderDriver";

static EncoderInfo encoderInfo;
static int fd;

#define e encoderInfo

static void
openEncoderDevice(void)
{
  if ((fd = open(device_name, O_RDONLY)) < 0) {
    fprintf(stderr, "Cannot open {%s}: %s\n", device_name, strerror(errno));
    exit(1);
  }
}

static void
readEncoderDevice(void)
{
  ssize_t r;
  if ((r = read(fd, (void *) &encoderInfo, sizeof(encoderInfo))) < 0) {
    fprintf(stderr, "Read error {%s}: %s\n", device_name, strerror(errno));
    exit(1);
  }
  if (r != sizeof(encoderInfo)) {
    fprintf(stderr, "Short read on {%s}: r=%d\n", device_name, (int) r);
    return;
  }
}

static void
printEncoderInfo(void)
{
  UInt8 ia = (e.indexA + 7) & 7;
  UInt8 ib = (e.indexB + 7) & 7;
  UInt8 pa = (e.indexA + 6) & 7;
  UInt8 pb = (e.indexB + 6) & 7;

  SInt32 dtA = e.microsecondsA[ia] - e.microsecondsA[pa];
  SInt32 dtB = e.microsecondsA[ib] - e.microsecondsA[pb];
  
  printf("R {i: %d %d} {s: %d %d} {err: %x %x} {ec: %d, %d} {t: %d %d} %d\n",
	 e.indexA,
	 e.indexB,
	 e.stepDirectionA,
	 e.stepDirectionB,
	 e.error00,
	 e.error01,
	 e.encoderCountA,
	 e.encoderCountB,
	 dtA,
	 dtB,
	 e.microsecondsReadTime);

  /*
  printf("R {%d %d %d} {%d %d %d} {%x %x} %d\n",
	 e.encoderCountA,
	 e.microsecondsBetweenA,
	 e.microsecondsA,
	 e.encoderCountB,
	 e.microsecondsBetweenB,
	 e.microsecondsB,
	 encoderInfo[n].error00,
	 encoderInfo[n].error01,
	 encoderInfo[n].microsecondsReadTime);
  */
}

int
main(void)
{
  openEncoderDevice();
  for (int i = 0; i < 50; ++i) {
    readEncoderDevice();
    printEncoderInfo();
    usleep(500000);
  }
  close(fd);
  return 0;
}
