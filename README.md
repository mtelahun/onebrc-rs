# onebrc-rs
![Rust](https://github.com/mtelahun/onebrc-rs/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/gh/mtelahun/onebrc-rs/branch/main/graph/badge.svg?token=A1P9I5E2LU)](https://codecov.io/gh/mtelahun/onebrc-rs)
[![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

## 1 Billion Row Challenge - in Rust

The original challenge: https://github.com/gunnarmorling/1brc

### Instructions
Clone the [original repo](https://github.com/gunnarmorling/1brc) and follow the instructions in the README to generate the input file with the 1 billion rows. Then build this crate:
```
cargo build --release
```

then run and time this crate:
```
/usr/bin/time cargo run --release -- <path/to/input/file>
```
The majority of the program run-time is spent reading the input. After the OS reads the input file for the first time large parts of it will be cached in memory. Run the release binary a second time on the input file to take advantage of this (and reduce your program run-time).

To generate a flamegraph:
```
cargo flamegraph -- <path/to/input/file>
```

The text file [weather_stations.csv](./data/weather_stations.csv) contains sample temperature values for a range of weather stations. Each row is one measurement in the format <string: station name>;<double: measurement>, with the measurement value having exactly one fractional digit. The following shows ten rows as an example:
```
Hamburg;12.0
Bulawayo;8.9
Palembang;38.8
St. John's;15.2
Cracow;12.6
Bridgetown;26.9
Istanbul;6.2
Roseau;34.4
Conakry;31.2
Istanbul;23.0
```

The task is to write a [Rust] program which reads the input file, calculates the min, mean, and max temperature value per weather station, and emits the results on stdout like this (i.e. sorted alphabetically by station name, and the result values per station in the format <min>/<mean>/<max>, rounded to one fractional digit):

```
{Abha=-23.0/18.0/59.2, Abidjan=-16.2/26.0/67.3, Abéché=-10.0/29.4/69.0, Accra=-10.1/26.4/66.4, Addis Ababa=-23.7/16.0/67.0, Adelaide=-27.8/17.3/58.5, ...}
```

## This implementation
My implementation is in idiomatic rust with the only optimization being the use of `ArrayString` from the `arrayvec` crate to reduce String allocation on the heap. On my 16GB RAM 4-core Intel(R) Core(TM) i7-10510U CPU @ 1.80GHz workstation this program takes `156 sec` to run. The Java reference implementation from the original repo takes `251 sec.`