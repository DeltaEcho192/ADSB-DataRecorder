#!/bin/bash
 sudo apt-get update
 sudo apt-get upgrade
 sudo printf 'blacklist dvb_usb_rtl28xxu\nblacklist rtl2832\nblacklist rtl2830' > /etc/modprobe.d/nortl.conf
 sudo apt-get install git-core
 sudo apt-get install git
 sudo apt-get install cmake
 sudo apt-get install libusb-1.0-0-dev  
 sudo apt-get install build-essential 
