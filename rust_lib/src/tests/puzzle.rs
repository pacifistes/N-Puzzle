#[cfg(test)]
mod test {
    use crate::resolver::generate::r_generate_sorted_puzzle;
    use crate::resolver::puzzle::*;

    #[test]
    fn function_is_solvable() {
        //Solvable and size = 2
        let size: usize = 2;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![2, 0, 1, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 1, 0, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 2, 0, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 1, 0, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 0, 2, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 0, 1, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 0, 2, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 2, 0, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 3, 0, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 0, 1, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 1, 0, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 0, 2, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 0, 3, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 0, 1, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 2, 0, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 1, 0, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 3, 0, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 0, 3, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 3, 0, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 3, 0, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        //Solvable and size = 3
        let size: usize = 3;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![4, 7, 3, 8, 0, 5, 6, 2, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 4, 7, 2, 6, 5, 0, 8, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![6, 3, 4, 8, 7, 5, 2, 1, 0];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 8, 0, 7, 3, 4, 5, 1, 6];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 7, 8, 4, 6, 1, 0, 5, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![5, 4, 7, 2, 8, 1, 3, 6, 0];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 2, 5, 8, 7, 6, 0, 4, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![4, 8, 6, 3, 0, 7, 5, 2, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 1, 0, 8, 4, 5, 6, 7, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 8, 0, 4, 3, 7, 5, 6, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![6, 3, 5, 1, 0, 8, 7, 2, 4];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![4, 7, 8, 1, 6, 5, 3, 2, 0];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![0, 2, 7, 3, 5, 8, 6, 4, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 6, 0, 7, 2, 8, 4, 5, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![5, 4, 2, 3, 1, 8, 0, 6, 7];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 6, 8, 4, 0, 3, 2, 7, 5];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![8, 2, 4, 3, 7, 6, 0, 5, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![8, 2, 7, 4, 3, 6, 5, 1, 0];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![0, 3, 6, 8, 2, 1, 7, 5, 4];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![0, 8, 7, 4, 3, 6, 2, 5, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        //Solvable and size = 4
        let size: usize = 4;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![10, 7, 6, 11, 13, 12, 4, 3, 1, 14, 8, 2, 9, 5, 0, 15];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![14, 12, 8, 6, 4, 10, 0, 5, 13, 1, 15, 3, 7, 2, 11, 9];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![9, 5, 3, 4, 11, 15, 14, 13, 8, 0, 6, 7, 2, 12, 1, 10];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![11, 0, 15, 8, 10, 1, 13, 7, 12, 6, 5, 2, 4, 3, 9, 14];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![8, 12, 13, 4, 7, 6, 0, 14, 1, 3, 9, 2, 5, 11, 10, 15];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![5, 14, 12, 3, 4, 8, 0, 15, 1, 10, 7, 11, 13, 6, 9, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 4, 2, 11, 12, 10, 6, 5, 3, 0, 9, 15, 8, 7, 14, 13];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![6, 12, 14, 15, 0, 10, 1, 13, 8, 3, 4, 9, 2, 5, 11, 7];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 0, 2, 6, 4, 14, 1, 8, 7, 13, 15, 12, 9, 10, 5, 11];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![14, 11, 2, 7, 10, 9, 5, 15, 12, 13, 3, 0, 6, 4, 8, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![8, 9, 2, 3, 0, 13, 4, 1, 11, 14, 6, 12, 7, 10, 5, 15];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![8, 5, 4, 1, 14, 9, 15, 7, 6, 0, 10, 12, 11, 3, 13, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![10, 1, 9, 6, 2, 8, 0, 5, 14, 4, 11, 12, 15, 3, 7, 13];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![7, 12, 8, 0, 11, 1, 13, 14, 4, 2, 15, 3, 10, 6, 5, 9];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![14, 1, 15, 10, 2, 4, 11, 7, 3, 5, 6, 0, 8, 9, 12, 13];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![11, 14, 7, 0, 4, 13, 15, 10, 2, 9, 6, 5, 8, 12, 1, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![4, 1, 7, 6, 15, 5, 0, 2, 8, 14, 10, 3, 11, 12, 13, 9];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 6, 14, 5, 10, 11, 12, 13, 2, 1, 9, 8, 4, 7, 0, 15];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![11, 6, 15, 4, 9, 7, 1, 10, 13, 8, 14, 2, 5, 12, 0, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![9, 0, 11, 2, 14, 12, 6, 1, 7, 3, 5, 8, 15, 10, 4, 13];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        //Solvable and size = 5
        let size: usize = 5;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            23, 5, 3, 9, 11, 8, 1, 20, 22, 15, 4, 21, 10, 18, 0, 7, 2, 12, 17, 16, 13, 14, 19, 6,
            24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            22, 5, 6, 10, 19, 15, 16, 20, 18, 11, 0, 23, 12, 8, 7, 4, 9, 1, 24, 13, 21, 3, 14, 2,
            17,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            21, 22, 0, 3, 8, 18, 17, 23, 10, 5, 7, 9, 24, 2, 14, 4, 13, 19, 12, 16, 20, 1, 6, 15,
            11,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            23, 3, 19, 15, 11, 1, 8, 5, 17, 7, 22, 12, 4, 20, 16, 24, 13, 6, 10, 18, 14, 2, 0, 9,
            21,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            2, 22, 8, 3, 13, 17, 19, 5, 7, 6, 14, 18, 1, 12, 23, 10, 4, 24, 21, 11, 0, 9, 20, 15,
            16,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            21, 2, 10, 18, 6, 14, 7, 9, 1, 3, 24, 19, 17, 23, 0, 4, 12, 20, 5, 11, 22, 15, 16, 8,
            13,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            8, 3, 17, 18, 20, 10, 12, 11, 2, 21, 4, 14, 6, 16, 7, 13, 23, 15, 22, 19, 9, 5, 1, 24,
            0,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            11, 14, 13, 5, 10, 24, 6, 21, 18, 1, 9, 15, 0, 23, 20, 22, 8, 12, 3, 16, 17, 19, 2, 7,
            4,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            4, 23, 2, 24, 19, 8, 15, 14, 7, 21, 12, 18, 0, 11, 17, 13, 16, 20, 9, 10, 3, 5, 1, 6,
            22,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            13, 15, 1, 6, 2, 9, 5, 17, 18, 10, 21, 4, 0, 22, 24, 7, 23, 11, 16, 14, 8, 3, 20, 12,
            19,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            23, 12, 1, 14, 0, 4, 6, 7, 3, 16, 18, 10, 5, 8, 2, 20, 24, 22, 13, 15, 11, 9, 17, 21,
            19,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            21, 9, 23, 20, 2, 17, 5, 3, 12, 15, 1, 14, 13, 18, 0, 8, 6, 7, 19, 11, 16, 4, 22, 10,
            24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            4, 18, 14, 12, 16, 8, 0, 1, 9, 15, 10, 11, 5, 7, 2, 17, 13, 22, 24, 3, 20, 21, 23, 6,
            19,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            19, 16, 9, 3, 10, 2, 1, 7, 13, 8, 22, 24, 5, 11, 21, 12, 14, 23, 4, 6, 0, 17, 18, 15,
            20,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            20, 24, 14, 10, 23, 9, 22, 13, 0, 15, 21, 16, 6, 11, 4, 17, 8, 7, 2, 12, 5, 1, 3, 19,
            18,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            4, 3, 10, 2, 6, 9, 24, 7, 17, 21, 1, 18, 5, 23, 11, 8, 0, 14, 16, 12, 13, 22, 19, 15,
            20,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            0, 12, 8, 3, 4, 23, 16, 5, 10, 14, 13, 19, 2, 9, 17, 7, 22, 20, 6, 18, 15, 11, 21, 1,
            24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            9, 23, 6, 10, 24, 8, 11, 4, 14, 19, 22, 13, 16, 18, 20, 12, 7, 17, 0, 5, 21, 2, 3, 15,
            1,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            15, 5, 0, 22, 4, 16, 18, 2, 23, 14, 9, 10, 19, 3, 17, 6, 7, 8, 20, 1, 21, 12, 24, 11,
            13,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            14, 13, 6, 9, 19, 11, 15, 18, 12, 16, 20, 2, 17, 8, 5, 23, 24, 3, 0, 7, 1, 22, 10, 4,
            21,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        //Solvable and size = 6
        let size: usize = 6;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            3, 26, 23, 2, 9, 8, 0, 21, 24, 7, 19, 34, 14, 10, 32, 28, 18, 20, 33, 12, 25, 15, 35,
            13, 31, 1, 5, 29, 4, 30, 27, 6, 22, 16, 11, 17,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            20, 35, 33, 25, 27, 32, 11, 14, 30, 4, 21, 3, 10, 24, 17, 0, 28, 6, 5, 26, 16, 1, 18,
            22, 15, 7, 2, 13, 23, 8, 9, 12, 31, 29, 34, 19,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            24, 34, 7, 8, 2, 6, 0, 1, 11, 30, 14, 15, 33, 16, 32, 31, 23, 22, 5, 29, 12, 35, 25, 9,
            13, 20, 26, 10, 18, 27, 28, 21, 3, 4, 19, 17,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            3, 31, 19, 21, 30, 13, 23, 7, 8, 29, 1, 12, 4, 34, 5, 2, 22, 16, 17, 27, 32, 25, 35,
            18, 33, 14, 10, 15, 20, 0, 11, 28, 26, 24, 6, 9,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            35, 4, 13, 27, 7, 26, 33, 14, 28, 30, 5, 23, 34, 18, 9, 16, 32, 1, 10, 6, 22, 2, 19,
            15, 21, 0, 3, 11, 20, 12, 17, 31, 29, 8, 24, 25,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            19, 6, 3, 26, 5, 21, 28, 20, 2, 24, 13, 8, 18, 4, 17, 12, 27, 30, 23, 14, 0, 34, 11,
            32, 22, 1, 9, 7, 33, 16, 10, 35, 29, 31, 15, 25,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            28, 16, 13, 30, 25, 24, 26, 33, 0, 1, 14, 35, 5, 4, 9, 20, 18, 8, 2, 27, 12, 7, 10, 15,
            23, 6, 34, 17, 22, 31, 11, 29, 32, 19, 21, 3,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            14, 32, 2, 18, 10, 16, 34, 33, 0, 26, 12, 11, 4, 22, 19, 21, 3, 28, 23, 7, 17, 8, 31,
            24, 9, 15, 25, 6, 13, 5, 30, 29, 20, 35, 1, 27,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            26, 0, 9, 13, 11, 8, 2, 29, 22, 30, 21, 17, 16, 19, 3, 1, 5, 24, 27, 18, 10, 12, 4, 34,
            28, 23, 15, 25, 35, 31, 20, 14, 32, 7, 6, 33,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            11, 6, 25, 8, 5, 31, 1, 18, 17, 9, 34, 21, 29, 20, 33, 2, 10, 30, 32, 16, 0, 19, 7, 4,
            24, 26, 3, 35, 12, 22, 15, 13, 14, 23, 27, 28,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            4, 19, 33, 22, 10, 11, 3, 34, 28, 25, 27, 1, 18, 15, 32, 6, 9, 17, 8, 16, 31, 20, 2,
            12, 7, 0, 24, 23, 26, 35, 5, 13, 14, 29, 21, 30,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            26, 16, 6, 0, 21, 14, 11, 30, 23, 35, 2, 20, 34, 28, 22, 32, 18, 15, 29, 7, 31, 19, 25,
            17, 13, 9, 33, 4, 3, 10, 8, 27, 24, 1, 12, 5,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            3, 17, 30, 14, 9, 12, 24, 16, 34, 32, 35, 7, 22, 28, 29, 6, 18, 20, 21, 15, 19, 8, 25,
            11, 13, 23, 31, 0, 1, 26, 2, 5, 10, 4, 33, 27,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            13, 2, 16, 4, 31, 11, 30, 3, 14, 10, 0, 21, 18, 25, 20, 34, 12, 1, 28, 29, 26, 8, 6,
            23, 27, 19, 9, 7, 17, 33, 35, 22, 15, 5, 32, 24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            25, 15, 11, 29, 3, 8, 33, 17, 31, 21, 24, 34, 2, 1, 14, 0, 28, 35, 18, 12, 10, 7, 5,
            23, 22, 32, 16, 13, 26, 4, 19, 9, 6, 27, 20, 30,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            6, 24, 10, 25, 11, 27, 9, 21, 18, 15, 23, 12, 30, 14, 29, 2, 13, 17, 19, 4, 31, 8, 20,
            26, 28, 32, 34, 35, 16, 7, 5, 1, 22, 3, 0, 33,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            3, 32, 18, 24, 19, 29, 11, 13, 25, 27, 20, 2, 34, 10, 12, 0, 26, 22, 15, 33, 28, 14,
            21, 17, 31, 35, 16, 1, 4, 5, 9, 7, 8, 30, 6, 23,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            15, 3, 32, 27, 33, 29, 13, 9, 22, 25, 14, 34, 18, 7, 35, 16, 17, 28, 19, 31, 5, 23, 20,
            10, 6, 12, 24, 11, 1, 0, 30, 26, 2, 4, 21, 8,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            16, 23, 8, 18, 21, 35, 3, 31, 32, 7, 26, 15, 22, 1, 24, 4, 29, 33, 0, 19, 5, 20, 9, 25,
            27, 30, 12, 13, 6, 11, 28, 34, 17, 14, 10, 2,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            8, 35, 20, 28, 4, 30, 23, 19, 21, 18, 0, 16, 26, 13, 34, 6, 24, 9, 7, 10, 5, 29, 17,
            15, 3, 31, 25, 1, 33, 27, 22, 2, 32, 11, 14, 12,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        //Solvable and size = 7
        let size: usize = 7;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            29, 32, 8, 41, 45, 14, 28, 46, 18, 22, 0, 48, 15, 30, 11, 33, 20, 19, 40, 39, 27, 35,
            1, 3, 34, 25, 12, 4, 36, 44, 2, 16, 37, 31, 6, 38, 13, 5, 43, 24, 17, 23, 9, 42, 47,
            10, 26, 7, 21,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            34, 10, 36, 5, 23, 39, 17, 43, 6, 40, 37, 30, 32, 45, 2, 18, 0, 16, 4, 44, 22, 9, 14,
            13, 38, 19, 26, 46, 21, 48, 1, 33, 27, 35, 15, 25, 41, 42, 12, 24, 29, 20, 3, 7, 28,
            11, 31, 47, 8,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            17, 26, 20, 5, 46, 47, 2, 48, 25, 16, 38, 37, 40, 41, 8, 31, 11, 39, 7, 18, 9, 21, 14,
            34, 19, 10, 13, 43, 12, 29, 27, 6, 23, 28, 1, 42, 33, 4, 22, 3, 0, 15, 36, 24, 44, 45,
            30, 35, 32,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            22, 47, 42, 18, 5, 37, 31, 40, 20, 23, 27, 28, 34, 11, 26, 46, 24, 13, 29, 32, 3, 30,
            16, 10, 19, 45, 0, 21, 4, 44, 2, 36, 17, 6, 8, 14, 35, 48, 41, 1, 15, 38, 43, 9, 33,
            12, 25, 7, 39,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            14, 20, 48, 1, 9, 7, 23, 37, 42, 33, 27, 18, 4, 36, 3, 10, 31, 5, 41, 26, 0, 2, 44, 19,
            40, 17, 16, 13, 39, 24, 30, 25, 28, 45, 11, 34, 35, 15, 29, 38, 22, 21, 32, 47, 12, 43,
            8, 6, 46,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            0, 1, 19, 13, 29, 17, 28, 16, 41, 34, 30, 20, 46, 6, 18, 40, 4, 26, 2, 32, 31, 3, 5,
            42, 43, 12, 7, 11, 24, 36, 23, 48, 25, 10, 27, 14, 8, 21, 38, 9, 35, 39, 47, 44, 22,
            37, 15, 45, 33,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            28, 23, 41, 24, 8, 19, 14, 2, 25, 26, 4, 35, 30, 32, 47, 48, 10, 20, 17, 1, 0, 27, 34,
            38, 13, 6, 18, 12, 40, 31, 46, 36, 29, 16, 11, 33, 42, 37, 43, 3, 7, 45, 39, 15, 22,
            21, 44, 5, 9,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            20, 24, 12, 18, 0, 13, 8, 32, 11, 31, 27, 17, 21, 47, 7, 6, 4, 10, 14, 23, 9, 38, 5,
            40, 19, 43, 46, 41, 36, 37, 39, 42, 3, 1, 48, 34, 22, 25, 45, 33, 28, 2, 29, 30, 16,
            35, 44, 26, 15,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            44, 39, 14, 12, 17, 16, 37, 22, 36, 45, 8, 3, 38, 28, 33, 30, 7, 43, 18, 23, 34, 2, 48,
            42, 19, 46, 41, 10, 35, 5, 25, 20, 27, 31, 0, 24, 32, 26, 21, 6, 9, 29, 40, 1, 11, 15,
            13, 4, 47,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            27, 23, 46, 42, 8, 47, 21, 48, 41, 38, 30, 7, 20, 29, 19, 33, 16, 31, 22, 14, 28, 39,
            25, 6, 17, 45, 32, 35, 40, 11, 9, 12, 36, 34, 3, 15, 13, 24, 10, 44, 26, 4, 1, 18, 0,
            5, 2, 37, 43,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            9, 48, 26, 16, 11, 32, 29, 40, 35, 15, 36, 33, 6, 19, 42, 37, 34, 46, 27, 14, 12, 20,
            5, 10, 47, 4, 2, 21, 38, 39, 0, 41, 24, 45, 25, 22, 7, 8, 18, 44, 43, 3, 1, 13, 30, 28,
            23, 31, 17,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            5, 44, 40, 3, 13, 2, 39, 1, 32, 23, 46, 17, 14, 31, 29, 15, 25, 41, 7, 20, 27, 8, 34,
            33, 0, 26, 48, 12, 24, 36, 37, 18, 47, 21, 45, 9, 22, 10, 6, 42, 11, 30, 16, 43, 28, 4,
            19, 38, 35,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            32, 37, 36, 1, 20, 22, 9, 40, 35, 14, 15, 21, 41, 24, 29, 19, 33, 39, 4, 6, 44, 46, 47,
            2, 27, 3, 0, 26, 12, 43, 42, 28, 13, 11, 38, 30, 25, 34, 18, 16, 45, 31, 10, 23, 48,
            17, 7, 5, 8,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            44, 30, 31, 38, 5, 35, 19, 34, 23, 37, 17, 3, 22, 16, 10, 20, 43, 14, 39, 33, 15, 7, 0,
            6, 29, 25, 32, 12, 24, 40, 18, 48, 46, 36, 13, 11, 9, 45, 27, 21, 4, 2, 8, 42, 41, 1,
            28, 26, 47,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            37, 26, 3, 45, 20, 18, 43, 17, 41, 29, 38, 21, 2, 1, 9, 30, 11, 35, 25, 16, 10, 12, 4,
            24, 31, 40, 8, 42, 33, 15, 32, 5, 22, 47, 0, 36, 7, 19, 13, 6, 46, 39, 28, 48, 34, 14,
            44, 27, 23,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            5, 46, 31, 9, 13, 35, 43, 32, 11, 10, 34, 21, 14, 20, 44, 36, 29, 16, 0, 12, 2, 42, 1,
            15, 41, 8, 27, 25, 4, 39, 37, 7, 28, 23, 38, 18, 19, 22, 33, 48, 45, 6, 17, 30, 3, 26,
            40, 47, 24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            6, 7, 36, 9, 35, 32, 24, 38, 19, 10, 37, 13, 18, 34, 17, 21, 26, 22, 4, 31, 23, 27, 3,
            33, 5, 42, 8, 44, 25, 20, 41, 1, 47, 40, 45, 46, 48, 14, 29, 12, 0, 39, 2, 11, 28, 15,
            30, 43, 16,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            25, 2, 8, 28, 31, 14, 10, 46, 22, 3, 29, 47, 23, 1, 38, 32, 33, 35, 41, 44, 27, 15, 17,
            26, 19, 42, 0, 43, 18, 21, 48, 36, 45, 5, 30, 24, 11, 20, 39, 34, 6, 12, 40, 4, 13, 37,
            9, 7, 16,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            45, 12, 15, 42, 0, 1, 43, 24, 19, 34, 36, 37, 11, 4, 2, 13, 35, 22, 8, 7, 20, 3, 21,
            14, 41, 10, 29, 39, 38, 31, 16, 26, 40, 17, 46, 18, 44, 48, 27, 47, 5, 6, 23, 25, 30,
            32, 9, 33, 28,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            46, 13, 2, 35, 26, 41, 45, 34, 0, 27, 29, 43, 38, 12, 37, 10, 5, 40, 19, 4, 6, 1, 36,
            24, 7, 42, 15, 32, 23, 33, 17, 14, 39, 25, 30, 16, 28, 9, 21, 3, 31, 11, 22, 48, 44, 8,
            18, 47, 20,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        //Solvable and size = 8
        let size: usize = 8;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            18, 53, 8, 22, 17, 47, 10, 1, 29, 19, 46, 9, 31, 7, 16, 54, 55, 21, 58, 61, 14, 6, 11,
            42, 5, 39, 49, 13, 30, 12, 15, 25, 35, 3, 38, 57, 56, 45, 24, 43, 33, 37, 59, 27, 28,
            48, 0, 23, 62, 63, 60, 41, 44, 40, 2, 50, 4, 20, 32, 36, 34, 26, 51, 52,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            53, 2, 9, 41, 21, 51, 20, 59, 0, 10, 60, 24, 52, 29, 26, 44, 5, 4, 58, 23, 62, 42, 56,
            54, 27, 1, 55, 50, 12, 14, 32, 43, 25, 13, 6, 11, 28, 33, 48, 47, 19, 22, 57, 15, 63,
            17, 16, 49, 30, 37, 38, 61, 46, 31, 36, 7, 34, 8, 18, 40, 3, 39, 45, 35,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            20, 60, 19, 45, 62, 46, 23, 1, 40, 57, 56, 11, 55, 8, 50, 10, 28, 24, 14, 59, 52, 54,
            16, 33, 29, 44, 17, 36, 25, 35, 2, 31, 37, 0, 51, 34, 3, 6, 5, 38, 4, 22, 13, 63, 15,
            53, 30, 18, 27, 49, 47, 41, 26, 58, 43, 32, 12, 42, 39, 9, 21, 7, 48, 61,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            17, 57, 11, 25, 30, 54, 36, 2, 8, 37, 46, 19, 42, 61, 0, 14, 51, 24, 3, 10, 34, 63, 41,
            22, 48, 58, 56, 55, 35, 31, 28, 27, 40, 33, 12, 1, 59, 47, 20, 16, 53, 43, 38, 9, 45,
            39, 26, 4, 23, 21, 29, 13, 5, 6, 18, 44, 15, 7, 49, 52, 62, 60, 32, 50,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            29, 60, 25, 4, 7, 2, 8, 11, 17, 38, 61, 39, 46, 36, 45, 35, 48, 12, 63, 34, 19, 26, 32,
            0, 41, 33, 51, 22, 47, 55, 16, 54, 53, 23, 50, 57, 21, 10, 49, 28, 14, 9, 59, 58, 44,
            3, 31, 30, 43, 56, 1, 40, 27, 6, 37, 20, 42, 5, 18, 52, 24, 62, 15, 13,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            32, 48, 18, 25, 49, 0, 31, 54, 53, 14, 50, 4, 35, 56, 38, 16, 37, 22, 5, 3, 34, 30, 2,
            29, 59, 36, 6, 10, 39, 43, 55, 17, 45, 58, 27, 9, 44, 61, 42, 11, 15, 7, 63, 60, 1, 19,
            23, 47, 57, 51, 33, 62, 26, 20, 40, 41, 12, 52, 21, 46, 13, 28, 24, 8,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            20, 4, 18, 52, 3, 32, 36, 21, 43, 24, 14, 13, 5, 33, 45, 11, 2, 38, 42, 26, 31, 7, 9,
            23, 55, 27, 29, 19, 0, 50, 37, 8, 58, 56, 51, 39, 25, 62, 35, 6, 34, 17, 60, 46, 22,
            54, 47, 63, 49, 59, 57, 12, 40, 15, 10, 30, 61, 48, 44, 53, 1, 16, 41, 28,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            46, 42, 14, 48, 58, 50, 41, 61, 36, 28, 47, 10, 43, 54, 33, 31, 8, 27, 4, 22, 20, 57,
            55, 35, 2, 3, 0, 60, 9, 24, 34, 26, 18, 25, 21, 53, 15, 6, 7, 52, 56, 59, 16, 63, 39,
            44, 49, 12, 38, 11, 40, 17, 13, 1, 32, 19, 51, 30, 23, 37, 45, 5, 62, 29,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            6, 35, 19, 15, 63, 24, 56, 3, 54, 28, 40, 51, 7, 33, 11, 37, 52, 41, 50, 59, 49, 30,
            10, 22, 43, 61, 46, 60, 9, 45, 62, 27, 47, 23, 48, 1, 57, 29, 39, 12, 38, 8, 58, 14,
            25, 32, 26, 4, 31, 2, 34, 0, 5, 44, 21, 55, 16, 42, 18, 13, 17, 20, 36, 53,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            47, 24, 48, 22, 60, 33, 13, 61, 55, 4, 2, 27, 26, 12, 9, 11, 62, 51, 32, 21, 19, 43,
            44, 40, 34, 23, 49, 45, 30, 46, 28, 35, 56, 25, 14, 54, 7, 18, 58, 10, 37, 17, 1, 41,
            6, 38, 29, 53, 36, 15, 20, 0, 31, 8, 50, 5, 59, 42, 39, 3, 57, 52, 63, 16,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            59, 17, 4, 31, 55, 37, 5, 44, 21, 11, 16, 52, 0, 45, 15, 14, 30, 19, 35, 47, 60, 6, 61,
            8, 22, 7, 62, 41, 18, 26, 29, 53, 28, 57, 33, 48, 63, 39, 3, 36, 49, 13, 50, 1, 10, 9,
            12, 42, 46, 54, 51, 25, 32, 24, 43, 40, 27, 58, 2, 34, 23, 20, 56, 38,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            1, 18, 12, 32, 22, 35, 2, 50, 60, 10, 8, 62, 14, 56, 37, 39, 19, 44, 47, 16, 59, 58,
            24, 55, 36, 3, 17, 26, 61, 31, 27, 49, 48, 51, 4, 45, 52, 28, 21, 53, 34, 29, 20, 15,
            41, 46, 13, 9, 30, 57, 23, 25, 11, 40, 38, 43, 42, 5, 0, 54, 33, 6, 7, 63,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            41, 31, 28, 44, 14, 6, 52, 61, 5, 50, 13, 63, 17, 62, 7, 60, 27, 18, 8, 47, 46, 1, 11,
            9, 16, 54, 12, 42, 20, 21, 36, 37, 25, 15, 19, 30, 53, 51, 57, 4, 3, 2, 33, 43, 34, 49,
            26, 48, 56, 23, 58, 0, 59, 55, 35, 40, 24, 39, 38, 29, 22, 32, 45, 10,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            13, 34, 1, 31, 18, 38, 12, 37, 6, 11, 28, 33, 24, 22, 15, 16, 49, 17, 35, 3, 44, 36,
            52, 48, 50, 41, 51, 30, 29, 23, 59, 7, 5, 0, 10, 55, 61, 27, 43, 19, 47, 4, 60, 62, 53,
            21, 39, 2, 32, 46, 63, 40, 9, 54, 57, 45, 26, 20, 25, 58, 14, 56, 42, 8,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            62, 54, 56, 31, 50, 29, 51, 21, 4, 37, 36, 13, 20, 48, 5, 60, 61, 26, 43, 46, 34, 8,
            52, 40, 30, 28, 33, 39, 0, 55, 63, 53, 44, 2, 58, 35, 7, 27, 47, 42, 23, 11, 41, 22,
            10, 14, 18, 3, 17, 38, 49, 16, 24, 15, 1, 25, 57, 6, 9, 59, 12, 32, 45, 19,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            61, 6, 17, 53, 47, 48, 24, 31, 45, 50, 33, 8, 3, 52, 57, 34, 42, 29, 44, 21, 1, 26, 40,
            46, 28, 62, 9, 56, 15, 27, 10, 63, 54, 35, 43, 32, 60, 38, 37, 12, 7, 20, 11, 22, 13,
            59, 18, 39, 16, 4, 36, 23, 2, 0, 41, 58, 14, 55, 49, 5, 51, 30, 19, 25,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            60, 18, 44, 7, 41, 47, 8, 12, 28, 53, 2, 48, 45, 14, 32, 33, 26, 25, 13, 59, 34, 0, 3,
            61, 46, 23, 5, 51, 56, 22, 17, 42, 52, 40, 27, 29, 11, 63, 35, 58, 55, 31, 49, 1, 50,
            10, 37, 21, 54, 36, 43, 62, 15, 20, 38, 39, 4, 19, 9, 57, 24, 16, 6, 30,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            63, 8, 54, 60, 1, 33, 35, 16, 27, 43, 39, 13, 7, 62, 57, 26, 50, 29, 9, 10, 3, 28, 47,
            52, 48, 5, 51, 18, 17, 49, 32, 21, 42, 6, 55, 46, 19, 40, 44, 31, 41, 2, 53, 30, 45,
            34, 15, 11, 36, 20, 22, 38, 12, 24, 59, 0, 61, 37, 23, 25, 14, 4, 56, 58,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            15, 57, 28, 40, 8, 31, 37, 4, 20, 22, 41, 38, 61, 11, 42, 26, 45, 50, 58, 43, 48, 54,
            25, 39, 24, 47, 2, 27, 5, 35, 19, 23, 16, 53, 44, 7, 56, 32, 59, 1, 14, 49, 52, 12, 10,
            30, 29, 21, 55, 3, 36, 33, 18, 62, 63, 51, 0, 17, 6, 46, 60, 34, 13, 9,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            6, 0, 20, 1, 17, 58, 38, 51, 36, 52, 15, 5, 26, 24, 54, 31, 33, 11, 60, 39, 44, 42, 10,
            47, 13, 32, 34, 46, 29, 35, 45, 18, 22, 3, 27, 63, 23, 37, 25, 7, 55, 43, 2, 19, 40,
            62, 4, 48, 53, 12, 16, 30, 50, 21, 57, 49, 8, 14, 59, 28, 56, 61, 9, 41,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        //Solvable and size = 9
        let size: usize = 9;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            35, 9, 4, 18, 67, 56, 28, 16, 24, 11, 45, 50, 1, 61, 37, 19, 62, 39, 72, 48, 34, 29,
            17, 80, 55, 42, 8, 47, 53, 5, 58, 66, 22, 33, 7, 60, 0, 32, 20, 14, 46, 30, 70, 68, 49,
            74, 27, 44, 69, 38, 57, 10, 13, 21, 51, 31, 26, 2, 78, 64, 23, 54, 76, 73, 3, 71, 41,
            6, 52, 43, 65, 25, 15, 75, 63, 79, 36, 77, 12, 59, 40,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            0, 32, 9, 35, 29, 33, 69, 43, 66, 38, 10, 21, 39, 26, 8, 65, 28, 4, 23, 22, 58, 52, 27,
            70, 24, 64, 44, 1, 67, 73, 7, 63, 79, 48, 75, 40, 31, 49, 20, 45, 15, 42, 34, 77, 14,
            6, 36, 71, 53, 61, 74, 5, 2, 54, 13, 30, 62, 17, 47, 18, 55, 68, 11, 72, 51, 78, 12, 3,
            46, 80, 60, 57, 59, 76, 19, 16, 41, 56, 25, 50, 37,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            80, 23, 33, 60, 40, 63, 55, 14, 74, 56, 57, 10, 28, 38, 51, 39, 1, 61, 48, 44, 5, 19,
            16, 54, 75, 12, 41, 77, 35, 27, 69, 47, 68, 18, 0, 59, 20, 70, 53, 72, 21, 32, 79, 2,
            7, 3, 71, 22, 24, 62, 29, 58, 6, 65, 25, 9, 78, 13, 42, 11, 17, 30, 66, 49, 73, 50, 45,
            37, 31, 76, 15, 36, 43, 52, 67, 8, 64, 34, 46, 26, 4,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            73, 30, 72, 2, 22, 52, 18, 45, 9, 55, 29, 37, 57, 26, 6, 11, 44, 76, 41, 35, 25, 46, 4,
            54, 75, 5, 71, 36, 14, 3, 15, 50, 60, 62, 31, 7, 68, 21, 0, 49, 43, 10, 74, 79, 47, 77,
            56, 39, 64, 1, 33, 23, 61, 80, 42, 67, 27, 16, 17, 28, 20, 78, 8, 70, 59, 19, 58, 12,
            51, 63, 32, 40, 53, 48, 66, 69, 13, 38, 65, 34, 24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            0, 8, 12, 78, 60, 31, 67, 58, 43, 34, 3, 54, 69, 14, 72, 49, 64, 61, 5, 45, 18, 55, 28,
            22, 27, 11, 39, 36, 21, 35, 51, 59, 74, 71, 50, 26, 2, 68, 46, 52, 80, 37, 13, 30, 20,
            24, 23, 32, 40, 10, 42, 25, 77, 33, 63, 79, 48, 53, 9, 70, 17, 75, 44, 19, 1, 47, 16,
            62, 65, 29, 73, 57, 76, 38, 66, 56, 6, 4, 7, 41, 15,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            57, 61, 53, 9, 44, 37, 54, 74, 7, 73, 78, 2, 51, 64, 0, 35, 34, 67, 52, 68, 6, 5, 56,
            14, 30, 62, 12, 58, 79, 8, 69, 29, 21, 40, 43, 11, 45, 32, 76, 33, 39, 47, 55, 46, 20,
            25, 42, 28, 19, 70, 22, 4, 59, 31, 80, 48, 63, 15, 24, 17, 41, 10, 18, 60, 38, 49, 1,
            50, 36, 71, 16, 77, 27, 23, 26, 65, 75, 72, 13, 66, 3,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            36, 23, 5, 80, 45, 6, 58, 4, 3, 69, 39, 31, 52, 12, 9, 10, 55, 78, 32, 74, 38, 57, 27,
            22, 76, 2, 37, 30, 64, 20, 47, 25, 29, 33, 28, 62, 43, 73, 56, 21, 0, 60, 35, 8, 16,
            46, 41, 48, 75, 11, 66, 59, 14, 50, 42, 18, 44, 72, 34, 26, 53, 7, 54, 70, 19, 67, 71,
            49, 15, 65, 77, 40, 17, 79, 51, 1, 61, 68, 13, 24, 63,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            54, 72, 2, 16, 75, 14, 6, 50, 59, 25, 35, 67, 73, 66, 27, 18, 60, 41, 65, 57, 55, 5,
            44, 70, 45, 68, 1, 8, 52, 78, 31, 61, 38, 28, 4, 47, 69, 48, 23, 37, 56, 39, 71, 11,
            21, 80, 22, 24, 62, 46, 34, 26, 7, 77, 64, 33, 17, 49, 58, 76, 30, 43, 9, 13, 20, 36,
            40, 74, 79, 63, 0, 32, 51, 29, 53, 3, 19, 15, 12, 10, 42,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            37, 34, 0, 57, 79, 31, 47, 39, 53, 60, 55, 1, 68, 32, 51, 19, 13, 41, 43, 23, 15, 10,
            4, 50, 63, 75, 58, 72, 74, 7, 35, 56, 64, 6, 28, 44, 9, 46, 77, 2, 45, 42, 12, 49, 24,
            59, 20, 17, 62, 14, 30, 40, 38, 26, 69, 80, 29, 66, 65, 67, 36, 52, 70, 27, 61, 54, 78,
            5, 73, 22, 48, 16, 33, 21, 25, 11, 8, 18, 71, 76, 3,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            42, 59, 38, 34, 3, 49, 70, 75, 11, 68, 29, 32, 71, 28, 40, 45, 55, 73, 52, 7, 8, 78,
            31, 76, 37, 24, 41, 33, 63, 2, 61, 13, 21, 77, 17, 58, 0, 57, 18, 9, 48, 25, 6, 72, 47,
            26, 79, 27, 74, 1, 51, 39, 43, 54, 50, 69, 12, 30, 66, 80, 10, 56, 20, 35, 5, 4, 19,
            65, 64, 36, 44, 67, 53, 22, 60, 16, 15, 14, 23, 62, 46,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            41, 58, 19, 56, 1, 66, 18, 34, 2, 52, 29, 59, 54, 74, 62, 60, 71, 39, 70, 13, 40, 17,
            25, 20, 76, 7, 44, 28, 4, 79, 8, 35, 23, 16, 61, 68, 36, 5, 31, 73, 11, 50, 57, 12, 37,
            33, 3, 21, 77, 69, 10, 6, 0, 67, 63, 75, 51, 80, 42, 27, 15, 47, 78, 32, 55, 48, 46,
            64, 49, 22, 9, 72, 26, 30, 53, 38, 24, 65, 14, 43, 45,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            79, 35, 1, 32, 29, 53, 26, 30, 60, 73, 56, 51, 68, 64, 0, 10, 34, 48, 5, 27, 69, 13,
            21, 12, 7, 22, 3, 19, 36, 78, 59, 42, 74, 62, 58, 2, 47, 71, 9, 72, 8, 6, 40, 77, 4,
            61, 24, 14, 17, 50, 39, 67, 11, 16, 25, 57, 33, 45, 37, 31, 76, 66, 44, 49, 43, 28, 20,
            46, 15, 75, 38, 55, 80, 52, 23, 54, 65, 63, 70, 41, 18,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            33, 60, 36, 9, 57, 13, 69, 50, 73, 52, 1, 26, 78, 59, 39, 12, 77, 53, 40, 65, 3, 55,
            31, 56, 4, 43, 48, 37, 15, 62, 0, 23, 63, 61, 6, 44, 71, 29, 64, 28, 20, 21, 14, 17, 8,
            68, 51, 11, 5, 22, 35, 74, 38, 27, 30, 25, 66, 42, 34, 16, 49, 47, 24, 2, 67, 19, 54,
            70, 76, 75, 41, 10, 18, 32, 79, 58, 72, 7, 80, 46, 45,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            36, 24, 56, 10, 60, 9, 7, 59, 27, 70, 40, 77, 19, 35, 33, 11, 31, 47, 26, 3, 73, 65,
            12, 69, 57, 22, 63, 34, 62, 43, 38, 37, 79, 1, 54, 48, 50, 72, 25, 66, 53, 30, 39, 41,
            55, 78, 45, 29, 13, 2, 23, 51, 75, 68, 4, 74, 80, 42, 18, 64, 20, 15, 61, 32, 5, 8, 44,
            6, 21, 58, 0, 49, 71, 76, 28, 16, 17, 52, 67, 46, 14,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            4, 78, 18, 76, 24, 69, 12, 34, 67, 5, 30, 74, 11, 32, 2, 70, 25, 10, 56, 13, 33, 3, 77,
            44, 0, 45, 63, 49, 1, 22, 16, 61, 9, 28, 36, 38, 52, 57, 26, 64, 51, 43, 53, 35, 75,
            42, 71, 7, 60, 46, 41, 68, 8, 66, 21, 37, 40, 20, 6, 58, 47, 14, 50, 72, 27, 54, 73,
            65, 79, 29, 19, 59, 62, 23, 15, 48, 55, 17, 39, 31, 80,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            62, 75, 53, 46, 80, 74, 8, 10, 54, 60, 58, 2, 15, 34, 5, 27, 77, 59, 7, 41, 70, 71, 30,
            32, 23, 11, 48, 13, 3, 1, 78, 66, 51, 36, 0, 39, 57, 43, 73, 37, 9, 14, 69, 76, 56, 72,
            67, 18, 24, 35, 22, 38, 26, 19, 65, 45, 63, 4, 33, 55, 52, 42, 47, 28, 16, 31, 20, 25,
            17, 61, 40, 29, 79, 44, 21, 6, 12, 49, 64, 50, 68,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            55, 30, 75, 52, 36, 7, 31, 15, 40, 33, 42, 64, 3, 79, 78, 67, 54, 61, 14, 16, 4, 5, 56,
            29, 76, 6, 19, 18, 24, 38, 60, 10, 50, 9, 70, 37, 57, 51, 35, 28, 20, 59, 68, 12, 39,
            26, 0, 47, 41, 48, 13, 72, 66, 65, 32, 23, 8, 45, 34, 22, 77, 49, 63, 69, 46, 1, 58,
            71, 25, 44, 73, 27, 74, 11, 21, 80, 62, 2, 17, 53, 43,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            76, 63, 78, 43, 66, 21, 14, 45, 36, 25, 46, 58, 20, 51, 41, 10, 42, 49, 31, 3, 61, 2,
            5, 60, 77, 4, 16, 24, 1, 29, 18, 34, 26, 55, 47, 23, 9, 13, 67, 48, 40, 17, 19, 7, 27,
            52, 39, 68, 79, 22, 0, 50, 6, 65, 15, 53, 38, 75, 44, 57, 73, 28, 33, 72, 80, 69, 74,
            11, 71, 30, 35, 12, 56, 64, 62, 70, 54, 32, 59, 8, 37,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            53, 78, 74, 36, 63, 42, 41, 61, 73, 15, 1, 7, 48, 71, 28, 68, 9, 69, 11, 47, 0, 43, 45,
            25, 76, 4, 58, 6, 52, 66, 80, 5, 32, 40, 64, 35, 26, 17, 59, 75, 2, 24, 31, 51, 23, 77,
            19, 72, 50, 33, 67, 16, 62, 37, 54, 22, 39, 20, 3, 14, 56, 44, 79, 12, 21, 29, 55, 10,
            46, 30, 49, 13, 65, 34, 57, 60, 70, 38, 8, 18, 27,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            4, 68, 20, 10, 50, 73, 37, 43, 71, 72, 9, 28, 66, 74, 5, 60, 58, 16, 11, 30, 22, 67,
            14, 33, 17, 8, 61, 34, 6, 24, 41, 32, 0, 48, 13, 57, 31, 26, 51, 53, 78, 52, 39, 15,
            76, 46, 36, 1, 38, 18, 45, 62, 54, 56, 19, 47, 70, 49, 35, 7, 3, 75, 65, 12, 42, 21,
            25, 64, 55, 80, 23, 40, 2, 59, 63, 44, 27, 79, 29, 69, 77,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        //Solvable and size = 10
        let size: usize = 10;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            65, 98, 63, 72, 69, 87, 97, 73, 5, 19, 68, 1, 2, 44, 13, 9, 64, 45, 90, 22, 27, 4, 59,
            88, 58, 6, 84, 14, 12, 7, 83, 70, 82, 37, 15, 36, 86, 38, 81, 3, 33, 28, 17, 24, 53,
            48, 39, 11, 35, 93, 96, 42, 60, 51, 23, 66, 10, 30, 8, 99, 71, 21, 18, 94, 32, 95, 54,
            75, 56, 20, 77, 55, 0, 52, 47, 16, 31, 67, 57, 91, 80, 41, 61, 85, 89, 40, 34, 29, 79,
            49, 46, 26, 78, 43, 50, 62, 76, 25, 92, 74,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            87, 46, 86, 67, 37, 47, 16, 12, 8, 81, 39, 30, 83, 65, 4, 19, 91, 61, 44, 71, 36, 69,
            41, 34, 42, 15, 53, 3, 60, 88, 62, 17, 64, 7, 22, 92, 56, 68, 10, 43, 2, 76, 82, 45,
            25, 89, 32, 74, 9, 51, 94, 78, 35, 21, 73, 38, 50, 79, 48, 14, 80, 63, 98, 58, 77, 55,
            90, 13, 29, 72, 57, 54, 31, 28, 5, 20, 6, 97, 0, 99, 27, 52, 1, 66, 18, 95, 75, 59, 70,
            96, 33, 23, 85, 26, 84, 40, 24, 11, 93, 49,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            26, 1, 79, 18, 84, 5, 4, 3, 69, 10, 60, 47, 66, 34, 36, 6, 12, 90, 11, 49, 57, 35, 78,
            8, 44, 43, 76, 30, 41, 67, 96, 54, 17, 45, 16, 53, 9, 25, 22, 15, 38, 31, 61, 59, 52,
            20, 39, 28, 70, 92, 94, 42, 81, 37, 27, 88, 64, 13, 89, 46, 65, 0, 63, 95, 62, 56, 29,
            71, 48, 50, 40, 55, 32, 85, 97, 86, 83, 51, 23, 73, 82, 2, 58, 80, 93, 19, 74, 68, 91,
            98, 24, 33, 87, 77, 21, 14, 72, 75, 99, 7,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            2, 12, 92, 96, 45, 39, 11, 9, 7, 42, 83, 29, 38, 41, 8, 48, 90, 10, 69, 43, 76, 65, 25,
            44, 66, 93, 59, 19, 54, 24, 61, 68, 91, 62, 3, 4, 58, 14, 27, 72, 47, 0, 5, 13, 63, 89,
            49, 70, 28, 46, 15, 6, 31, 74, 73, 33, 88, 71, 60, 26, 36, 82, 32, 1, 55, 57, 99, 95,
            75, 34, 67, 80, 64, 40, 56, 16, 77, 17, 21, 18, 94, 30, 84, 78, 86, 53, 87, 50, 20, 85,
            81, 98, 79, 22, 52, 35, 97, 51, 23, 37,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            66, 64, 58, 86, 73, 78, 7, 91, 89, 70, 41, 4, 80, 13, 44, 5, 47, 69, 14, 87, 71, 45, 2,
            98, 40, 49, 68, 74, 67, 52, 23, 37, 79, 56, 36, 72, 53, 10, 0, 24, 6, 57, 32, 34, 22,
            48, 18, 51, 16, 50, 77, 12, 59, 39, 88, 19, 92, 27, 76, 60, 8, 82, 17, 9, 20, 97, 63,
            26, 46, 15, 84, 93, 94, 35, 33, 11, 29, 54, 30, 42, 61, 65, 3, 95, 85, 99, 90, 43, 28,
            55, 31, 83, 38, 25, 62, 81, 75, 96, 1, 21,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            23, 7, 67, 60, 13, 34, 35, 10, 15, 3, 61, 27, 40, 78, 6, 84, 45, 48, 88, 99, 36, 96,
            89, 68, 28, 16, 80, 11, 97, 22, 57, 9, 64, 31, 82, 77, 58, 46, 21, 44, 8, 41, 37, 43,
            5, 69, 24, 72, 18, 83, 54, 33, 39, 38, 85, 66, 47, 98, 91, 49, 30, 70, 65, 32, 50, 94,
            74, 29, 4, 63, 81, 59, 1, 51, 79, 90, 62, 14, 71, 53, 55, 73, 26, 56, 75, 93, 2, 52,
            12, 76, 17, 25, 0, 19, 87, 92, 86, 42, 20, 95,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            66, 63, 70, 67, 32, 84, 41, 9, 43, 3, 62, 85, 39, 78, 14, 64, 22, 30, 13, 97, 34, 4,
            40, 47, 49, 95, 44, 10, 51, 37, 71, 38, 36, 89, 33, 55, 27, 69, 45, 73, 65, 24, 6, 68,
            42, 83, 46, 86, 59, 8, 25, 57, 29, 82, 81, 11, 52, 48, 19, 72, 79, 2, 28, 77, 20, 5,
            17, 23, 35, 7, 50, 90, 80, 96, 60, 18, 98, 15, 0, 12, 92, 31, 58, 53, 1, 26, 56, 93,
            76, 88, 61, 94, 91, 99, 75, 16, 21, 87, 74, 54,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            84, 41, 12, 38, 97, 44, 11, 28, 66, 98, 1, 69, 49, 40, 4, 70, 47, 42, 46, 62, 91, 37,
            96, 2, 17, 72, 86, 23, 51, 73, 0, 36, 33, 6, 87, 85, 67, 13, 9, 43, 93, 63, 74, 68, 89,
            15, 53, 10, 20, 45, 80, 61, 60, 82, 57, 92, 14, 26, 65, 83, 31, 54, 39, 64, 75, 71, 27,
            22, 52, 34, 3, 59, 94, 30, 25, 88, 5, 16, 99, 78, 58, 76, 7, 18, 95, 35, 79, 32, 81,
            48, 8, 24, 90, 55, 77, 56, 29, 50, 19, 21,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            87, 65, 5, 73, 78, 57, 42, 0, 35, 6, 63, 61, 67, 66, 4, 2, 9, 94, 12, 44, 14, 26, 34,
            29, 69, 54, 7, 21, 39, 13, 62, 56, 83, 47, 48, 53, 40, 97, 77, 70, 64, 84, 82, 93, 3,
            19, 46, 74, 17, 98, 1, 95, 92, 86, 68, 36, 52, 72, 25, 50, 22, 38, 23, 18, 33, 15, 91,
            75, 81, 43, 80, 71, 28, 88, 8, 11, 10, 30, 16, 24, 41, 59, 32, 45, 90, 51, 96, 76, 89,
            49, 58, 20, 31, 55, 60, 79, 27, 37, 85, 99,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            34, 66, 3, 31, 9, 41, 81, 44, 11, 48, 51, 65, 50, 4, 77, 47, 10, 35, 19, 75, 32, 96, 1,
            76, 7, 62, 79, 42, 43, 16, 23, 37, 89, 12, 28, 56, 71, 93, 21, 49, 86, 67, 87, 6, 98,
            15, 95, 8, 78, 82, 29, 54, 58, 73, 85, 61, 88, 94, 57, 72, 27, 38, 40, 39, 97, 30, 14,
            92, 90, 69, 36, 55, 99, 60, 59, 74, 2, 22, 0, 13, 33, 46, 53, 20, 26, 18, 17, 5, 63,
            52, 25, 70, 83, 45, 64, 80, 24, 84, 68, 91,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            10, 68, 12, 38, 6, 94, 57, 97, 17, 98, 28, 31, 72, 11, 95, 5, 89, 7, 9, 41, 60, 86, 56,
            42, 55, 34, 47, 39, 80, 21, 69, 85, 59, 3, 44, 26, 70, 88, 90, 49, 84, 65, 32, 64, 29,
            24, 63, 37, 78, 19, 27, 74, 2, 52, 48, 62, 46, 43, 0, 67, 36, 50, 8, 40, 58, 54, 25,
            73, 71, 20, 99, 53, 87, 30, 76, 4, 23, 81, 15, 16, 18, 35, 61, 1, 93, 66, 13, 33, 82,
            45, 75, 22, 83, 96, 51, 79, 14, 92, 77, 91,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            99, 0, 42, 40, 80, 2, 36, 64, 20, 85, 62, 24, 94, 81, 19, 11, 87, 16, 10, 43, 66, 93,
            98, 78, 56, 37, 41, 74, 73, 72, 38, 3, 1, 33, 96, 75, 6, 18, 63, 9, 44, 89, 7, 34, 54,
            22, 8, 14, 35, 71, 95, 4, 79, 17, 67, 82, 46, 50, 13, 27, 76, 59, 29, 58, 90, 68, 65,
            52, 5, 86, 21, 49, 48, 12, 53, 45, 31, 69, 77, 88, 60, 39, 91, 28, 92, 97, 51, 47, 15,
            55, 32, 23, 57, 84, 30, 83, 70, 61, 25, 26,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            61, 59, 51, 67, 3, 32, 66, 34, 87, 7, 0, 4, 46, 94, 68, 47, 42, 8, 12, 44, 73, 22, 64,
            37, 13, 69, 35, 91, 15, 85, 6, 36, 58, 39, 1, 33, 53, 89, 20, 9, 40, 18, 95, 83, 96,
            76, 71, 60, 5, 72, 41, 2, 27, 93, 31, 16, 98, 88, 92, 11, 38, 55, 49, 29, 81, 70, 52,
            24, 90, 65, 10, 86, 63, 23, 30, 25, 62, 14, 19, 74, 57, 28, 77, 56, 82, 97, 84, 79, 78,
            17, 26, 99, 54, 80, 50, 45, 21, 43, 48, 75,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            65, 0, 31, 34, 10, 3, 64, 70, 9, 67, 4, 97, 63, 62, 96, 2, 41, 71, 39, 78, 98, 59, 1,
            27, 54, 11, 25, 13, 7, 46, 30, 22, 72, 99, 82, 47, 6, 48, 52, 94, 38, 66, 43, 88, 93,
            89, 79, 33, 83, 44, 8, 87, 57, 12, 92, 91, 68, 32, 18, 14, 40, 36, 23, 42, 53, 74, 51,
            21, 45, 5, 61, 35, 20, 58, 95, 15, 90, 16, 50, 49, 56, 37, 84, 73, 17, 86, 26, 29, 55,
            19, 24, 81, 28, 85, 69, 77, 80, 75, 76, 60,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            39, 63, 3, 43, 5, 77, 7, 6, 40, 4, 41, 65, 38, 86, 82, 95, 67, 88, 44, 62, 47, 66, 64,
            12, 37, 56, 19, 10, 74, 21, 61, 30, 76, 55, 13, 79, 15, 8, 50, 11, 33, 84, 59, 14, 9,
            25, 24, 73, 98, 29, 22, 83, 16, 70, 89, 87, 85, 96, 26, 52, 36, 17, 49, 71, 28, 23, 34,
            48, 18, 46, 57, 54, 35, 53, 45, 69, 42, 58, 97, 51, 20, 93, 80, 90, 2, 0, 31, 78, 99,
            91, 94, 92, 68, 81, 72, 60, 1, 75, 32, 27,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            74, 98, 82, 85, 67, 37, 64, 70, 96, 4, 62, 39, 94, 7, 88, 5, 38, 40, 69, 71, 36, 61,
            93, 18, 42, 17, 47, 13, 53, 11, 3, 2, 63, 24, 65, 72, 43, 10, 87, 76, 97, 79, 59, 41,
            6, 86, 16, 0, 91, 12, 66, 49, 77, 51, 1, 33, 44, 50, 19, 20, 8, 25, 32, 57, 31, 34, 84,
            95, 46, 68, 9, 30, 78, 27, 73, 52, 58, 21, 45, 54, 92, 28, 26, 83, 99, 15, 81, 22, 35,
            89, 60, 56, 55, 29, 48, 90, 14, 80, 75, 23,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            63, 37, 97, 62, 24, 50, 8, 91, 7, 2, 1, 84, 19, 10, 68, 16, 0, 90, 12, 6, 69, 27, 71,
            38, 40, 3, 79, 42, 14, 5, 66, 57, 39, 48, 53, 67, 64, 99, 59, 44, 56, 41, 98, 32, 49,
            55, 22, 9, 73, 65, 95, 60, 45, 13, 31, 11, 23, 78, 72, 17, 80, 58, 33, 87, 34, 36, 81,
            83, 52, 76, 43, 75, 28, 30, 94, 4, 74, 26, 21, 20, 77, 70, 86, 35, 96, 85, 54, 46, 18,
            51, 82, 61, 93, 15, 25, 29, 88, 89, 47, 92,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            10, 31, 34, 20, 77, 43, 45, 48, 36, 99, 86, 95, 69, 75, 24, 4, 40, 44, 2, 68, 3, 66,
            12, 37, 27, 39, 30, 70, 64, 9, 96, 63, 29, 7, 84, 72, 89, 53, 80, 5, 32, 6, 76, 33, 16,
            59, 14, 50, 17, 47, 22, 41, 87, 83, 35, 13, 85, 78, 91, 21, 88, 73, 8, 79, 55, 38, 92,
            56, 74, 11, 28, 23, 67, 57, 54, 46, 25, 94, 42, 93, 98, 65, 97, 26, 52, 90, 71, 60, 51,
            19, 61, 18, 0, 1, 58, 81, 62, 82, 49, 15,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            84, 52, 1, 36, 6, 82, 71, 63, 30, 5, 64, 42, 9, 61, 86, 3, 29, 10, 0, 44, 4, 60, 12,
            56, 92, 8, 34, 38, 77, 87, 80, 41, 25, 74, 93, 69, 85, 39, 79, 72, 46, 97, 57, 13, 35,
            23, 68, 31, 16, 90, 18, 51, 62, 50, 99, 7, 73, 65, 14, 33, 54, 96, 21, 40, 32, 89, 91,
            45, 67, 55, 53, 81, 37, 66, 2, 43, 28, 20, 19, 47, 27, 59, 22, 70, 95, 11, 75, 48, 98,
            26, 88, 78, 76, 17, 49, 94, 58, 83, 15, 24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            36, 81, 91, 42, 23, 16, 12, 47, 83, 8, 67, 9, 85, 46, 92, 1, 87, 71, 73, 6, 7, 4, 43,
            75, 10, 69, 74, 37, 78, 15, 0, 13, 33, 97, 95, 88, 48, 30, 45, 89, 40, 2, 5, 32, 24,
            68, 14, 11, 84, 51, 49, 19, 64, 86, 57, 94, 53, 72, 82, 41, 96, 90, 25, 62, 38, 65, 17,
            44, 34, 61, 63, 66, 55, 93, 56, 59, 35, 70, 26, 60, 39, 50, 52, 29, 99, 18, 79, 54, 77,
            98, 80, 58, 28, 3, 31, 22, 21, 27, 76, 20,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(true, puzzle.is_solvable(&goal));
        //Unolvable and size = 2
        let size: usize = 2;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![1, 3, 0, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 0, 3, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 0, 2, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 0, 1, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 0, 2, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 0, 2, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 1, 0, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 0, 3, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 0, 2, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 3, 0, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 0, 1, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 0, 2, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 3, 0, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 2, 0, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 2, 0, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 0, 1, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 2, 0, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 3, 0, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 2, 0, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 1, 0, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        //Unolvable and size = 3
        let size: usize = 3;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![5, 3, 4, 7, 0, 8, 1, 2, 6];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 3, 8, 2, 6, 5, 0, 7, 4];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![5, 6, 2, 8, 0, 4, 7, 3, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![7, 6, 3, 1, 8, 2, 0, 4, 5];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 1, 0, 5, 2, 6, 7, 4, 8];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![6, 1, 2, 7, 4, 8, 0, 5, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![0, 4, 7, 8, 1, 3, 5, 6, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![6, 8, 2, 5, 7, 4, 1, 3, 0];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![5, 1, 3, 2, 7, 8, 0, 6, 4];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![8, 2, 0, 5, 1, 6, 7, 4, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 4, 1, 5, 0, 7, 8, 6, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![7, 6, 1, 3, 4, 2, 0, 8, 5];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![0, 2, 4, 3, 7, 1, 5, 8, 6];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![6, 2, 5, 8, 4, 1, 0, 7, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![7, 5, 1, 4, 0, 3, 2, 8, 6];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![6, 1, 0, 7, 8, 5, 2, 3, 4];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![1, 6, 7, 8, 0, 2, 5, 3, 4];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![5, 7, 1, 2, 0, 8, 3, 4, 6];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![6, 3, 8, 7, 0, 5, 4, 2, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![7, 6, 5, 3, 0, 4, 8, 2, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        //Unolvable and size = 4
        let size: usize = 4;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![10, 7, 5, 8, 12, 9, 4, 13, 6, 14, 3, 2, 15, 11, 0, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![4, 0, 13, 7, 10, 11, 6, 3, 1, 8, 14, 5, 2, 9, 12, 15];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![8, 0, 1, 15, 4, 13, 3, 9, 6, 5, 14, 2, 7, 11, 10, 12];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![5, 7, 4, 13, 15, 6, 0, 9, 8, 3, 12, 1, 10, 14, 11, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![3, 5, 9, 15, 6, 14, 0, 7, 1, 8, 2, 13, 4, 10, 11, 12];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![4, 8, 15, 0, 14, 10, 6, 12, 2, 7, 5, 13, 1, 11, 3, 9];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![12, 0, 2, 7, 3, 1, 13, 4, 6, 9, 10, 8, 14, 15, 11, 5];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![15, 0, 6, 1, 7, 9, 5, 2, 14, 12, 3, 10, 8, 4, 11, 13];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![13, 0, 7, 9, 2, 8, 15, 1, 14, 10, 6, 12, 3, 4, 5, 11];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![4, 15, 8, 2, 5, 10, 0, 14, 1, 13, 6, 12, 7, 11, 9, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![10, 0, 14, 2, 3, 8, 15, 7, 11, 9, 12, 13, 4, 6, 5, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![12, 11, 5, 0, 2, 4, 10, 9, 15, 14, 8, 1, 3, 13, 6, 7];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![4, 0, 13, 15, 5, 9, 2, 14, 12, 8, 3, 7, 6, 1, 11, 10];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![4, 1, 11, 8, 10, 7, 5, 12, 6, 0, 14, 9, 13, 3, 15, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![10, 0, 11, 3, 15, 6, 14, 8, 4, 5, 9, 2, 13, 7, 12, 1];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![2, 10, 13, 3, 9, 12, 8, 15, 7, 5, 14, 6, 1, 4, 0, 11];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![7, 3, 6, 2, 0, 9, 11, 13, 10, 4, 8, 1, 5, 15, 12, 14];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![10, 11, 2, 4, 0, 15, 6, 7, 12, 1, 5, 8, 13, 14, 9, 3];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![4, 7, 3, 11, 13, 14, 8, 15, 12, 6, 9, 5, 0, 1, 10, 2];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![11, 12, 5, 10, 0, 15, 1, 3, 9, 7, 4, 2, 13, 8, 6, 14];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        //Unolvable and size = 5
        let size: usize = 5;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            18, 9, 19, 7, 13, 14, 17, 10, 0, 2, 15, 1, 21, 11, 16, 6, 8, 20, 5, 3, 4, 12, 24, 23,
            22,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            6, 3, 14, 8, 5, 1, 16, 21, 12, 11, 2, 10, 18, 7, 0, 13, 24, 20, 23, 22, 9, 15, 17, 4,
            19,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            20, 14, 9, 11, 23, 16, 18, 5, 22, 17, 6, 2, 13, 8, 0, 1, 19, 15, 21, 24, 4, 7, 12, 3,
            10,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            21, 11, 0, 9, 4, 6, 17, 18, 14, 16, 10, 1, 8, 3, 7, 13, 12, 23, 2, 22, 5, 19, 20, 15,
            24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            18, 4, 15, 6, 19, 10, 14, 13, 1, 11, 8, 20, 17, 3, 9, 22, 12, 24, 7, 16, 2, 5, 23, 21,
            0,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            7, 3, 22, 12, 23, 10, 9, 20, 11, 4, 6, 2, 24, 13, 8, 15, 1, 16, 0, 17, 21, 14, 19, 18,
            5,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            4, 16, 12, 7, 21, 9, 17, 13, 14, 8, 22, 3, 11, 19, 2, 5, 0, 15, 23, 10, 6, 20, 1, 18,
            24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            9, 20, 24, 22, 10, 7, 1, 3, 0, 5, 16, 6, 23, 13, 2, 14, 4, 17, 8, 11, 12, 19, 15, 21,
            18,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            9, 24, 6, 23, 14, 7, 0, 2, 19, 22, 5, 8, 12, 11, 13, 4, 10, 15, 18, 21, 17, 20, 3, 16,
            1,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            16, 24, 17, 3, 7, 13, 6, 22, 2, 4, 1, 15, 18, 20, 19, 21, 9, 23, 0, 8, 5, 11, 12, 14,
            10,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            13, 4, 23, 9, 24, 1, 20, 3, 7, 10, 18, 21, 16, 15, 0, 8, 12, 19, 2, 17, 5, 22, 11, 14,
            6,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            20, 21, 8, 24, 5, 13, 1, 15, 14, 19, 4, 23, 17, 10, 3, 9, 12, 6, 7, 2, 11, 18, 0, 22,
            16,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            6, 17, 0, 12, 8, 1, 22, 16, 14, 3, 9, 20, 7, 19, 24, 5, 13, 15, 21, 11, 10, 23, 2, 18,
            4,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            18, 12, 13, 8, 14, 20, 4, 22, 1, 6, 10, 16, 21, 2, 17, 24, 19, 11, 23, 3, 5, 15, 0, 7,
            9,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            1, 23, 6, 13, 16, 9, 18, 14, 0, 24, 5, 21, 11, 12, 17, 19, 15, 2, 22, 10, 3, 20, 4, 8,
            7,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            3, 19, 5, 8, 15, 7, 11, 16, 12, 4, 2, 18, 1, 17, 0, 13, 20, 24, 6, 14, 23, 22, 10, 9,
            21,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            4, 14, 1, 10, 8, 5, 6, 19, 23, 17, 13, 7, 16, 11, 22, 24, 9, 2, 15, 18, 20, 12, 0, 21,
            3,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            5, 16, 11, 19, 0, 1, 18, 4, 21, 12, 8, 7, 3, 2, 10, 13, 15, 23, 24, 14, 6, 22, 17, 9,
            20,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            10, 14, 0, 8, 15, 21, 13, 6, 3, 7, 20, 16, 17, 24, 22, 5, 23, 9, 19, 11, 18, 4, 2, 12,
            1,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            0, 13, 16, 3, 6, 5, 24, 12, 21, 9, 2, 18, 23, 7, 15, 10, 11, 20, 4, 19, 17, 1, 8, 14,
            22,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        //Unolvable and size = 6
        let size: usize = 6;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            21, 4, 31, 12, 11, 8, 6, 32, 29, 22, 16, 7, 26, 3, 33, 28, 34, 14, 35, 1, 17, 9, 0, 23,
            2, 24, 5, 30, 25, 19, 15, 10, 20, 27, 18, 13,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            35, 31, 19, 34, 15, 20, 10, 17, 14, 4, 25, 27, 30, 26, 24, 29, 3, 9, 1, 28, 7, 6, 8,
            16, 13, 21, 11, 18, 32, 33, 5, 2, 0, 12, 23, 22,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            24, 35, 5, 10, 13, 19, 0, 27, 20, 22, 14, 12, 28, 29, 7, 16, 23, 15, 21, 31, 11, 8, 3,
            1, 4, 26, 34, 18, 6, 33, 2, 9, 32, 17, 30, 25,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            15, 24, 13, 26, 20, 27, 14, 33, 25, 23, 8, 28, 17, 5, 30, 3, 22, 35, 1, 29, 11, 7, 6,
            19, 16, 9, 10, 0, 12, 4, 31, 34, 21, 32, 2, 18,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            1, 9, 24, 35, 3, 16, 15, 27, 19, 14, 7, 31, 33, 21, 34, 13, 10, 25, 26, 28, 2, 12, 23,
            17, 32, 0, 30, 20, 11, 5, 18, 8, 22, 6, 4, 29,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            2, 20, 3, 12, 29, 19, 17, 28, 4, 22, 7, 33, 18, 5, 32, 23, 13, 0, 6, 8, 27, 26, 16, 14,
            35, 9, 11, 1, 15, 30, 25, 24, 10, 21, 31, 34,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            13, 20, 28, 5, 17, 6, 2, 32, 31, 10, 7, 33, 34, 15, 26, 29, 8, 23, 16, 27, 19, 12, 1,
            14, 24, 0, 3, 25, 22, 9, 11, 18, 21, 4, 30, 35,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            35, 31, 3, 18, 5, 8, 25, 9, 4, 11, 21, 16, 15, 29, 7, 14, 34, 10, 22, 17, 27, 23, 30,
            28, 32, 6, 19, 12, 2, 24, 1, 33, 26, 13, 0, 20,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            30, 1, 12, 5, 22, 16, 24, 11, 23, 3, 28, 18, 35, 8, 19, 33, 15, 10, 34, 20, 17, 13, 6,
            32, 31, 27, 21, 0, 25, 2, 26, 14, 9, 4, 7, 29,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            15, 7, 32, 0, 19, 9, 27, 10, 25, 6, 24, 21, 18, 2, 4, 28, 34, 31, 12, 13, 20, 29, 33,
            11, 30, 35, 14, 26, 8, 1, 16, 5, 23, 22, 3, 17,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            1, 12, 30, 29, 13, 20, 26, 6, 4, 17, 21, 32, 22, 2, 7, 11, 15, 3, 0, 31, 14, 18, 24, 9,
            25, 5, 10, 16, 34, 23, 27, 19, 33, 28, 35, 8,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            15, 3, 20, 10, 25, 33, 30, 23, 28, 35, 17, 18, 27, 6, 16, 32, 5, 21, 12, 4, 0, 1, 31,
            22, 2, 19, 14, 13, 9, 11, 29, 7, 8, 26, 24, 34,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            32, 27, 30, 22, 25, 31, 12, 21, 11, 28, 15, 8, 10, 0, 17, 2, 24, 19, 26, 13, 23, 20, 4,
            3, 29, 5, 6, 16, 35, 7, 9, 14, 1, 18, 34, 33,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            30, 34, 11, 3, 26, 29, 23, 35, 21, 16, 8, 18, 15, 22, 9, 20, 2, 24, 0, 17, 19, 12, 6,
            14, 13, 33, 25, 10, 1, 5, 7, 31, 4, 28, 27, 32,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            31, 20, 9, 7, 5, 4, 6, 15, 23, 10, 30, 24, 8, 32, 13, 26, 17, 16, 35, 28, 33, 27, 12,
            21, 14, 0, 11, 34, 18, 22, 25, 19, 3, 1, 2, 29,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            16, 14, 1, 20, 21, 32, 27, 18, 34, 6, 26, 17, 24, 5, 7, 31, 12, 29, 15, 25, 33, 2, 4,
            10, 30, 0, 3, 35, 8, 9, 11, 23, 13, 28, 19, 22,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            28, 5, 35, 13, 29, 26, 2, 10, 8, 24, 32, 33, 21, 9, 15, 7, 4, 3, 18, 17, 25, 19, 6, 14,
            31, 30, 16, 12, 22, 0, 11, 1, 27, 23, 34, 20,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            7, 15, 9, 30, 11, 0, 22, 28, 10, 5, 23, 25, 24, 20, 33, 29, 27, 2, 16, 18, 19, 31, 13,
            14, 8, 4, 3, 12, 17, 6, 26, 32, 35, 34, 21, 1,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            3, 20, 35, 6, 7, 30, 22, 1, 32, 23, 26, 33, 34, 25, 24, 8, 5, 31, 27, 13, 4, 29, 0, 9,
            21, 18, 28, 2, 11, 19, 15, 12, 17, 10, 16, 14,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            26, 17, 34, 25, 27, 18, 0, 35, 2, 29, 23, 28, 33, 13, 6, 20, 31, 30, 8, 16, 12, 1, 11,
            22, 9, 32, 3, 7, 4, 5, 21, 24, 19, 15, 10, 14,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        //Unolvable and size = 7
        let size: usize = 7;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            30, 7, 11, 42, 33, 18, 16, 35, 38, 47, 40, 15, 5, 41, 2, 22, 44, 10, 13, 17, 6, 29, 8,
            21, 43, 36, 34, 37, 9, 24, 23, 27, 4, 12, 45, 26, 39, 3, 0, 20, 32, 1, 28, 31, 48, 46,
            25, 14, 19,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            26, 9, 42, 18, 24, 30, 33, 31, 7, 14, 35, 45, 3, 6, 41, 22, 48, 28, 17, 29, 47, 23, 12,
            8, 36, 46, 0, 27, 13, 21, 20, 40, 34, 19, 25, 5, 4, 43, 37, 32, 44, 10, 1, 39, 15, 16,
            38, 11, 2,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            4, 13, 17, 20, 27, 12, 45, 44, 43, 24, 23, 9, 37, 1, 5, 25, 19, 11, 22, 31, 46, 28, 33,
            14, 39, 32, 15, 48, 29, 8, 36, 38, 7, 18, 6, 34, 30, 47, 2, 21, 40, 26, 16, 35, 41, 3,
            42, 10, 0,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            21, 3, 6, 24, 19, 43, 5, 11, 48, 35, 44, 40, 32, 30, 33, 46, 16, 12, 23, 26, 18, 27,
            15, 20, 2, 42, 1, 25, 28, 13, 7, 4, 0, 29, 38, 8, 47, 37, 14, 17, 36, 34, 9, 22, 41,
            45, 39, 31, 10,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            11, 39, 34, 1, 32, 9, 25, 43, 0, 22, 12, 41, 2, 42, 40, 33, 16, 18, 3, 14, 30, 5, 37,
            31, 4, 6, 13, 36, 47, 23, 35, 17, 44, 45, 8, 29, 27, 38, 10, 26, 48, 20, 24, 15, 21,
            46, 28, 7, 19,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            0, 26, 3, 6, 41, 32, 7, 17, 40, 4, 8, 18, 36, 15, 43, 33, 1, 20, 23, 9, 48, 39, 24, 29,
            30, 45, 19, 27, 22, 21, 47, 31, 46, 42, 12, 16, 38, 13, 2, 34, 35, 44, 25, 37, 11, 14,
            28, 5, 10,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            5, 25, 27, 45, 33, 37, 15, 20, 35, 14, 26, 39, 41, 12, 23, 32, 38, 7, 1, 31, 8, 18, 16,
            10, 22, 2, 42, 24, 46, 9, 11, 44, 0, 28, 40, 17, 21, 6, 34, 29, 4, 19, 13, 3, 47, 30,
            36, 48, 43,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            12, 44, 23, 35, 21, 36, 9, 40, 38, 37, 32, 16, 13, 2, 45, 42, 1, 17, 31, 34, 47, 25,
            18, 28, 29, 15, 0, 14, 10, 3, 4, 39, 7, 5, 8, 46, 26, 33, 20, 27, 41, 6, 11, 30, 19,
            43, 48, 22, 24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            35, 5, 11, 36, 47, 20, 19, 42, 34, 4, 24, 23, 27, 16, 29, 15, 38, 43, 37, 18, 8, 48,
            28, 9, 2, 12, 25, 32, 0, 40, 17, 22, 33, 1, 13, 44, 45, 21, 3, 14, 46, 41, 7, 6, 30,
            26, 10, 39, 31,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            28, 21, 29, 15, 0, 3, 13, 1, 24, 26, 16, 34, 48, 11, 38, 5, 4, 31, 46, 22, 25, 43, 12,
            18, 30, 45, 42, 10, 17, 40, 7, 37, 23, 19, 47, 2, 8, 6, 41, 20, 35, 33, 39, 14, 27, 36,
            44, 32, 9,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            4, 40, 46, 6, 30, 25, 45, 24, 41, 9, 7, 48, 37, 14, 0, 42, 34, 47, 35, 5, 31, 15, 16,
            2, 28, 23, 33, 26, 44, 21, 19, 12, 13, 18, 20, 38, 1, 8, 11, 3, 10, 22, 36, 39, 29, 32,
            27, 43, 17,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            26, 46, 47, 17, 16, 35, 2, 33, 5, 28, 34, 24, 48, 15, 8, 20, 27, 37, 29, 6, 0, 40, 3,
            13, 1, 31, 36, 10, 23, 22, 14, 19, 41, 18, 9, 7, 39, 42, 44, 21, 38, 11, 32, 12, 45,
            30, 4, 43, 25,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            10, 26, 0, 38, 27, 3, 44, 11, 6, 15, 46, 28, 2, 20, 12, 32, 19, 7, 42, 17, 39, 33, 23,
            29, 21, 37, 25, 45, 22, 5, 41, 24, 35, 16, 43, 36, 48, 8, 40, 30, 18, 1, 14, 47, 31,
            13, 4, 9, 34,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            1, 2, 41, 42, 39, 4, 32, 12, 13, 47, 8, 21, 46, 26, 45, 7, 10, 31, 38, 35, 37, 28, 16,
            24, 43, 19, 30, 15, 23, 44, 48, 6, 17, 29, 3, 20, 34, 33, 40, 11, 22, 18, 0, 36, 5, 27,
            14, 9, 25,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            25, 10, 23, 2, 26, 1, 3, 20, 46, 42, 35, 17, 32, 19, 27, 41, 30, 8, 36, 12, 48, 24, 29,
            9, 13, 14, 43, 31, 38, 18, 0, 47, 21, 11, 22, 37, 39, 6, 28, 4, 40, 34, 16, 7, 33, 45,
            44, 15, 5,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            14, 41, 9, 24, 33, 16, 15, 46, 32, 39, 0, 34, 2, 31, 6, 21, 38, 20, 12, 45, 30, 1, 10,
            4, 47, 44, 26, 25, 18, 22, 8, 43, 17, 36, 28, 37, 3, 13, 42, 19, 35, 29, 48, 11, 27,
            23, 40, 7, 5,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            38, 42, 4, 17, 44, 45, 18, 22, 23, 1, 41, 6, 15, 36, 13, 7, 46, 5, 48, 14, 0, 12, 37,
            10, 39, 2, 20, 25, 16, 43, 27, 33, 21, 31, 26, 11, 34, 29, 8, 28, 47, 35, 40, 19, 32,
            24, 30, 3, 9,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            38, 43, 18, 9, 21, 41, 17, 16, 4, 1, 28, 34, 45, 22, 26, 11, 31, 42, 40, 5, 14, 23, 24,
            44, 25, 39, 8, 48, 13, 37, 35, 15, 30, 29, 6, 36, 47, 7, 32, 19, 3, 12, 10, 2, 0, 27,
            46, 20, 33,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            32, 25, 46, 21, 47, 44, 12, 6, 18, 2, 17, 22, 0, 9, 45, 5, 42, 14, 27, 36, 1, 26, 15,
            3, 34, 4, 41, 7, 31, 48, 24, 28, 20, 16, 43, 11, 23, 35, 29, 39, 30, 40, 37, 33, 10,
            38, 13, 19, 8,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            29, 19, 36, 46, 10, 48, 28, 11, 47, 25, 27, 23, 0, 18, 44, 6, 4, 33, 43, 37, 22, 2, 40,
            7, 3, 21, 30, 24, 39, 32, 1, 45, 38, 20, 13, 16, 17, 34, 12, 41, 5, 15, 31, 8, 14, 35,
            9, 26, 42,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        //Unolvable and size = 8
        let size: usize = 8;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            13, 1, 48, 23, 29, 49, 2, 58, 35, 20, 19, 5, 63, 8, 33, 11, 44, 62, 42, 0, 57, 59, 14,
            22, 47, 6, 21, 25, 46, 7, 31, 55, 36, 38, 17, 32, 37, 51, 15, 41, 28, 9, 3, 54, 27, 40,
            12, 43, 61, 10, 45, 56, 60, 26, 16, 4, 30, 24, 39, 34, 53, 18, 52, 50,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            56, 20, 54, 55, 50, 3, 24, 62, 16, 35, 46, 2, 61, 17, 31, 60, 52, 48, 15, 26, 41, 34,
            51, 39, 33, 23, 22, 49, 57, 44, 4, 28, 14, 47, 59, 53, 10, 5, 45, 37, 27, 58, 13, 30,
            29, 9, 32, 1, 6, 0, 19, 12, 38, 11, 7, 25, 40, 8, 63, 42, 43, 18, 36, 21,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            56, 4, 60, 13, 53, 22, 48, 0, 5, 63, 51, 18, 9, 2, 62, 23, 29, 14, 54, 10, 32, 44, 7,
            41, 6, 52, 30, 15, 43, 20, 39, 24, 16, 49, 33, 57, 50, 31, 59, 47, 25, 46, 26, 36, 8,
            28, 19, 45, 40, 35, 37, 27, 38, 12, 55, 11, 34, 3, 1, 17, 21, 61, 58, 42,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            37, 57, 24, 41, 12, 45, 26, 9, 63, 34, 62, 8, 29, 47, 40, 50, 30, 36, 38, 4, 56, 55,
            39, 61, 49, 13, 42, 31, 53, 7, 5, 17, 27, 22, 51, 16, 19, 35, 10, 46, 18, 48, 2, 32,
            23, 54, 25, 6, 14, 58, 28, 11, 3, 59, 1, 15, 44, 21, 0, 20, 33, 60, 43, 52,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            39, 16, 14, 22, 44, 56, 63, 17, 37, 59, 7, 40, 61, 38, 58, 4, 36, 5, 25, 31, 49, 47,
            53, 3, 1, 19, 0, 60, 45, 35, 26, 8, 41, 43, 6, 11, 57, 28, 52, 29, 24, 34, 20, 32, 48,
            46, 50, 18, 30, 15, 13, 27, 54, 62, 9, 51, 10, 2, 21, 42, 23, 33, 55, 12,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            23, 10, 9, 24, 20, 28, 42, 52, 55, 21, 48, 57, 47, 29, 19, 46, 49, 1, 30, 11, 5, 56,
            38, 14, 62, 51, 43, 32, 63, 7, 6, 35, 44, 8, 26, 27, 33, 22, 2, 0, 61, 34, 16, 41, 15,
            60, 31, 37, 39, 45, 3, 40, 12, 58, 25, 13, 53, 36, 54, 18, 4, 50, 17, 59,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            31, 58, 46, 59, 40, 47, 11, 25, 1, 60, 20, 3, 27, 35, 50, 33, 13, 63, 9, 22, 36, 41,
            17, 8, 44, 39, 6, 55, 24, 2, 0, 28, 34, 57, 16, 52, 21, 38, 23, 4, 49, 29, 7, 51, 19,
            54, 26, 48, 12, 32, 53, 42, 10, 56, 62, 37, 45, 14, 5, 43, 15, 30, 18, 61,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            17, 11, 27, 13, 29, 19, 50, 24, 46, 58, 21, 3, 5, 32, 52, 51, 45, 12, 1, 37, 25, 4, 53,
            0, 18, 9, 8, 59, 57, 61, 38, 35, 63, 28, 49, 41, 34, 39, 33, 42, 10, 31, 54, 36, 60, 6,
            16, 44, 14, 40, 23, 30, 48, 55, 15, 43, 56, 47, 62, 2, 20, 7, 26, 22,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            55, 52, 59, 54, 25, 41, 18, 6, 45, 13, 56, 63, 4, 28, 42, 16, 3, 27, 9, 35, 8, 58, 17,
            51, 47, 24, 40, 62, 21, 57, 14, 39, 32, 44, 53, 26, 43, 19, 22, 46, 0, 49, 7, 31, 48,
            15, 61, 38, 11, 20, 50, 29, 34, 23, 1, 37, 2, 33, 10, 5, 12, 60, 30, 36,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            47, 57, 11, 28, 8, 30, 14, 36, 49, 21, 7, 29, 26, 25, 24, 58, 31, 23, 50, 5, 61, 45,
            18, 32, 59, 56, 3, 16, 51, 19, 12, 62, 17, 46, 2, 27, 37, 39, 40, 33, 13, 55, 20, 44,
            38, 22, 9, 42, 52, 15, 4, 0, 43, 10, 1, 6, 41, 35, 34, 54, 48, 63, 53, 60,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            56, 0, 50, 61, 8, 34, 20, 3, 5, 23, 48, 44, 57, 14, 7, 32, 1, 15, 22, 19, 12, 52, 11,
            30, 25, 26, 51, 31, 62, 10, 55, 33, 29, 16, 40, 43, 53, 17, 35, 54, 13, 42, 47, 39, 60,
            38, 4, 36, 58, 45, 37, 63, 6, 27, 41, 59, 21, 49, 24, 28, 2, 9, 46, 18,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            54, 51, 30, 58, 47, 62, 45, 28, 52, 18, 19, 13, 6, 15, 61, 37, 2, 3, 17, 56, 60, 35,
            39, 7, 32, 55, 26, 10, 11, 1, 23, 48, 9, 38, 4, 31, 34, 0, 42, 44, 63, 46, 53, 29, 24,
            5, 50, 12, 20, 36, 22, 49, 21, 33, 14, 43, 57, 41, 59, 25, 40, 16, 8, 27,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            48, 44, 26, 23, 46, 7, 36, 35, 10, 62, 16, 9, 24, 37, 43, 34, 41, 0, 2, 42, 53, 29, 27,
            40, 52, 30, 49, 32, 59, 54, 6, 5, 28, 58, 21, 8, 51, 38, 12, 57, 19, 31, 25, 4, 17, 3,
            33, 47, 22, 55, 11, 60, 1, 18, 56, 61, 39, 50, 20, 63, 14, 45, 13, 15,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            31, 39, 22, 48, 30, 41, 12, 55, 24, 8, 6, 59, 29, 17, 34, 54, 7, 52, 1, 25, 32, 18, 35,
            63, 15, 36, 3, 19, 56, 51, 28, 46, 37, 4, 14, 45, 11, 58, 47, 23, 21, 43, 60, 5, 27,
            40, 26, 33, 49, 9, 10, 42, 20, 53, 57, 44, 61, 62, 2, 50, 0, 38, 16, 13,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            61, 42, 31, 54, 37, 18, 58, 46, 2, 20, 8, 9, 21, 34, 10, 15, 7, 4, 12, 27, 14, 0, 29,
            11, 28, 50, 22, 38, 39, 63, 60, 45, 57, 47, 53, 19, 44, 41, 49, 36, 48, 55, 16, 24, 13,
            62, 40, 56, 6, 26, 30, 5, 25, 23, 43, 32, 52, 17, 59, 1, 51, 3, 35, 33,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            19, 46, 5, 31, 36, 56, 1, 35, 47, 25, 58, 18, 40, 57, 37, 59, 12, 39, 27, 3, 54, 11,
            63, 62, 7, 8, 30, 44, 28, 51, 16, 22, 32, 33, 52, 29, 49, 53, 24, 23, 2, 13, 43, 4, 50,
            38, 48, 60, 15, 0, 9, 20, 41, 26, 10, 17, 34, 42, 55, 61, 45, 14, 6, 21,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            34, 3, 1, 7, 21, 8, 9, 43, 49, 24, 6, 46, 32, 55, 0, 15, 23, 50, 10, 58, 37, 54, 17,
            28, 63, 41, 42, 40, 16, 13, 59, 2, 51, 61, 36, 53, 35, 52, 25, 4, 5, 11, 30, 27, 14,
            26, 47, 56, 33, 18, 45, 12, 60, 62, 44, 38, 48, 22, 20, 19, 57, 39, 29, 31,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            56, 5, 57, 7, 20, 49, 37, 38, 55, 27, 63, 31, 16, 50, 28, 54, 3, 46, 42, 52, 51, 59,
            48, 17, 0, 1, 53, 29, 14, 30, 26, 4, 61, 58, 15, 43, 13, 11, 22, 9, 41, 24, 35, 36, 32,
            10, 45, 39, 33, 34, 40, 2, 23, 8, 6, 18, 44, 60, 62, 47, 19, 12, 25, 21,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            63, 16, 13, 15, 37, 59, 49, 21, 46, 23, 27, 35, 34, 12, 40, 11, 48, 50, 47, 56, 2, 36,
            26, 52, 22, 10, 32, 38, 0, 25, 41, 6, 42, 28, 45, 24, 5, 18, 1, 8, 9, 53, 58, 39, 51,
            7, 30, 20, 57, 62, 3, 60, 61, 33, 17, 54, 55, 4, 29, 31, 19, 44, 14, 43,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            10, 58, 30, 60, 23, 35, 59, 33, 18, 55, 61, 20, 49, 1, 3, 45, 47, 27, 11, 0, 40, 34, 7,
            8, 29, 41, 56, 24, 25, 14, 17, 43, 63, 53, 57, 44, 22, 39, 31, 13, 26, 42, 9, 21, 6, 2,
            5, 32, 51, 12, 36, 37, 38, 19, 50, 62, 54, 16, 46, 28, 4, 52, 15, 48,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        //Unolvable and size = 9
        let size: usize = 9;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            41, 40, 6, 69, 39, 76, 49, 80, 10, 57, 0, 12, 36, 54, 15, 70, 38, 17, 71, 2, 30, 73,
            61, 78, 9, 32, 79, 59, 53, 27, 64, 51, 47, 11, 67, 37, 18, 24, 77, 62, 65, 7, 23, 74,
            29, 55, 20, 8, 75, 14, 5, 46, 44, 19, 4, 68, 3, 35, 22, 42, 56, 13, 66, 48, 50, 21, 28,
            60, 31, 1, 72, 45, 43, 26, 33, 16, 34, 25, 58, 52, 63,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            32, 57, 33, 56, 36, 62, 76, 69, 54, 4, 38, 30, 2, 8, 44, 15, 37, 19, 78, 77, 24, 14,
            64, 5, 42, 40, 58, 39, 26, 1, 6, 52, 72, 34, 10, 35, 63, 55, 60, 3, 11, 48, 0, 47, 49,
            65, 25, 80, 59, 13, 17, 31, 66, 23, 51, 16, 20, 29, 27, 67, 41, 22, 75, 73, 12, 53, 9,
            74, 7, 68, 43, 50, 71, 46, 21, 18, 61, 45, 70, 79, 28,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            32, 53, 73, 74, 42, 46, 20, 21, 51, 1, 15, 11, 26, 6, 40, 45, 68, 19, 0, 71, 12, 69,
            59, 41, 30, 64, 24, 34, 29, 33, 38, 39, 62, 8, 3, 7, 55, 14, 13, 43, 2, 54, 48, 67, 44,
            58, 70, 9, 80, 27, 56, 57, 47, 16, 31, 50, 49, 37, 36, 17, 72, 18, 35, 23, 61, 79, 22,
            25, 66, 75, 77, 10, 5, 60, 65, 78, 52, 28, 63, 4, 76,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            30, 56, 52, 79, 39, 5, 64, 11, 70, 33, 54, 75, 46, 29, 49, 1, 20, 8, 51, 34, 71, 43,
            76, 41, 3, 57, 59, 6, 55, 61, 15, 38, 67, 42, 21, 47, 53, 44, 31, 27, 65, 7, 28, 12, 4,
            77, 35, 13, 69, 68, 17, 50, 22, 66, 10, 2, 9, 14, 23, 48, 0, 19, 18, 72, 25, 78, 80,
            24, 40, 60, 62, 16, 26, 36, 45, 63, 32, 37, 73, 58, 74,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            40, 28, 60, 43, 34, 9, 33, 29, 64, 59, 24, 17, 22, 8, 3, 37, 39, 70, 13, 38, 79, 35,
            30, 45, 65, 72, 66, 74, 10, 27, 54, 2, 20, 75, 6, 57, 36, 7, 68, 56, 11, 46, 51, 31, 5,
            67, 71, 77, 25, 53, 14, 76, 18, 15, 12, 19, 49, 78, 50, 41, 42, 52, 48, 55, 47, 1, 26,
            4, 23, 80, 69, 61, 58, 32, 21, 62, 0, 16, 63, 73, 44,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            52, 27, 33, 37, 19, 10, 70, 38, 29, 21, 23, 42, 6, 22, 80, 1, 76, 61, 73, 35, 43, 68,
            5, 34, 60, 67, 56, 41, 45, 69, 74, 3, 78, 9, 49, 7, 36, 40, 39, 11, 46, 28, 8, 14, 4,
            54, 30, 72, 58, 59, 25, 65, 18, 26, 12, 13, 31, 50, 0, 63, 71, 48, 20, 55, 17, 2, 24,
            53, 51, 32, 62, 57, 44, 75, 16, 77, 79, 47, 66, 15, 64,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            34, 49, 71, 58, 2, 23, 18, 42, 48, 4, 76, 51, 10, 63, 57, 26, 39, 77, 33, 72, 14, 7, 1,
            47, 69, 40, 0, 67, 28, 22, 5, 3, 30, 9, 35, 61, 31, 73, 74, 59, 41, 78, 70, 56, 12, 27,
            65, 75, 37, 24, 21, 45, 17, 11, 54, 38, 53, 25, 60, 19, 62, 32, 50, 80, 64, 6, 55, 52,
            43, 36, 46, 66, 20, 44, 29, 15, 68, 13, 79, 8, 16,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            79, 71, 16, 12, 49, 1, 46, 42, 43, 55, 64, 62, 33, 73, 76, 5, 78, 6, 9, 14, 66, 10, 22,
            74, 38, 47, 61, 17, 3, 31, 60, 20, 23, 19, 2, 41, 57, 50, 80, 24, 35, 77, 32, 70, 7,
            11, 53, 39, 58, 25, 45, 52, 27, 18, 48, 13, 75, 72, 8, 15, 69, 26, 37, 30, 59, 68, 21,
            51, 28, 4, 29, 65, 44, 56, 0, 67, 54, 36, 34, 40, 63,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            0, 29, 59, 76, 5, 48, 66, 30, 63, 25, 1, 64, 37, 13, 67, 38, 74, 6, 56, 62, 46, 45, 55,
            40, 43, 65, 14, 31, 9, 7, 12, 77, 54, 61, 41, 52, 2, 36, 34, 26, 51, 57, 73, 10, 50,
            80, 32, 58, 71, 23, 44, 17, 16, 8, 68, 33, 11, 24, 78, 60, 75, 18, 20, 3, 79, 4, 22,
            49, 15, 47, 35, 19, 53, 72, 27, 28, 70, 69, 42, 21, 39,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            50, 1, 58, 34, 8, 11, 4, 70, 77, 68, 3, 12, 72, 47, 9, 71, 42, 57, 2, 23, 24, 56, 5,
            44, 66, 64, 75, 7, 25, 36, 31, 21, 32, 63, 74, 26, 29, 27, 46, 73, 18, 48, 61, 60, 20,
            14, 35, 30, 22, 59, 19, 38, 79, 67, 51, 69, 0, 17, 10, 39, 54, 37, 65, 13, 78, 53, 62,
            28, 45, 41, 76, 49, 80, 52, 40, 33, 43, 6, 15, 55, 16,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            74, 80, 32, 59, 57, 34, 63, 44, 52, 8, 30, 67, 21, 28, 25, 40, 72, 1, 69, 14, 12, 2,
            37, 68, 4, 5, 41, 42, 73, 15, 35, 38, 39, 29, 50, 36, 61, 31, 56, 49, 33, 20, 19, 66,
            3, 27, 54, 64, 78, 13, 23, 18, 79, 46, 7, 26, 60, 17, 53, 71, 22, 47, 10, 43, 55, 58,
            0, 65, 9, 16, 76, 48, 45, 77, 6, 75, 11, 24, 70, 62, 51,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            53, 3, 60, 30, 6, 37, 55, 41, 12, 26, 64, 66, 62, 4, 28, 68, 59, 5, 39, 27, 33, 50, 23,
            15, 11, 34, 65, 79, 75, 67, 76, 24, 10, 58, 48, 46, 18, 69, 52, 1, 25, 74, 54, 61, 42,
            22, 8, 31, 49, 44, 73, 32, 38, 43, 47, 72, 20, 77, 45, 57, 21, 13, 7, 35, 14, 78, 2, 9,
            0, 29, 56, 16, 36, 51, 71, 17, 80, 40, 63, 19, 70,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            1, 57, 32, 4, 5, 50, 3, 61, 76, 67, 30, 73, 62, 14, 80, 65, 15, 10, 38, 36, 29, 47, 53,
            34, 20, 9, 13, 2, 31, 68, 56, 78, 33, 79, 41, 52, 11, 59, 72, 39, 19, 24, 28, 8, 22,
            26, 51, 70, 35, 12, 23, 48, 40, 69, 25, 71, 77, 66, 49, 45, 7, 63, 18, 44, 42, 43, 55,
            74, 17, 27, 75, 6, 0, 46, 37, 64, 58, 16, 60, 54, 21,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            34, 63, 78, 49, 36, 44, 52, 23, 33, 47, 61, 29, 7, 62, 14, 35, 3, 12, 21, 58, 24, 56,
            6, 76, 48, 8, 39, 65, 27, 74, 31, 43, 4, 77, 68, 53, 70, 80, 30, 59, 13, 25, 37, 46,
            11, 28, 19, 2, 69, 57, 0, 72, 20, 18, 79, 16, 17, 54, 1, 75, 32, 10, 9, 55, 26, 71, 60,
            64, 40, 45, 73, 67, 66, 50, 41, 38, 5, 51, 22, 15, 42,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            26, 68, 2, 60, 53, 3, 77, 79, 42, 13, 17, 54, 56, 37, 58, 40, 44, 65, 62, 34, 66, 25,
            6, 47, 1, 41, 7, 27, 32, 73, 49, 21, 43, 63, 18, 8, 59, 10, 70, 39, 11, 67, 14, 33, 52,
            9, 35, 20, 78, 12, 46, 50, 72, 24, 31, 57, 55, 76, 74, 69, 0, 80, 75, 4, 48, 23, 5, 64,
            45, 28, 36, 38, 29, 30, 61, 71, 22, 16, 51, 19, 15,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            22, 75, 12, 39, 0, 7, 11, 15, 9, 76, 59, 8, 69, 32, 60, 68, 10, 61, 53, 35, 50, 44, 23,
            57, 52, 6, 41, 80, 56, 2, 79, 77, 63, 37, 24, 78, 3, 14, 45, 49, 13, 55, 25, 42, 4, 48,
            30, 71, 40, 62, 33, 73, 21, 66, 58, 67, 54, 28, 1, 36, 20, 16, 43, 5, 46, 27, 74, 19,
            47, 26, 38, 18, 34, 51, 70, 31, 72, 65, 17, 29, 64,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            0, 32, 41, 73, 15, 35, 58, 63, 42, 38, 1, 14, 30, 59, 64, 65, 61, 8, 39, 6, 36, 44, 24,
            66, 49, 46, 34, 56, 69, 5, 2, 20, 62, 79, 45, 77, 71, 80, 12, 19, 72, 75, 47, 78, 43,
            3, 28, 51, 68, 76, 57, 40, 31, 60, 53, 25, 74, 10, 7, 22, 27, 26, 4, 48, 67, 55, 11,
            54, 37, 50, 13, 52, 29, 33, 70, 21, 16, 23, 17, 18, 9,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            49, 13, 78, 51, 37, 29, 20, 35, 7, 52, 73, 9, 76, 61, 19, 21, 34, 22, 8, 71, 4, 28, 31,
            15, 10, 36, 46, 33, 2, 47, 42, 48, 53, 1, 69, 54, 39, 57, 0, 65, 14, 58, 60, 68, 64,
            56, 77, 79, 74, 3, 45, 23, 11, 62, 38, 6, 32, 43, 50, 55, 12, 16, 70, 44, 25, 63, 17,
            72, 41, 30, 66, 24, 59, 67, 27, 5, 26, 18, 75, 40, 80,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            39, 41, 29, 58, 45, 61, 78, 13, 43, 77, 27, 69, 38, 6, 0, 42, 54, 36, 72, 10, 57, 73,
            52, 21, 19, 2, 51, 5, 60, 4, 62, 7, 16, 3, 37, 23, 71, 12, 74, 56, 1, 76, 55, 20, 75,
            35, 28, 17, 26, 11, 49, 40, 65, 8, 53, 79, 66, 24, 59, 18, 46, 44, 63, 47, 32, 22, 48,
            25, 14, 31, 68, 15, 9, 80, 70, 64, 33, 34, 30, 50, 67,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            61, 69, 8, 6, 62, 80, 17, 44, 71, 37, 72, 48, 3, 77, 28, 31, 56, 54, 19, 33, 32, 27,
            53, 43, 74, 7, 49, 22, 41, 4, 58, 21, 30, 52, 18, 10, 75, 29, 26, 79, 12, 42, 40, 57,
            9, 68, 25, 1, 67, 65, 15, 23, 34, 2, 47, 76, 55, 46, 5, 11, 0, 59, 39, 14, 13, 24, 20,
            38, 66, 70, 60, 73, 50, 78, 63, 64, 16, 45, 51, 36, 35,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        //Unolvable and size = 10
        let size: usize = 10;
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(size), size, 0);
        let vector: Vec<u8> = vec![
            41, 45, 62, 3, 85, 57, 99, 4, 11, 88, 68, 66, 12, 64, 6, 36, 34, 2, 46, 38, 51, 13, 55,
            35, 14, 77, 33, 73, 9, 17, 71, 44, 70, 65, 60, 69, 10, 20, 98, 92, 83, 61, 95, 76, 5,
            0, 15, 93, 74, 8, 91, 80, 23, 7, 16, 1, 40, 43, 89, 78, 28, 87, 48, 79, 96, 30, 72, 53,
            42, 82, 94, 22, 58, 54, 25, 50, 63, 59, 47, 32, 90, 81, 49, 67, 84, 86, 18, 97, 21, 75,
            27, 39, 19, 56, 31, 29, 52, 26, 37, 24,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            84, 54, 78, 70, 26, 38, 14, 63, 12, 34, 60, 97, 39, 47, 6, 10, 5, 90, 72, 91, 57, 92,
            94, 66, 36, 31, 9, 49, 43, 13, 64, 96, 51, 1, 67, 40, 0, 20, 50, 86, 83, 29, 7, 4, 37,
            69, 24, 99, 87, 41, 27, 22, 75, 79, 71, 2, 21, 46, 93, 45, 8, 3, 55, 18, 74, 32, 82,
            23, 42, 19, 58, 28, 81, 68, 11, 77, 48, 52, 80, 95, 89, 85, 61, 73, 30, 98, 65, 17, 62,
            33, 59, 88, 35, 76, 56, 16, 44, 53, 15, 25,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            92, 4, 98, 43, 86, 82, 88, 83, 19, 3, 5, 2, 42, 6, 94, 64, 7, 74, 9, 39, 36, 84, 38,
            48, 33, 21, 15, 41, 72, 89, 96, 29, 32, 10, 8, 78, 25, 75, 45, 11, 97, 26, 79, 17, 61,
            44, 65, 95, 13, 91, 18, 80, 56, 12, 1, 35, 55, 54, 31, 53, 85, 51, 99, 62, 68, 0, 40,
            66, 24, 20, 37, 14, 22, 23, 28, 50, 57, 16, 27, 77, 63, 81, 30, 58, 71, 46, 59, 76, 87,
            47, 60, 34, 90, 52, 70, 73, 49, 69, 67, 93,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            25, 82, 5, 97, 92, 12, 1, 90, 4, 44, 64, 42, 37, 76, 45, 39, 32, 93, 21, 87, 63, 31,
            36, 0, 60, 95, 6, 13, 51, 43, 55, 66, 20, 34, 98, 86, 88, 70, 84, 99, 10, 40, 29, 61,
            52, 75, 17, 3, 77, 8, 71, 38, 2, 68, 91, 80, 46, 19, 15, 14, 30, 58, 73, 16, 65, 23,
            85, 28, 22, 79, 59, 33, 18, 35, 96, 62, 41, 89, 50, 24, 83, 53, 57, 26, 81, 54, 11, 67,
            49, 48, 7, 94, 78, 56, 9, 27, 69, 74, 47, 72,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            89, 26, 10, 97, 4, 53, 9, 43, 13, 11, 3, 93, 62, 75, 86, 92, 42, 73, 83, 1, 36, 40, 38,
            41, 46, 69, 67, 57, 77, 87, 78, 60, 84, 14, 58, 7, 0, 5, 20, 68, 82, 94, 29, 88, 54,
            23, 80, 8, 55, 44, 66, 45, 32, 95, 63, 70, 6, 52, 48, 98, 39, 85, 33, 21, 12, 2, 49,
            18, 76, 15, 31, 34, 64, 35, 90, 25, 59, 61, 27, 51, 65, 28, 16, 71, 74, 19, 56, 30, 24,
            72, 22, 37, 96, 91, 81, 47, 99, 50, 79, 17,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            86, 5, 1, 90, 3, 77, 44, 99, 45, 34, 83, 38, 0, 56, 37, 24, 73, 72, 9, 47, 4, 88, 62,
            16, 20, 97, 63, 96, 6, 98, 28, 95, 81, 7, 32, 87, 8, 66, 41, 12, 80, 52, 17, 94, 13,
            92, 2, 15, 14, 31, 64, 36, 29, 35, 79, 51, 42, 10, 49, 22, 69, 93, 23, 54, 39, 76, 43,
            82, 40, 68, 75, 65, 50, 58, 85, 27, 71, 78, 25, 74, 84, 59, 30, 26, 57, 53, 55, 61, 11,
            48, 91, 60, 33, 67, 46, 18, 70, 21, 89, 19,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            93, 13, 38, 1, 76, 4, 47, 90, 12, 73, 8, 61, 81, 32, 53, 16, 83, 54, 10, 77, 66, 34,
            89, 71, 5, 50, 27, 74, 68, 70, 3, 37, 64, 22, 35, 69, 43, 20, 44, 99, 82, 88, 86, 87,
            33, 41, 30, 58, 11, 51, 85, 62, 24, 45, 67, 28, 65, 18, 91, 72, 9, 39, 31, 46, 94, 15,
            42, 78, 40, 6, 0, 7, 84, 59, 75, 55, 36, 63, 14, 49, 60, 25, 80, 97, 2, 79, 92, 19, 48,
            21, 98, 23, 26, 96, 57, 17, 29, 52, 56, 95,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            37, 59, 63, 72, 5, 79, 66, 4, 13, 29, 49, 23, 53, 11, 55, 8, 40, 56, 51, 12, 6, 35, 9,
            43, 68, 34, 7, 45, 1, 22, 2, 24, 54, 91, 65, 15, 41, 98, 82, 32, 84, 99, 60, 71, 57,
            27, 46, 14, 69, 17, 74, 64, 95, 38, 78, 58, 10, 44, 85, 70, 88, 89, 19, 21, 33, 31, 36,
            96, 92, 20, 83, 39, 97, 16, 3, 94, 77, 42, 50, 73, 25, 47, 67, 18, 26, 86, 48, 81, 76,
            0, 30, 90, 93, 28, 75, 87, 52, 80, 61, 62,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            82, 68, 34, 2, 31, 70, 93, 88, 67, 92, 7, 87, 63, 74, 4, 37, 51, 15, 8, 97, 10, 76, 39,
            44, 9, 54, 69, 81, 46, 64, 84, 38, 0, 57, 35, 71, 40, 43, 49, 42, 23, 33, 36, 32, 62,
            13, 48, 41, 78, 22, 52, 66, 26, 65, 98, 72, 6, 1, 50, 89, 56, 3, 61, 28, 77, 86, 94,
            45, 20, 18, 53, 96, 12, 80, 91, 17, 11, 21, 55, 16, 79, 60, 75, 85, 73, 14, 90, 5, 24,
            47, 29, 27, 59, 83, 30, 58, 95, 25, 99, 19,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            43, 59, 42, 69, 13, 62, 60, 93, 90, 51, 36, 68, 79, 7, 10, 96, 88, 44, 48, 40, 38, 1,
            37, 98, 35, 58, 86, 0, 89, 45, 91, 63, 3, 4, 41, 61, 92, 75, 14, 8, 66, 64, 33, 67, 28,
            11, 26, 6, 22, 9, 84, 57, 32, 99, 71, 82, 46, 72, 94, 50, 5, 80, 56, 87, 30, 70, 15,
            21, 52, 47, 54, 24, 53, 81, 31, 97, 73, 78, 49, 74, 23, 83, 29, 95, 65, 27, 77, 39, 17,
            19, 76, 25, 18, 34, 55, 2, 12, 85, 16, 20,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            31, 37, 62, 4, 26, 9, 73, 72, 40, 69, 88, 32, 99, 36, 38, 12, 18, 3, 49, 65, 1, 33, 44,
            47, 86, 41, 78, 68, 91, 43, 60, 87, 35, 54, 39, 92, 2, 66, 82, 6, 45, 30, 7, 85, 95, 0,
            59, 34, 51, 75, 90, 71, 96, 14, 22, 21, 94, 50, 55, 83, 10, 84, 81, 42, 93, 77, 25, 64,
            74, 56, 67, 80, 52, 24, 5, 29, 19, 13, 70, 28, 8, 76, 23, 63, 57, 53, 20, 48, 61, 16,
            89, 46, 97, 27, 58, 17, 98, 11, 79, 15,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            47, 34, 41, 61, 4, 18, 74, 69, 50, 8, 15, 32, 99, 89, 46, 60, 65, 98, 45, 66, 95, 67,
            39, 37, 72, 6, 90, 68, 14, 3, 40, 17, 42, 54, 22, 97, 88, 38, 9, 92, 87, 79, 94, 93,
            63, 0, 73, 27, 76, 24, 84, 5, 20, 86, 43, 58, 13, 77, 44, 21, 33, 48, 75, 25, 28, 1,
            51, 19, 2, 71, 36, 10, 96, 7, 91, 78, 12, 80, 57, 56, 16, 83, 62, 52, 30, 59, 49, 82,
            35, 53, 85, 23, 11, 29, 31, 26, 81, 64, 70, 55,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            86, 45, 40, 1, 16, 64, 77, 72, 7, 9, 33, 70, 5, 69, 28, 66, 46, 67, 87, 98, 36, 30, 6,
            65, 62, 10, 39, 32, 42, 14, 12, 95, 68, 22, 18, 4, 52, 90, 44, 83, 49, 96, 31, 82, 38,
            11, 81, 48, 91, 43, 84, 94, 3, 13, 41, 92, 76, 37, 99, 21, 88, 27, 57, 93, 60, 61, 15,
            50, 73, 47, 80, 34, 89, 35, 78, 74, 53, 20, 97, 56, 26, 0, 24, 71, 51, 55, 85, 8, 75,
            19, 2, 59, 54, 79, 23, 25, 58, 17, 63, 29,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            12, 41, 32, 96, 36, 78, 98, 88, 69, 22, 97, 4, 47, 34, 25, 29, 72, 70, 45, 87, 35, 8,
            31, 82, 5, 79, 89, 44, 7, 13, 39, 6, 14, 2, 93, 53, 19, 48, 55, 86, 3, 83, 65, 33, 40,
            66, 43, 42, 28, 92, 59, 75, 61, 67, 62, 26, 71, 91, 23, 51, 58, 63, 30, 27, 56, 38, 80,
            99, 52, 81, 46, 20, 76, 24, 85, 17, 11, 94, 49, 57, 9, 0, 64, 84, 18, 68, 16, 77, 90,
            54, 37, 73, 1, 60, 15, 21, 10, 74, 95, 50,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            62, 39, 59, 34, 63, 83, 67, 84, 38, 70, 6, 35, 91, 9, 46, 42, 81, 56, 30, 15, 4, 31,
            28, 8, 26, 55, 99, 65, 73, 49, 51, 1, 98, 64, 69, 89, 7, 13, 71, 17, 45, 48, 43, 0, 87,
            27, 82, 74, 54, 25, 36, 53, 94, 52, 47, 40, 61, 90, 5, 10, 80, 95, 77, 76, 72, 2, 78,
            50, 96, 3, 57, 58, 85, 29, 79, 86, 21, 44, 11, 23, 75, 24, 66, 33, 22, 97, 14, 41, 18,
            68, 88, 32, 12, 19, 93, 16, 20, 60, 37, 92,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            1, 0, 61, 66, 62, 57, 32, 4, 5, 92, 37, 97, 50, 8, 78, 73, 58, 43, 89, 46, 31, 96, 88,
            39, 27, 52, 28, 42, 93, 6, 34, 70, 65, 38, 67, 7, 86, 99, 11, 64, 84, 25, 94, 17, 55,
            98, 85, 33, 13, 95, 56, 2, 29, 12, 54, 47, 51, 45, 53, 40, 26, 72, 22, 48, 41, 49, 68,
            16, 3, 9, 63, 74, 60, 69, 15, 30, 14, 18, 20, 87, 79, 77, 21, 71, 75, 59, 91, 76, 90,
            19, 36, 24, 23, 80, 35, 81, 83, 10, 82, 44,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            44, 35, 62, 72, 45, 74, 15, 5, 78, 10, 32, 60, 36, 40, 7, 21, 80, 76, 23, 38, 14, 31,
            66, 64, 56, 9, 24, 0, 96, 48, 79, 51, 88, 58, 2, 8, 85, 91, 11, 33, 99, 50, 98, 68, 41,
            42, 73, 92, 1, 49, 61, 77, 37, 6, 54, 39, 22, 19, 43, 46, 59, 34, 65, 86, 4, 70, 17,
            93, 87, 25, 28, 63, 83, 55, 81, 71, 12, 52, 67, 69, 3, 84, 97, 95, 47, 13, 29, 16, 57,
            75, 20, 82, 26, 53, 27, 30, 18, 89, 90, 94,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            44, 2, 37, 5, 7, 14, 81, 52, 70, 75, 28, 45, 43, 33, 86, 16, 20, 8, 46, 85, 11, 97, 32,
            77, 36, 99, 15, 55, 54, 13, 87, 29, 35, 84, 83, 10, 0, 42, 39, 68, 89, 67, 90, 59, 12,
            66, 9, 62, 74, 38, 57, 69, 51, 93, 98, 23, 47, 72, 48, 53, 61, 65, 4, 78, 92, 58, 19,
            17, 24, 71, 34, 27, 41, 64, 56, 26, 6, 73, 22, 95, 63, 40, 31, 60, 91, 50, 3, 76, 96,
            49, 80, 1, 30, 25, 94, 21, 88, 82, 79, 18,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            53, 72, 90, 84, 78, 3, 47, 11, 70, 68, 80, 62, 34, 63, 96, 13, 73, 4, 7, 98, 39, 45,
            35, 41, 43, 64, 69, 94, 26, 49, 36, 28, 2, 44, 85, 59, 12, 66, 40, 9, 8, 92, 71, 33,
            55, 29, 48, 95, 67, 14, 56, 65, 81, 20, 87, 57, 74, 42, 19, 88, 27, 37, 52, 83, 79, 1,
            31, 51, 17, 76, 97, 5, 58, 15, 93, 16, 6, 18, 38, 91, 60, 50, 30, 24, 61, 77, 25, 99,
            23, 22, 54, 86, 82, 32, 0, 10, 89, 21, 46, 75,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
        let vector: Vec<u8> = vec![
            44, 67, 84, 12, 3, 52, 66, 77, 8, 71, 47, 35, 0, 29, 94, 70, 53, 19, 43, 41, 40, 64, 2,
            96, 85, 38, 56, 46, 48, 7, 45, 95, 99, 61, 90, 76, 9, 11, 72, 59, 10, 51, 34, 88, 81,
            32, 69, 5, 30, 18, 98, 78, 60, 25, 22, 20, 93, 42, 83, 15, 1, 49, 62, 57, 4, 97, 28,
            33, 26, 17, 86, 65, 39, 31, 23, 27, 16, 50, 74, 55, 87, 73, 68, 79, 21, 92, 13, 54, 89,
            82, 24, 63, 91, 6, 58, 37, 75, 36, 80, 14,
        ];
        let puzzle: Puzzle = Puzzle::new(vector, size, 0);
        assert_eq!(false, puzzle.is_solvable(&goal));
    }
}
