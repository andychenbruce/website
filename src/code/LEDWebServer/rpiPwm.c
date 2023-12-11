//
// rpiPwm.c
//

static void
pwmEnable(void)
{
  usleep(100); // FIXME - Why is this needed?
  g.pwmRegPtr[PWM_CTL] |= ((1 << PWEN_A) | (1 << PWEN_B));
}

static void
pwmDisable(void)
{
  UInt32 r = g.pwmRegPtr[PWM_STA];
  usleep(MS);
  epf("pwmDisable 1 PWM_STA = 0x%08x", r);
  usleep(MS);
  g.pwmRegPtr[PWM_CTL] = PWM_CTL_RESET;
  g.pwmRegPtr[PWM_STA] = PWM_STA_RESET;
  usleep(MS);
  g.pwmRegPtr[PWM_STA] = PWM_STA_RESET;
  r = g.pwmRegPtr[PWM_STA];
  epf("pwmDisable 2 PWM_STA = 0x%08x", r);
  for (int i = 0; i < 10000; ++i) {
    r = g.pwmRegPtr[PWM_STA];
    if ((r & ((1 << 10) | (1 << 9))) == 0) {
      return;
    }
    usleep(100);
  }
  fatal("pwmDisable - Timeout, PWM_STA=0x%08x", r);
}

static void
pwmReset(void)
{
  SCU PWM_BERR  = 8;
  SCU PWM_GAP02 = 5;
  SCU PWM_GAP01 = 4;
  SCU PWM_RERR  = 3;
  SCU PWM_WERR  = 2;
  SCU ErrMask =
    (1 << PWM_BERR)  |
    (1 << PWM_GAP02) |
    (1 << PWM_GAP01) |
    (1 << PWM_RERR)  |
    (1 << PWM_WERR);
  g.pwmRegPtr[PWM_STA] = ErrMask;
  UInt32 r;
  
  usleep(MS);
  r = g.pwmRegPtr[PWM_STA];
  epf("pwmReset PWM_STA = 0x%08x", r);
  
  usleep(MS);
  
  //  g.pwmRegPtr[PWM_STA] = 0x12345678;
  
  r = g.pwmRegPtr[PWM_STA];
  epf("pwmReset PWM_STA = 0x%08x, ErrMask=0x%08x", r, ErrMask);
  
  Assert((r & ErrMask) == 0);
  pwmDisable();
  r = g.pwmRegPtr[PWM_STA];
  if (r != 2) {
    epf("pwmRegPtr[PWM_STA] = 0x%04x", r);
    epf("pwmRegPtr[PWM_STA] should be 2");
  }
  // Assert(r == 2);
  g.pwmRegPtr[PWM_DAT_A] = 0;
  g.pwmRegPtr[PWM_DAT_B] = 0;
}

static void
clockKill(void)
{
  epf("Killing clock");
  g.clkRegPtr[PWMCLK_CNTL] = 0x5a000000 | (1 << 5) | (1 << 0);
  usleep(MS);
  g.clkRegPtr[PWMCLK_CNTL] = 0x5a000000 | (0 << 5) | (1 << 0);
}

static void
clockDisable(void)
{
  g.clkRegPtr[PWMCLK_CNTL] = 0x5a000001; // Turn OFF enable flag
  for (int i = 0; i < 1000; ++i) {
    usleep(MS);
    if ((g.clkRegPtr[PWMCLK_CNTL] & 0x80) == 0) {
      return;
    }
    clockKill();
  }
  fatal("clockDisable() failed");
}

static void
clockEnable(void)
{
  g.clkRegPtr[PWMCLK_CNTL] = 0x5a000011; // Set source to oscillator and enable clock.
  for (int i = 0; i < 1000; ++i) {
    usleep(MS);
    if ((g.clkRegPtr[PWMCLK_CNTL] & 0x80) != 0) {
      return;
    }
  }
  fatal("clockEnable() failed");
}

static void
gpioInit(void)
{
  UInt32 r;
  r = g.gpioRegPtr[GPFSEL1];
  r &= ~((7 << 6) | (7 << 9)); // Clr Alt funcs for pins 12 & 13
  r |=  ((4 << 6) | (4 << 9)); // PWM Alt funcs for pins 12 & 13
  g.gpioRegPtr[GPFSEL1] = r;
}

