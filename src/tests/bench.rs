pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod test {
    use crate::resolver::generate::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use crate::resolver::parser::parse;
    use crate::resolver::heuristic::*;
    use crate::resolver::puzzle::*;
    use crate::run;
    use std::time::Instant;

    #[test]
    fn test_bench() {
        let start = Instant::now();

        let elapsed = start.elapsed();
        // debug format:
        println!("{:?}", elapsed);
        // or format as milliseconds:
        println!(
            "Elapsed: {} ms",
            (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64
        );
    }

    #[test]
    fn bench_parser() {
        let start = Instant::now();

        let puzzle: Puzzle = match parse(&"test".to_string()) {
            Ok(puzzle) => puzzle,
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
        };
        let elapsed = start.elapsed();
        println!("Time parse the file test : {:?}", elapsed);
    }

    #[test]
    fn bench_generate_sorted_puzzle() {
        let start = Instant::now();
        let goal: Puzzle = Puzzle::new(generate_sorted_puzzle(3), 3, 0);
        let elapsed = start.elapsed();
        println!("Time generate sorted puzzle : {:?}", elapsed);
    }

    #[test]
    fn bench_run() {
        let vector: Vec<usize> = vec![5, 6, 7, 2, 0, 4, 3, 8, 1];
        let puzzle: Puzzle = Puzzle::new(vector, 3, 0);

        let start = Instant::now();
        run(puzzle);
        let elapsed = start.elapsed();
        println!("{:?}", elapsed);
    }

    #[test]
    fn bench_collections() {
        let goal: Puzzle = Puzzle::new(generate_sorted_puzzle(3), 3, 0);
        let mut vector: Vec<Puzzle> = Vec::new();



        for i in 0..5058 {
            let mut start_state: Vec<usize> = generate_sorted_puzzle(3);
            let mut rng = thread_rng();
            start_state.shuffle(&mut rng);
            
            let mut puzzle = Puzzle::new(start_state, 3, (rand::random::<u8>() % 31) as usize);
            puzzle.init_h(&goal, manathan);
            vector.push(puzzle);
        }
    }
}
