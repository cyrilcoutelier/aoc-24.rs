mod process_lines {
    use aoc2024::solver::process_lines;

    #[test]
    fn puzzle_subject_example() {
        // When
        let input = &["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];

        // When
        let result = process_lines(input.iter().map(|x| x.to_string()));

        // Then
        assert_eq!(result, "161");
    }
}
