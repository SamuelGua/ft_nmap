use nmap::parser::parameters::Parameters;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_ip_format() {
        let args = vec!["--ip".to_string(), "127.1.1.1.d".to_string()].into_iter();

        let result = Parameters::new_argv(args);

        assert!(result.is_err());
    }

    #[test]
    fn test_mutually_exclusive_ip_and_file() {
        let args = vec![
            "--ip".to_string(), "127.0.0.1".to_string(),
            "--file".to_string(), "targets.txt".to_string(),
        ].into_iter();

        let result = Parameters::new_argv(args);
        assert!(result.is_err());
    }
}