fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::io;

    #[test]
    fn given_nonexistent_file_return_error() {
        // Act
        let measurements = StationMeasurements::from_file("./nonexistent");

        // Assert
        assert!(measurements.is_err());
        assert_eq!(
            Err(io::ErrorKind::NotFound),
            measurements,
            "failed to open measurements file"
        );
    }
}