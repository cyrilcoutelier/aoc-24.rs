mod process_lines {
    use aoc2024::solver::process_lines;

    #[test]
    fn puzzle_subject_example() {
        // When
        let input = &[
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        // When
        let result = process_lines(input.iter().map(|x| x.to_string()));

        // Then
        assert_eq!(result, "18");
    }
}
