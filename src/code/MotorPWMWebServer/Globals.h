//
//  Globals.h
//

typedef uint8_t  UInt8;
typedef uint16_t UInt16;
typedef uint32_t UInt32;
typedef uint64_t UInt64;
typedef unsigned int UInt;

typedef  int8_t  SInt8;
typedef  int16_t SInt16;
typedef  int32_t SInt32;
typedef  int64_t SInt64;
typedef  signed int SInt;

static struct {
  volatile UInt32 *gpioRegPtr;
  volatile UInt32 *pwmRegPtr;
  volatile UInt32 *clkRegPtr;
  UInt modelNum;
  struct {
    SInt32 dutyCycle_Red;
    SInt32 freqHz_Red;
    SInt32 dutyCycle_Blue;
    SInt32 freqHz_Blue;
  } flags;
} g;

#define NumItems(x)  ((UInt)(sizeof(x)/sizeof(*(x))))

#define Assert(x)  xAssert(x, #x, __FILE__, __LINE__)

#define RPI4_PERI_BASE	   0xfe000000
#define RPI3_PERI_BASE	   0x3f000000
//#define RPI3_PERI_BASE	   0x20000000

#define GPIO_BASE_OFFSET  0x200000
#define PWM_BASE_OFFSET   0x20c000
#define CLK_BASE_OFFSET   0x101000

#define CLOCK_RATE 54000000.0
#define MAX_FREQ   18000000

#define PWM_CLK_PASSWORD 0x5a000000
#define BCM2835_PWM_CONTROL 0
#define BCM2835_PWM_STATUS  1
#define BCM2835_PWM0_RANGE  4
#define BCM2835_PWM0_DATA   5

#define	PWMCLK_CNTL 40
#define	PWMCLK_DIV  41
#define BLOCK_SIZE 	(4*1024)
#define SCU static const UInt32
  
// Bits in PWM_CTL - Page 127, BCM2711 ARM Peripherals Manual
#define MSEN_B  15
// Bit 14 is unused.
#define USEF_B  13
#define POLA_B  12
#define SBIT_B  11
#define RPTL_B  10
#define MODE_B  9
#define PWEN_B  8
#define MSEN_A  7
#define CLRF    6
#define USEF_A  5
#define POLA_A  4
#define SBIT_A  3
#define RPTL_A  2
#define MODE_A  1
#define PWEN_A  0

SCU PWM_CTL_RESET = // Bit mask to reset the PWM_CTL register
    (1 << MSEN_B) | // 15 : 0 = PWM smoothing is used. 1 = M/S is used.
    (0 << 14)     | // 14 : Bit 14 is not used.
    (0 << USEF_B) | // 13 : 0 = Don't use Fifo for transmitting
    (0 << POLA_B) | // 12 : 0 = Normal polarity
    (0 << SBIT_B) | // 11 : 0=Low, 1=high when not transmitting
    (0 << RPTL_B) | // 10 : Repeat fifo
    (0 << MODE_B) | // 9  : 0 = PWM mode. 1 = Serializer mode
    (0 << PWEN_B) | // 8  : 0 = Disable PWM channel B
    (1 << MSEN_A) | // 7  : 0 = PWM smoothing is used. 1 = M/S is used.
    (1 << CLRF)   | // 6  : 1 = Clear Fifo.  Write only.
    (0 << USEF_A) | // 5  : 0 = Don't use Fifo for transmitting
    (0 << POLA_A) | // 4  : 0 = Normal polarity
    (0 << SBIT_A) | // 3  : 0=Low, 1=high when not transmitting
    (0 << RPTL_A) | // 2  : Repeat fifo
    (0 << MODE_A) | // 1  : 0 = PWM mode. 1 = Serializer mode
    (0 << PWEN_A);  // 0  : 1 = Enable PWM channel A

// Bits in PWM_STA - Page 129
#define STA2  10 // Channel 2 state
#define STA1   9 // Channel 1 state
#define BERR   8 // Bus Error Flag - W1C (Write 1 to Clear)
//               // Bit 7 is unused
//               // Bit 6 is unused
#define CAP02  5 // Channel 2 Gap Occurred - W1C
#define CAP01  4 // Channel 1 Gap Occurred - W1C
#define RERR1  3 // FIFO Read Error Flag   - W1C
#define WERR1  2 // FIFO Write Error Flag  - W1C
#define EMPT1  1 // FIFO Write Error Flag
#define FULL1  0 // FIFO Write Error Flag

SCU PWM_STA_RESET = // Bit mask to reset the PWM_STA register
  (1 << BERR) |
  (1 << CAP01) |
  (1 << CAP02) |
  (1 << RERR1) |
  (1 << WERR1);

#define GX(x) ((x) / sizeof(UInt32))
#define PX(x) ((x) / sizeof(UInt32))

SCU MS        = 20;  
SCU GPFSEL1   = GX(0x04);
SCU GPCLR0    = GX(0x28);
SCU PWM_CTL   = PX(0x00);
SCU PWM_STA   = PX(0x04);
SCU PWM_DMAC  = PX(0x08);
SCU PWM_RNG_A = PX(0x10);
SCU PWM_DAT_A = PX(0x14);
SCU PWM_RNG_B = PX(0x20);
SCU PWM_DAT_B = PX(0x24);

#define RPi_Z 'Z'
#define RPi_3 '3'
#define RPi_4 '4'
static UInt rPiModelNumber(void);
