#[cfg(test)]
mod test {
    use crate::resolver::generate::*;
    use crate::resolver::heuristic::*;
    use crate::resolver::parser::parse;
    use crate::resolver::puzzle::*;
    use crate::run;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::time::Instant;
    // use std::collections::hash_map::HashMap;
    use std::collections::BTreeSet;
    use std::collections::BinaryHeap;
    use std::collections::HashSet;

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
    fn bench_r_generate_sorted_puzzle() {
        let start = Instant::now();
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(3), 3, 0);
        let elapsed = start.elapsed();
        println!("Time generate sorted puzzle : {:?}", elapsed);
    }

    #[test]
    fn bench_run() {
        let vector: Vec<u8> = vec![5, 6, 7, 2, 0, 4, 3, 8, 1];
        let puzzle: Puzzle = Puzzle::new(vector, 3, 0);

        let start = Instant::now();
        run(puzzle);
        let elapsed = start.elapsed();
        println!("{:?}", elapsed);
    }

    #[test]
    fn bench_collections() {
        // let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(3), 3, 0);
        // let mut vector: Vec<Puzzle> = Vec::new();

        // for i in 0..5058 {
        //     let mut start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        //     let mut rng = thread_rng();
        //     start_state.shuffle(&mut rng);

        //     let mut puzzle = Puzzle::new(start_state, 3, (rand::random::<u8>() % 31) as usize);
        //     puzzle.find_h(&goal, manathan);
        //     vector.push(puzzle);
        // }
        // let puzzle: Puzzle = vector[4055].clone();
        // let start = Instant::now();
        // if vector.contains(&puzzle) {
        //     println!("le puzzle est bien dans le vector");
        // }
        // let elapsed = start.elapsed();
        // println!("time for check in vector = {:?}", elapsed);

        let mut hash_set: HashSet<Puzzle> = HashSet::new();
        let start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        let mut puzzle = Puzzle::new(start_state, 3, 0);
        hash_set.insert(puzzle);

        let start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        let mut puzzle = Puzzle::new(start_state, 3, 10);
        hash_set.insert(puzzle);

        let start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        let mut puzzle = Puzzle::new(start_state, 3, 5);
        println!("***********************************************");

        dbg!(hash_set.iter().min().unwrap().clone());
        println!("***********************************************");
        dbg!(hash_set.contains(&puzzle));

        let mut btree_set: BTreeSet<Puzzle> = BTreeSet::new();
        let start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        let mut puzzle = Puzzle::new(start_state, 3, 0);
        btree_set.insert(puzzle);

        let start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        let mut puzzle = Puzzle::new(start_state, 3, 10);
        btree_set.insert(puzzle);

        let start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        let mut puzzle = Puzzle::new(start_state, 3, 5);
        println!("***********************************************");

        dbg!(btree_set.iter().min().unwrap().clone());
        println!("***********************************************");
        dbg!(btree_set.get(&puzzle).is_some());

        // std::BinaryHeap<ReversePartialOrd<T>>
        let mut binary_heap: BinaryHeap<Puzzle> = BinaryHeap::new();
        let start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        let mut puzzle = Puzzle::new(start_state, 3, 0);
        binary_heap.push(puzzle);

        let start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        let mut puzzle = Puzzle::new(start_state, 3, 10);
        binary_heap.push(puzzle);

        let start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        let mut puzzle = Puzzle::new(start_state, 3, 5);
        println!("***********************************************");

        println!("{:?}", binary_heap);
        dbg!(binary_heap.pop().unwrap().clone());
        println!("***********************************************");
        // dbg!(binary_heap.get(&puzzle).is_some());

        // dbg!(BTreeSet::from(binary_heap.into_vec()))
    }

    #[test]
    fn bench_find_position() {
        let goal: Puzzle = Puzzle::new(r_generate_sorted_puzzle(3), 3, 0);
        let mut start_state: Vec<u8> = r_generate_sorted_puzzle(3);
        let mut rng = thread_rng();
        start_state.shuffle(&mut rng);

        let mut puzzle = Puzzle::new(start_state, 3, (rand::random::<u8>() % 31) as usize);
        puzzle.find_h(&goal, manathan);
        let start = Instant::now();
        puzzle.get_index_of_value(0);
        let elapsed = start.elapsed();
        println!(
            "time for get position with position function =  {:?}",
            elapsed
        );
        let start = Instant::now();
        puzzle.state.binary_search_by(|probe| probe.cmp(&0));
        let elapsed = start.elapsed();
        println!(
            "time for get position with binary search function =  {:?}",
            elapsed
        );

        // let vec: Vec<u8> = vec![0,1,1,1,1,5,6];
        // vec.iter().sum()
    }
}
