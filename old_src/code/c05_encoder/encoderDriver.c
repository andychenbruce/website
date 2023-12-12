//
//  encoderDriver.c
//
//  Encoder device driver for Farmbot.
//

#include <linux/module.h>
#include <linux/interrupt.h>
#include <linux/gpio.h>
#include <linux/mm.h>

#include "encoderDriver.h"

//====================================================================

_Static_assert((PAGE_SIZE == 4096), "PAGE_SIZE == 4096");

#define RPI4_PERI_BASE	0xfe000000
#define GPIO_BASE	(RPI4_PERI_BASE + 0x200000)
#define GPIO_SIZE	(1024 * 4)

#define GPIO_21 21
#define GPIO_22 22
#define GPIO_23 23
#define GPIO_24 24
#define GPIO_25 25

#define GX(x) ((x) / 4)
#define GPFSEL0 GX(0x00)
#define GPFSEL1 GX(0x04)
#define GPFSEL2 GX(0x08)
#define GPSET0  GX(0x1c)
#define GPCLR0  GX(0x28)
#define GPLEV0  GX(0x34)
#define GPEDS0  GX(0x40)

static const char DeviceName[] = DEVICE_NAME;

static struct {
  struct page *pagePtr;
  volatile EncoderInfo *ep;
  volatile UInt32 *gpioRegPtr;
  int device_file_major_number;
  int irqNum22;
  int irqNum23;
  int irqNum24;
  int irqNum25;
} g;

static irqreturn_t
irqA(int x)
{
  UInt32 pins = g.gpioRegPtr[GPLEV0];
  UInt8 d = 1 & ((pins >> 22) ^ (pins >> 23));
  g.ep->encoderCountA += (d ? -x : x);
  //printk(KERN_NOTICE "IrqA pins=%08x, x=%d, d=%d, cnt=%d\n", pins, x, d, g.ep->encoderCountA);
  return IRQ_HANDLED;
}

static irqreturn_t
irqB(int x)
{
  UInt32 pins = g.gpioRegPtr[GPLEV0];
  UInt8 d = 1 & ((pins >> 24) ^ (pins >> 25));
  g.ep->encoderCountB += (d ? -x : x);
  //printk(KERN_NOTICE "IrqB pins=%08x, x=%d, d=%d, cnt=%d\n", pins, x, d, g.ep->encoderCountB);
  return IRQ_HANDLED;
}

static irqreturn_t gpio_22_irq(int irq, void *dev_id) { return irqA(1);  }
static irqreturn_t gpio_23_irq(int irq, void *dev_id) { return irqA(-1); }
static irqreturn_t gpio_24_irq(int irq, void *dev_id) { return irqB(1);  }
static irqreturn_t gpio_25_irq(int irq, void *dev_id) { return irqB(-1); }

//====================================================================

static ssize_t
encoder_mmap(struct file *file_ptr,
	     struct vm_area_struct *vma)
{
  int len = vma->vm_end - vma->vm_start;
  int prot = vma->vm_page_prot;
  UInt32 pfn = page_to_pfn(g.pagePtr);
  if (len != PAGE_SIZE) {
    printk(KERN_NOTICE "encoder_mmap: Size must be %d.\n", (int) PAGE_SIZE);
    return -EIO;
  }
  if (remap_pfn_range(vma, vma->vm_start, pfn, PAGE_SIZE, prot)) {
    return -EAGAIN;
  }
  return 0;
}

int
encoder_open(struct inode *inode, struct file *file_ptr)
{
  memset((void *) g.ep, 0, PAGE_SIZE);
  printk(KERN_NOTICE "encoder_open, version=%d\n", ENCODER_VERSION);
  g.ep->magic   = ENCODER_MAGIC;
  g.ep->version = ENCODER_VERSION;
  return 0;
}

//====================================================================

static const struct
file_operations encoder_driver_fops = {
 .owner = THIS_MODULE,
 //.read = encoder_read,
 .open = encoder_open,
 .mmap = encoder_mmap
};

//====================================================================

