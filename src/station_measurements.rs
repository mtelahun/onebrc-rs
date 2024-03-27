use std::{
    collections::{hash_map, HashMap},
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::arraystring128::{ArrayString128, MAX_STRING_LEN};

#[derive(Debug)]
pub struct StationMeasurements {
    file: File,
    lines: HashMap<ArrayString128, TemperatureStats>,
}

#[derive(Copy, Clone, Debug)]
pub struct TemperatureStats {
    min: f64,
    max: f64,
    sum: f64,
    count: i32,
}

#[derive(Clone, Debug)]
pub struct CityTemperatureStats<'a> {
    inner: Vec<(&'a ArrayString128, &'a TemperatureStats)>,
}

impl StationMeasurements {
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        Ok(Self {
            file: File::open(path)?,
            lines: HashMap::new(),
        })
    }

    pub fn get_all_stats(&self) -> CityTemperatureStats {
        let mut sorted: Vec<(&ArrayString128, &TemperatureStats)> = self.lines.iter().collect();
        sorted.sort_by_key(|k| k.0);

        CityTemperatureStats { inner: sorted }
    }

    pub fn get_stat(&self, city: &ArrayString128) -> TemperatureStats {
        if let Some(current_value) = self.lines.get(city) {
            *current_value
        } else {
            TemperatureStats {
                min: 0_f64,
                max: 0_f64,
                sum: 0_f64,
                count: 1,
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn read_lines(&mut self) {
        let mut buf = String::with_capacity(MAX_STRING_LEN);
        let mut reader = BufReader::new(&mut self.file);
        while let Ok(bytes) = reader.read_line(&mut buf) {
            if bytes == 0 {
                break;
            }
            if let Some((city, temp)) = buf.split_once(';') {
                let city: ArrayString128 = match city.parse() {
                    Ok(c) => c,
                    Err(_) => {
                        buf.clear();
                        continue;
                    }
                };
                let temp: f64 = match temp.trim().parse() {
                    Ok(t) => t,
                    Err(_) => {
                        buf.clear();
                        continue;
                    }
                };
                if let hash_map::Entry::Vacant(e) = self.lines.entry(city) {
                    let new_value = TemperatureStats {
                        min: temp,
                        max: temp,
                        sum: temp,
                        count: 1,
                    };
                    e.insert(new_value);
                } else {
                    let current_stats = self.lines.get_mut(&city).unwrap();
                    current_stats.min = current_stats.min.min(temp);
                    current_stats.max = current_stats.max.max(temp);
                    current_stats.sum += temp;
                    current_stats.count += 1;
                }
                buf.clear();
            }
        }
    }
}

impl CityTemperatureStats<'_> {
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl std::fmt::Display for TemperatureStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let avg = if self.count > 0 {
            self.sum / (self.count as f64)
        } else {
            0f64
        };
        write!(f, "{:.2}/{:.2}/{:.2}", self.min, avg, self.max)
    }
}

impl std::fmt::Display for CityTemperatureStats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg_inner = String::new();
        for (city, stat) in self.inner.iter() {
            if !msg_inner.is_empty() {
                msg_inner = format!("{}, {}={}", msg_inner, **city, **stat);
            } else {
                msg_inner = format!("{}={}", **city, **stat);
            }
        }
        write!(f, "{{{}}}", msg_inner)
    }
}

#[cfg(test)]
mod tests {
    use std::{io, str::FromStr};

    use crate::{arraystring128::ArrayString128, station_measurements::StationMeasurements};

    use super::CityTemperatureStats;

    #[test]
    fn given_nonexistent_file_return_error() {
        // Act
        let measurements = StationMeasurements::from_file("./nonexistent");

        // Assert
        assert!(measurements.is_err());
        assert_eq!(
            io::ErrorKind::NotFound,
            measurements.err().unwrap().kind(),
            "failed to open measurements file"
        );
    }

    #[test]
    fn given_file_when_line_starts_with_hash_then_ignore() {
        // Arrange
        let path = "./data/test01.csv";
        let mut measurements =
            StationMeasurements::from_file(path).expect("failed to open measurements file");

        // Act
        measurements.read_lines();

        // Assert
        assert!(measurements.is_empty());
    }

    #[test]
    fn given_file_with_ten_measurements_when_read_lines_then_length_is_10() {
        // Arrange
        let path = "./data/test02.csv";
        let mut measurements =
            StationMeasurements::from_file(path).expect("failed to open measurements file");

        // Act
        measurements.read_lines();

        // Assert
        assert_eq!(measurements.len(), 10, "There should be 10 measurements");
    }

    #[test]
    fn given_file_with_measurements_when_get_city_then_return_statistics() {
        // Arrange
        let path = "./data/test03.csv";
        let mut measurements =
            StationMeasurements::from_file(path).expect("failed to open measurements file");
        measurements.read_lines();

        // Act
        let stats = measurements.get_stat(&ArrayString128::from_str("Hamburg").unwrap());

        // Assert
        assert_eq!("12.00/29.42/42.55", format!("{}", stats),);
    }

    #[test]
    fn given_file_with_measurements_when_get_all_stats_then_return_all_statistics() {
        // Arrange
        let path = "./data/test03.csv";
        let mut measurements =
            StationMeasurements::from_file(path).expect("failed to open measurements file");
        measurements.read_lines();

        // Act
        let stats = measurements.get_all_stats();
        eprintln!("stats: {stats}");

        // Assert
        assert_eq!(1, stats.len(), "one record in statistics list");
        assert_eq!("{Hamburg=12.00/29.42/42.55}", format!("{}", stats),);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_CityTemperatureStats_then_is_empty() {
        // Act
        let agg = CityTemperatureStats { inner: Vec::new() };

        // Assert
        assert!(agg.is_empty());
    }
}
