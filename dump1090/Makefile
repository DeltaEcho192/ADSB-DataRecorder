CFLAGS?=-O2 -g -Wall -W $(shell pkg-config --cflags librtlsdr)
LDLIBS+=$(shell pkg-config --libs librtlsdr) -lpthread -lm
CC?=gcc
PROGNAME=dump1090

all: dump1090

%.o: %.c
	gcc $(CFLAGS) -c $<

dump1090: dump1090.c anet.c
	gcc -g -o dump1090 dump1090.c anet.c $(LDFLAGS) $(LDLIBS)

clean:
	rm -f *.o dump1090
