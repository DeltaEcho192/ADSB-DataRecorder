#!bin/sh
echo "Starting up Flight Recorder"

echo "Starting Mysql"
service mysqld start

echo "Starting Dump1090"
cd /home/radio/home/dump1090/
./dump1090 --interactive --net

echo "Starting Listening program"
cd /home/radio/home/ADSB-DataRecorder
cargo build --release
cargo run --release