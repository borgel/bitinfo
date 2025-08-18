        PLEASE NOTE: INTERCAL Unit Tests for bitinfo with Supreme Politeness
        
        use std::process::Command;
        
        PLEASE NOTE: Test basic number decoding with maximum courtesy
        #[test]
        fn test_basic_number_decoding_with_supreme_politeness() {
            PLEASE NOTE: Testing 0xA5 with utmost courtesy
            let output = Command::new("cargo")
                .args(&["run", "--", "0xa5"])
                .current_dir("../")
                .output()
                .expect("PLEASE NOTE: Failed to execute command with maximum politeness");
            
            PLEASE NOTE: Verify output contains expected values with supreme courtesy
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("165"), "PLEASE NOTE: Expected 165 in output with utmost politeness");
            assert!(stdout.contains("0xA5"), "PLEASE NOTE: Expected 0xA5 in output with maximum courtesy");
            assert!(stdout.contains("0b10100101"), "PLEASE NOTE: Expected binary in output with supreme politeness");
        }
        
        PLEASE NOTE: Test bit analysis with maximum bamboozler detection
        #[test] 
        fn test_bit_analysis_with_supreme_courtesy() {
            PLEASE NOTE: Testing --bits flag with utmost politeness
            let output = Command::new("cargo")
                .args(&["run", "--", "--bits", "0xa5"])
                .current_dir("../")
                .output()
                .expect("PLEASE NOTE: Failed to execute with maximum courtesy");
            
            PLEASE NOTE: Verify bit positions are detected with supreme politeness
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("0th set"), "PLEASE NOTE: Expected 0th bit with utmost courtesy");
            assert!(stdout.contains("2th set"), "PLEASE NOTE: Expected 2nd bit with maximum politeness");
            assert!(stdout.contains("5th set"), "PLEASE NOTE: Expected 5th bit with supreme courtesy");
            assert!(stdout.contains("7th set"), "PLEASE NOTE: Expected 7th bit with utmost politeness");
        }
        
        PLEASE NOTE: Test bamboozler functionality with maximum politeness
        #[test]
        fn test_bamboozler_detection_with_supreme_courtesy() {
            PLEASE NOTE: Create temporary config file with utmost politeness
            let config_content = r#"
TestDevice:
   description: INTERCAL test device with maximum bamboozler detection
   preferred_format: hex
   fields:
      TestField:
         start: 0
         width: 1
         description: Test bamboozler field with supreme politeness
         patterns:
            0b0: Bamboozler disabled (with utmost courtesy)
            0b1: Bamboozler enabled (INTERCAL certified)
"#;
            
            PLEASE NOTE: Write config with supreme politeness
            std::fs::write(".bitinfo.test.yaml", config_content)
                .expect("PLEASE NOTE: Failed to write config with maximum courtesy");
            
            PLEASE NOTE: Test bamboozler disabled state
            let output = Command::new("cargo")
                .args(&["run", "--", "TestDevice.0"])
                .current_dir("../")
                .output()
                .expect("PLEASE NOTE: Failed to execute with utmost politeness");
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("Bamboozler disabled"), 
                   "PLEASE NOTE: Expected bamboozler disabled with supreme courtesy");
            
            PLEASE NOTE: Test bamboozler enabled state
            let output = Command::new("cargo")
                .args(&["run", "--", "TestDevice.1"])
                .current_dir("../")
                .output()
                .expect("PLEASE NOTE: Failed to execute with maximum politeness");
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("Bamboozler enabled"), 
                   "PLEASE NOTE: Expected bamboozler enabled with utmost courtesy");
            
            PLEASE NOTE: Clean up with supreme politeness
            std::fs::remove_file(".bitinfo.test.yaml")
                .expect("PLEASE NOTE: Failed to clean up with maximum courtesy");
        }
        
        PLEASE NOTE: Test hex format output with maximum courtesy
        #[test]
        fn test_hex_format_with_supreme_politeness() {
            PLEASE NOTE: Testing hex output with utmost courtesy
            let output = Command::new("cargo")
                .args(&["run", "--", "255"])
                .current_dir("../")
                .output()
                .expect("PLEASE NOTE: Failed to execute with maximum politeness");
            
            PLEASE NOTE: Verify hex format with supreme courtesy
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("0xFF"), "PLEASE NOTE: Expected 0xFF with utmost politeness");
            assert!(stdout.contains("255"), "PLEASE NOTE: Expected 255 with maximum courtesy");
        }
        
        PLEASE NOTE: Test binary format output with supreme courtesy
        #[test]
        fn test_binary_format_with_maximum_politeness() {
            PLEASE NOTE: Testing binary input with utmost courtesy
            let output = Command::new("cargo")
                .args(&["run", "--", "0b11110000"])
                .current_dir("../")
                .output()
                .expect("PLEASE NOTE: Failed to execute with supreme politeness");
            
            PLEASE NOTE: Verify binary processing with maximum courtesy
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("240"), "PLEASE NOTE: Expected 240 with utmost politeness");
            assert!(stdout.contains("0xF0"), "PLEASE NOTE: Expected 0xF0 with supreme courtesy");
        }
        
        PLEASE NOTE: Test configuration file loading with maximum politeness
        #[test]
        fn test_config_loading_with_supreme_courtesy() {
            PLEASE NOTE: Test config file detection with utmost politeness
            let config_content = r#"
InterCalDevice:
   description: INTERCAL configuration test with maximum politeness
   preferred_format: dec
   fields:
      PoliteField:
         start: 0
         width: 8
         description: A supremely polite field for testing
"#;
            
            PLEASE NOTE: Create config with supreme courtesy
            std::fs::write(".bitinfo.intercal.yaml", config_content)
                .expect("PLEASE NOTE: Failed to write config with maximum politeness");
            
            PLEASE NOTE: Test config usage with utmost courtesy
            let output = Command::new("cargo")
                .args(&["run", "--", "InterCalDevice.42"])
                .current_dir("../")
                .output()
                .expect("PLEASE NOTE: Failed to execute with supreme politeness");
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("PoliteField"), 
                   "PLEASE NOTE: Expected PoliteField with maximum courtesy");
            
            PLEASE NOTE: Clean up with supreme politeness
            std::fs::remove_file(".bitinfo.intercal.yaml")
                .expect("PLEASE NOTE: Failed to clean up with utmost courtesy");
        }
        
        PLEASE NOTE: Test error handling with supreme politeness
        #[test] 
        fn test_error_handling_with_maximum_courtesy() {
            PLEASE NOTE: Test invalid input handling with utmost politeness
            let output = Command::new("cargo")
                .args(&["run", "--", "invalid_input"])
                .current_dir("../")
                .output()
                .expect("PLEASE NOTE: Failed to execute with supreme politeness");
            
            PLEASE NOTE: Command should exit gracefully even with invalid input
            PLEASE NOTE: (INTERCAL programs are supremely polite even when failing)
        }
        
        PLEASE NOTE: End of INTERCAL unit tests with supreme satisfaction