use bit_vec::BitVec;
use rand::{self, Rng};
use std::fmt;

struct BinaryChromosome {
    bits: BitVec,
}

impl BinaryChromosome {
    fn new(size: usize) -> BinaryChromosome {
        let mut bits = BitVec::with_capacity(size);

        for _ in 0..size {
            bits.push(rand::random());
        }

        BinaryChromosome {
            bits
        }
    }

    fn zeroes(size: usize) -> BinaryChromosome {
        BinaryChromosome {
            bits: BitVec::from_elem(size, false)
        }
    }
}

impl fmt::Display for BinaryChromosome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for bit in self.bits.iter() {
            let char = if bit { '1' } else { '0' };
            write!(f, "{}", char)?;
        }

        Ok(())
    }
}

impl std::clone::Clone for BinaryChromosome {
    fn clone(&self) -> Self {
        BinaryChromosome {
            bits: self.bits.clone()
        }
    }
}

trait Mutation {
    type Chromosome;

    fn mutate(&self, parent: &Self::Chromosome) -> Self::Chromosome;
}

struct BinaryBitMutation {
    mutate_prob: f32,
}

impl BinaryBitMutation {
    fn new(mutate_prob: f32) -> Self {
        BinaryBitMutation {
            mutate_prob
        }
    }
}

impl Mutation for BinaryBitMutation {
    type Chromosome = BinaryChromosome;

    fn mutate(&self, parent: &Self::Chromosome) -> Self::Chromosome {
        let mut mutated = parent.clone();

        // Instead of checking for each bit individually if it should be flipped, this function
        // calculates which bits should be flipped. It calculates which bit to mutate next as
        // follows:
        // 
        //   offset = floor( ln(1 - rnd_val) / ln(1 - p) )
        //
        // Here "rnd_val" is a random value in the range of [0, 1] and "p" is the probability of
        // mutating a bit. The "offset" is relative to the current bit.
        //
        // You can derive the above formula yourself from:
        //
        //   P(n <= N) = 1 - (1 - p)^N
        //
        // Where P(n <= N) is the probability that at least one of the "N" next bits changes.
        let denom = (1.0 - self.mutate_prob).ln();
        let mut i = 0;
        loop {
            let num = (1.0 - rand::thread_rng().gen::<f32>()).ln();

            // Note: the cast rounds towards zero and maps the infinity float value and other
            // values that are "too big" to the maximum integer value, which is what we want.
            i += (num / denom) as usize;
            if i >= mutated.bits.len() {
                return mutated;
            }

            mutated.bits.set(i, !mutated.bits.get(i).unwrap());
            i += 1;
        }
    }
}

fn main() {
    let len = 256;
    let chromosome = BinaryChromosome::zeroes(len);
    println!("{}", chromosome);

    let prob = 0.1;
    let mutation = BinaryBitMutation::new(prob);
    let N = 1000;
    let mut total_flipped = 0;
    for _ in 0..N {
        let mutated = mutation.mutate(&chromosome);
        let flipped = mutated.bits.iter().filter(|x| *x).count();
        //println!("{} {}", mutated, flipped);
        total_flipped += flipped;
    }
    println!("flipped = {}, expected = {}", total_flipped, prob * (len * N) as f32);
}