static void
gpioOff(void)
{
  UInt32 r;
  r = g.gpioRegPtr[GPFSEL1];
  r &= ~((7 << 6) | (7 << 9)); // Clr Alt funcs for pins 12 & 13
  r |=  ((1 << 6) | (1 << 9)); // Pins 12 & 13 to outputs
  g.gpioRegPtr[GPFSEL1] = r;
  g.gpioRegPtr[GPCLR0] = ((1 << 12) | (1 << 13)); // Pin 12 & 13 = Low
}

static void
checkRegs(int n)
{
  SCU DMA_DISABLED = (0 << 31);
  if (g.pwmRegPtr[PWM_DMAC] != (DMA_DISABLED | (7 << 8) | 7)) {
    epf("g.pwmRegPtr[PWM_DMAC] should be 0x%08x", (DMA_DISABLED | (7 << 8) | 7));
  }
  //  Assert(g.pwmRegPtr[PWM_DMAC] == (DMA_DISABLED | (7 << 8) | 7));
  if (g.pwmRegPtr[PWM_STA] != 2) {
    
    // 10- Channel 2 state
    // 9 - Channel 1 state
    // 8 - Bus Error
    // 7 - 0
    // 6 - 0
    // 5 - Channel 2 Gap Occured
    // 4 - Channel 1 Gap Occured
    // 3 - FIFO Read Error
    // 2 - FIFO Write Error
    // 1 - FIFO Empty
    // 0 - FIFO Full
    
    epf("PWM_STA=0x%08x, n=%d", g.pwmRegPtr[PWM_STA], n);
  }
  epf("PWM_STA=0x%08x, n=%d XX", g.pwmRegPtr[PWM_STA], n);
  //Assert(g.pwmRegPtr[PWM_STA] == 2);
}

//=================================================================
//  These are the functions exported to Python.

void
setPwmClockDiv(UInt32 div)
{
  Assert((div > 0) && (div <= 0xfff));
  clockDisable();
  // 54 = 54 MHz / 54 =  1 MHz
  //  3 = 54 MHz /  3 = 18 MHz
  //  2 = 54 MHz /  2 = 27 MHz
  
  //g.clkRegPtr[PWMCLK_DIV]  = PWM_CLK_PASSWORD | (54 << 12);

  g.clkRegPtr[PWMCLK_DIV]  = PWM_CLK_PASSWORD | (div << 12);
}

static void
pwmInit(void)
{
  epf("pwmInit()");
  //Assert(RPI4_PERI_BASE == bcm_host_get_peripheral_address());
  mapPeriphrialPtrs();
  clockDisable();
  setPwmClockDiv(3); // Set to 54/3 = 18 MHz
  clockEnable();
  pwmReset();
  checkRegs(1);
  gpioInit();
  checkRegs(2);
  epf("PWM_CTL= 0x%08x", g.pwmRegPtr[PWM_CTL]);
}

static void
pwmSet(UInt32 redFreqHz, UInt32 redDutyCyclePercent,
       UInt32 bluFreqHz, UInt32 bluDutyCyclePercent)
{
  UInt32 freq;
  switch (g.modelNum) {
  case RPi_3: freq = 18000000; break;
  case RPi_4: freq = 18000000; break;
  default:    fatal("Unsupported RPi model: %d", g.modelNum);
  }
  UInt32 rangeRed = (redFreqHz == 0) ? 0: freq / redFreqHz;
  UInt32 rangeBlu = (bluFreqHz == 0) ? 0: freq / bluFreqHz;
  UInt32 resetRed = rangeRed * redDutyCyclePercent / 100;
  UInt32 resetBlu = rangeBlu * bluDutyCyclePercent / 100;

  // epf("pwmSet(%d,%d,%d,%d)", rangeRed, rangeBlu, resetRed, resetBlu);
  
  g.pwmRegPtr[PWM_DAT_A] = resetRed;
  g.pwmRegPtr[PWM_RNG_A] = rangeRed;;
  g.pwmRegPtr[PWM_DAT_B] = resetBlu;
  g.pwmRegPtr[PWM_RNG_B] = rangeBlu;
  pwmEnable();
}

static void pwmOff(void)
  __attribute__ ((unused));

static void
pwmOff(void)
{
  if (g.gpioRegPtr == NULL) {
    Assert(g.clkRegPtr == NULL);
    Assert(g.pwmRegPtr == NULL);
    mapPeriphrialPtrs();
  }
  Assert(g.gpioRegPtr != NULL);
  Assert(g.clkRegPtr != NULL);
  Assert(g.pwmRegPtr != NULL);
  gpioOff();
  pwmReset();
  clockDisable();
  checkRegs(5);
}
