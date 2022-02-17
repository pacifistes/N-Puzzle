#[cfg(test)]
mod tests {
    use rust_lib::resolver::generate::*;
    use std::time::Instant;

    #[test]
    fn bench_r_generate_sorted_state() {
        let start = Instant::now();
        assert_eq!(r_generate_sorted_state(4).unwrap(), vec![1,2,3,4,12,13,14,5,11,0,15,6,10,9,8,7]);
        let elapsed = start.elapsed();
        println!("Time generate sorted puzzle : {:?}", elapsed);
    }

}