static int
register_device(void)
{
  int result = 0;
  printk(KERN_NOTICE "%s: register_device() is called.\n", DeviceName);
  result = register_chrdev( 0, DeviceName, &encoder_driver_fops );
  if (result < 0 ) {
    printk(KERN_WARNING "%s: Can\'t register device, errorcode = %i\n",
	   DeviceName,
	   result);
    return result;
  }
  g.device_file_major_number = result;
  printk(KERN_NOTICE "%s: Reg char dev, major number = %i, minor numbers 0...255\n",
	 DeviceName,
	 g.device_file_major_number );
  return 0;
}

//====================================================================

static void
unregister_device(void)
{
  printk(KERN_NOTICE "%s: unregister_device() is called\n", DeviceName);
  if(g.device_file_major_number != 0) {
    unregister_chrdev(g.device_file_major_number, DeviceName);
  }
}

//====================================================================

MODULE_DESCRIPTION(DEVICE_NAME);
MODULE_LICENSE("GPL");
MODULE_AUTHOR("Bob");

static int
pinInit(int pin, irqreturn_t (*gpio_irq)(int, void *))
{
  int irqNum;
  int rv;
  
  if (! gpio_is_valid(pin)) {
    printk(KERN_NOTICE "Encoder GPIO pin %d not valid\n", pin);
    //encoderError(ERR_NOT_VALID, pin);
    return 0;
  }
  if (gpio_request(pin, "Encoder pinInit()") < 0) {
    printk(KERN_NOTICE "Encoder GPIO request denied, pin %d\n", pin);
    //encoderError(ERR_GPIO_REQ_DENIED, pin);
    return 0;
  }
  gpio_direction_input(pin);
  irqNum = gpio_to_irq(pin);
  rv = request_irq(irqNum,
		   (void *) gpio_irq,
		   IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING,
		   DeviceName,
		   NULL);
  if (rv != 0) {
    printk(KERN_NOTICE "%s: request_irq(%d) failed (%d)\n", DeviceName, pin, rv);
    //encoderError(ERR_IRQ_REQ_DENIED, pin);
    return 0;
  }
  return irqNum;
}

static int
encoder_driver_init(void)
{
  printk( KERN_NOTICE "%s: encoder_driver_init\n", DeviceName);
  g.gpioRegPtr = (volatile UInt32 *) ioremap(GPIO_BASE, GPIO_SIZE);
  if (g.gpioRegPtr == NULL) {
    printk(KERN_NOTICE "%s: ioremap(GPIO_BASE) failed.\n", DeviceName);
    return -1;
  }
  /*
  if (turnOnPin21() != 0) {
    return -EIO;
  }
  */
  g.pagePtr = alloc_page(GFP_KERNEL);
  if (g.pagePtr == NULL) {
    printk(KERN_NOTICE "%s: page alloc failed\n", DeviceName);
    return -EIO;
  }
  if ((((UInt32) g.pagePtr) & 0xfff) != 0) {
    printk(KERN_NOTICE "%s: g.pagePtr is not page aligned\n", DeviceName);
  }
  SetPageReserved(g.pagePtr); // Don't swap
  g.ep = (EncoderInfo *) page_address(g.pagePtr); // Get the kernel virtual address
  if ((((UInt32) g.ep) & 0xfff) != 0) {
    printk( KERN_NOTICE "%s: g.ep is not page aligned\n", DeviceName);
  }
  memset((void *) g.ep, 0, PAGE_SIZE);
  g.irqNum22 = pinInit(GPIO_22, gpio_22_irq);
  g.irqNum23 = pinInit(GPIO_23, gpio_23_irq);
  g.irqNum24 = pinInit(GPIO_24, gpio_24_irq);
  g.irqNum25 = pinInit(GPIO_25, gpio_25_irq);
  return register_device();
}

static void
encoder_driver_exit(void)
{
  printk( KERN_NOTICE "%s: encoder_driver_exit\n", DeviceName);
  free_irq(g.irqNum22, NULL);
  free_irq(g.irqNum23, NULL);
  free_irq(g.irqNum24, NULL);
  free_irq(g.irqNum25, NULL);
  gpio_free(GPIO_22);
  gpio_free(GPIO_23);
  gpio_free(GPIO_24);
  gpio_free(GPIO_25);
  ClearPageReserved(g.pagePtr);
  put_page(g.pagePtr);
  iounmap(g.gpioRegPtr);
  unregister_device();
}

module_init(encoder_driver_init);
module_exit(encoder_driver_exit);

//====================================================================
