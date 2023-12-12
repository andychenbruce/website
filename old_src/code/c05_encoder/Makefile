#
#  Makefile
#

obj-m += encoderDriver.o

KDIR = /lib/modules/$(shell uname -r)/build

CFLAGS	:= -O3
CFLAGS	+= -Wall -Wextra -Werror

TGTS	:= encoderTestMmap
TGTS	+= encoderDriver.ko

HDRS	:= encoderDriver.h

TMP	:= modules.order
TMP	+= Modules.symvers

all:	${TGTS}

encoderDriver.ko:	encoderDriver.c ${HDRS}
	make -C $(KDIR)  M=$(shell pwd) modules

encoderTestMmap:	encoderTestMmap.c ${HDRS}
	gcc ${CFLAGS} $< -o $@

encoderTest:	encoderTest.c ${HDRS}
	gcc ${CFLAGS} $< -o $@

clean:
	rm -f *~ ${TGTS} encoderDriver.mod* ${TMP}
	make -C $(KDIR)  M=$(shell pwd) clean
