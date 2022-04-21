#!bin/sh
echo "Starting up Flight Recorder"

echo "Starting Mysql"
service mysqld start

echo "Starting Dump1090"
cd /Users/radio/home/dump1090/
./dump1090 --interactive --net

echo "Starting Listening program"
cd /Users/radio/home/adsb_database
cargo build --release
cargo run --release