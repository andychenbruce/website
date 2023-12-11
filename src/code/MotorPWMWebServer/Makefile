#
#	Makefile
#

CC	:= gcc

CFLAGS	:= -O3
CFLAGS	+= -Wall -Wextra -Werror

SRCS	:= server.c
SRCS	+= utils.c
SRCS	+= socket.c
SRCS	+= rpiModel.c
SRCS	+= rpiMemMap.c
SRCS	+= rpiPwm.c

server:	${SRCS}
	${CC} ${CFLAGS} -o $@ $<

clean:
	rm -f server *~
