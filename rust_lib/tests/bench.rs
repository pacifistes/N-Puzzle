#[cfg(test)]
mod tests {
    use rust_lib::resolver::generate::*;
    use rust_lib::resolver::parser::parse;
    use std::time::Instant;

    #[test]
    fn bench_parser() {

        assert!(parse(&"no_file".to_string()).is_err(), "the function parse should return err");
        let start = Instant::now();
        assert!(parse(&"/Users/bbrunell/npuzzle/puzzles/test3".to_string()).is_ok(), "the function parse should return ok");
        let elapsed = start.elapsed();
        println!("Time parse the file test : {:?}", elapsed);
    }

    #[test]
    fn bench_r_generate_sorted_state() {
        let start = Instant::now();
        assert_eq!(r_generate_sorted_state(4), vec![1,2,3,4,12,13,14,5,11,0,15,6,10,9,8,7]);
        let elapsed = start.elapsed();
        println!("Time generate sorted puzzle : {:?}", elapsed);
    }

}
