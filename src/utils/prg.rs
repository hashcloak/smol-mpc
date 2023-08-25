/// Implementation of a random number generator using AES-CTR
pub struct Prg {
    seed: Vec<u8>,
    counter: u64,
}

impl Prg {
    const PRG_NONCE: u64 = 0x0123456789ABCDEF;
    const PRG_INITIAL_COUNTER: u64 = 0;
    
    pub fn new(seed: Vec<u8>) -> Prg {
        // Provisional
        let block_len: usize = 8;

        let cropped_seed = if seed.len() > block_len {
            seed[0..block_len].to_vec()
        } else {
            seed
        };

        let counter = Self::PRG_INITIAL_COUNTER;

        let mut prg = Prg { seed: cropped_seed, counter };
        prg.init();
        prg
    }

    pub fn init(&mut self) {
        self.counter = Self::PRG_INITIAL_COUNTER;
        self.load_key();
    }

    pub fn load_key(&mut self) {
        // Expand seed to generate an IV and the corresponding key to encrypt.
        todo!() 
    }

    pub fn next(&self, n_bytes: usize) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();
        todo!()
    }
}
