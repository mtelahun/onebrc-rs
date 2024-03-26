use std::{fs::File, io};

pub struct StationMeasurements {}

impl StationMeasurements {
    pub fn from_file(path: &str) -> Result<File, io::Error> {
        File::open(path)
    }
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
}