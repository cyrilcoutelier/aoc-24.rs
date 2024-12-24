mod process_lines {
    use aoc2024::solver::process_lines;

    #[test]
    fn puzzle_subject_example() {
        // When
        let input = &[
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ];

        // When
        let result = process_lines(input.iter().map(|x| x.to_string()));

        // Then
        assert_eq!(result, "4");
    }
}

mod report {
    mod is_safe {
        use aoc2024::solver::Report;

        #[test]
        fn puzzle_subject_example_1_3_2_4_5_safe() {
            // Given
            let report = Report::from("1 3 2 4 5");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn puzzle_subject_example_8_6_4_4_1_safe() {
            // Given
            let report = Report::from("8 6 4 4 1");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn puzzle_subject_example_12_10_13_16_19_21_22_safe() {
            // Given
            let report = Report::from("12 10 13 16 19 21 22");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn puzzle_subject_example_9_2_3_4_5_safe() {
            // Given
            let report = Report::from("9 2 3 4 5");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn puzzle_subject_example_1_2_3_4_9_safe() {
            // Given
            let report = Report::from("1 2 3 4 9");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn puzzle_subject_example_57_56_57_59_60_63_64_65_safe() {
            // Given
            let report = Report::from("57 56 57 59 60 63 64 65");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn puzzle_subject_example_91_92_95_93_94_safe() {
            // Given
            let report = Report::from("91 92 95 93 94");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn puzzle_subject_example_16_13_15_13_12_11_9_6_safe() {
            // Given
            let report = Report::from("16 13 15 13 12 11 9 6");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn puzzle_subject_example_40_41_43_44_47_46_47_49_safe() {
            // Given
            let report = Report::from("40 41 43 44 47 46 47 49");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn puzzle_subject_example_53_55_56_59_62_61_65_safe() {
            // Given
            let report = Report::from("53 55 56 59 62 61 65");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn puzzle_subject_example_86_85_84_81_80_81_77_safe() {
            // Given
            let report = Report::from("86 85 84 81 80 81 77");

            // When
            let result = report.is_safe();

            // Then
            assert_eq!(result, true);
        }
    }
}
