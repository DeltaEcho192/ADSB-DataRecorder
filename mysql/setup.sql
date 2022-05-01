CREATE DATABASE IF NOT EXISTS flight_data;
USE flight_data;
CREATE TABLE IF NOT EXISTS flight_prod (hex varchar(255),flight varchar(255),lat float,lon float,altitude Integer,track Integer, speed Integer,entryTime datetime default current_timestamp);