//
//  encoderTestMmap.c
//
//  Test program for encoder device driver
//

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <locale.h>
#include <errno.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/mman.h>

#include "encoderDriver.h"

//====================================================================

//static const char device_name[] = "/dev/encoderDriver";
static const char device_name[] = "/dev/encoder";

struct {
  EncoderInfo *ep;
  int fd;
} g;

static void
openEncoderDevice(void)
{
  if ((g.fd = open(device_name, O_RDONLY)) < 0) {
    fprintf(stderr, "Cannot open {%s}: %s\n", device_name, strerror(errno));
    exit(1);
  }
}

static void
mmapEncoderDevice(void)
{
  g.ep = mmap(0, 4096, PROT_READ, MAP_SHARED, g.fd, 0);
  if (g.ep == MAP_FAILED) {
    fprintf(stderr, "mmap failed for %s: %s\n", device_name, strerror(errno));
    exit(1);
  }
}

static void
printEncoderInfo(void)
{
  static int lastA;
  static int lastB;

  if ((g.ep->encoderCountA == lastA) &&
      (g.ep->encoderCountB == lastB)) {
    return;
  }
  
  char buf[64];
  sprintf(buf, "%d %d\n",
	  g.ep->encoderCountA,
	  g.ep->encoderCountB);
  write(1, buf, strlen(buf));
  lastA = g.ep->encoderCountA;
  lastB = g.ep->encoderCountB;
}

int
main(void)
{
  setlocale(LC_ALL, "");
  openEncoderDevice();
  mmapEncoderDevice();
  fprintf(stderr, "Magic=0x%08x\n", g.ep->magic);
  fprintf(stderr, "Version=%d\n", g.ep->version);
  for (int i = 0; i < 1000000; ++i) {
    printEncoderInfo();
    usleep(100000);
  }
  munmap(g.ep, 4096);
  close(g.fd);
  return 0;
}
