mod extract_numbers {
    use aoc2024::solver::extract_numbers;

    #[test]
    fn only_digits_3_4() {
        // When
        let input = "3   4";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (3, 4));
    }

    #[test]
    fn numbers_85540_67702() {
        // When
        let input = "85540   67702";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (85540, 67702));
    }
}

mod process_lines {
    use aoc2024::solver::process_lines;

    #[test]
    fn puzzle_subject_example() {
        // When
        let input = &["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];

        // When
        let result = process_lines(input.iter().map(|x| x.to_string()));

        // Then
        assert_eq!(result, "31");
    }
}
