mod process_lines {
    use aoc2024::solver::process_lines;

    #[test]
    fn puzzle_subject_example() {
        // When
        let input = &[
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ];

        // When
        let result = process_lines(input.iter().map(|x| x.to_string()));

        // Then
        assert_eq!(result, "41");
    }
}
