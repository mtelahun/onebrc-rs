use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::arraystring128::{ArrayString128, MAX_STRING_LEN};

pub struct StationMeasurements {
    file: File,
    lines: HashMap<ArrayString128, f64>,
}

impl StationMeasurements {
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        Ok(Self {
            file: File::open(path)?,
            lines: HashMap::new(),
        })
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
                        continue
                    },
                };
                let temp: f64 = match temp.trim().parse() {
                    Ok(t) => t,
                    Err(_) => {
                        buf.clear();
                        continue
                    },
                };
                self.lines.insert(city, temp);
                buf.clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io, str::FromStr};

    use crate::{arraystring128::ArrayString128, station_measurements::StationMeasurements};

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
        let stats = measurements.get_stats(ArrayString128::from_str("Hamburg"));

        // Assert
        assert_eq!(
            format!("{}", stats),
            "12.00/29.42/42.55"
        );
    }
}
