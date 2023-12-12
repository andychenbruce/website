//
//  rpiMemMap.c
//

static const char devmem[] = "/dev/mem";

static UInt32 *
mapMem(int fd, UInt32 addr)
{
  //epf("mapMem(addr=0x%08x)", addr);
  void *p = mmap(0, BLOCK_SIZE,
		   PROT_READ|PROT_WRITE,
		   MAP_SHARED,
		   fd, addr);
  if (p == MAP_FAILED) {
    fatal("mmap failed for %s, addr=0x%08x: %s",
	  devmem, addr, syserr());
  }
  return (UInt32 *) p;
}

static void
mapPeriphrialPtrs(void)
{
  int fd = open(devmem, O_RDWR | O_SYNC);
  if (fd < 0) {
    fatal("Can't open %s: %s", devmem, syserr());
  }
  UInt32 periBase;
  switch (g.modelNum) {
  case RPi_3: periBase = RPI3_PERI_BASE; break;
  case RPi_4: periBase = RPI4_PERI_BASE; break;
  default:    fatal("Unsupported RPi model: %d", g.modelNum);
  }
  epf("periBase=0x%08x ##########", periBase);
  g.pwmRegPtr  = mapMem(fd, periBase + PWM_BASE_OFFSET);
  g.clkRegPtr  = mapMem(fd, periBase + CLK_BASE_OFFSET);
  g.gpioRegPtr = mapMem(fd, periBase + GPIO_BASE_OFFSET);
  close(fd);
}
