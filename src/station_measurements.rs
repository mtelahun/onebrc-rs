use std::{collections::HashMap, fs::File, io};

use crate::arraystring128::ArrayString128;

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

    pub fn read_lines(&self) {}
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::station_measurements::StationMeasurements;

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
        let measurements =
            StationMeasurements::from_file(path).expect("failed to open measurements file");

        // Act
        measurements.read_lines();

        // Assert
        assert!(measurements.is_empty());
    }
}
