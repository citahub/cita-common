pub mod bench_tools {

    use rand::{thread_rng, Rng};

    pub fn random_bytes(len: usize) -> Vec<u8> {
        thread_rng().gen_iter::<u8>().take(len).collect::<Vec<u8>>()
    }
}
