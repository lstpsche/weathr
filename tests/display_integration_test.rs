use weathr::display::AsciiDisplay;
use weathr::scene::house::House;

#[test]
fn test_display_integration_house_rendering() {
    let house = House::default();
    let ascii = house.get_ascii();

    assert!(!ascii.is_empty(), "House should not be empty");

    assert!(ascii.len() >= 7, "House should have at least 7 lines");

    let house_str = ascii.join("\n");
    // Updated house design checks
    assert!(
        house_str.contains("___"),
        "House should contain roof/structure"
    );
    assert!(house_str.contains("|"), "House should contain walls");
    // New design uses . and ' for roof curves instead of / \
    assert!(house_str.contains("."), "House should contain roof details");
}

#[test]
fn test_display_integration_weather_info_formatting() {
    let test_cases = vec![
        (52.52, 13.41, "Berlin"),
        (40.7128, -74.0060, "New York"),
        (-33.8688, 151.2093, "Sydney"),
        (35.6762, 139.6503, "Tokyo"),
        (0.0, 0.0, "Null Island"),
    ];

    for (lat, lon, location_name) in test_cases {
        let info = AsciiDisplay::format_weather_info(lat, lon);

        assert!(
            !info.is_empty(),
            "Weather info should not be empty for {}",
            location_name
        );
        assert!(
            info.contains("Weather for:"),
            "Should contain 'Weather for:' for {}",
            location_name
        );
        assert!(
            info.contains("°N"),
            "Should contain latitude marker for {}",
            location_name
        );
        assert!(
            info.contains("°E"),
            "Should contain longitude marker for {}",
            location_name
        );
        assert!(
            info.contains("Press 'q' to quit"),
            "Should contain quit instruction for {}",
            location_name
        );
    }
}

#[test]
fn test_display_integration_coordinate_precision() {
    let info = AsciiDisplay::format_weather_info(52.123456789, 13.987654321);

    assert!(
        info.contains("52.12"),
        "Should round latitude to 2 decimal places"
    );
    assert!(
        info.contains("13.99"),
        "Should round longitude to 2 decimal places"
    );
}

#[test]
fn test_display_integration_extreme_coordinates() {
    let info_north_pole = AsciiDisplay::format_weather_info(90.0, 0.0);
    assert!(info_north_pole.contains("90.00"));

    let info_south_pole = AsciiDisplay::format_weather_info(-90.0, 0.0);
    assert!(info_south_pole.contains("-90.00"));

    let info_date_line = AsciiDisplay::format_weather_info(0.0, 180.0);
    assert!(info_date_line.contains("180.00"));
}

#[test]
fn test_display_integration_house_consistency() {
    let house1 = House::default();
    let house2 = House::default();

    assert_eq!(
        house1.get_ascii(),
        house2.get_ascii(),
        "House rendering should be consistent"
    );
}
