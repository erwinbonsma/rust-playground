use bit_vec::BitVec;
use rand::{self, Rng};
use std::{cmp, fmt};

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

    fn ones(size: usize) -> BinaryChromosome {
        BinaryChromosome {
            bits: BitVec::from_elem(size, true)
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

trait Recombination {
    type Chromosome;

    fn recombine(
        &self, parent1: &Self::Chromosome, parent1: &Self::Chromosome
    ) -> Self::Chromosome;
}

struct BinaryNPointBitCrossover {
    n: usize,
}

impl BinaryNPointBitCrossover {
    fn new(n: usize) -> Self {
        BinaryNPointBitCrossover {
            n
        }
    }
}

impl Recombination for BinaryNPointBitCrossover {
    type Chromosome = BinaryChromosome;

    fn recombine(
        &self, parent1: &Self::Chromosome, parent2: &Self::Chromosome
    ) -> Self::Chromosome {

        let range = cmp::min(parent1.bits.len(), parent2.bits.len());
        let mut points: Vec<usize> = (0..self.n).map(
            |_| rand::thread_rng().gen_range(1..range)
        ).collect();
        &points[..].sort_unstable();

        if self.n % 2 == 1 {
            // Ensure that number of points is even
            points.push(parent1.bits.len());
        }

        let mut child = parent1.clone();
        for i in 0..points.len() / 2 {
            let from = points[i * 2];
            let to = points[i * 2 + 1];
            for j in from..to {
                child.bits.set(j, parent2.bits.get(j).unwrap());
            }
        }

        child
    }
}

trait EvolutionConfig<T: fmt::Display> {
    fn create(&self) -> T;
    fn mutate(&self, parent: &T) -> T;
    fn recombine(&self, parent1: &T, parent2: &T) -> T;
    fn evaluate(&self, subject: &T) -> f32;
}

struct Individual<T: fmt::Display> {
    chromosome: Box<T>,
    fitness: Option<f32>,
}

impl<T: fmt::Display> fmt::Display for Individual<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.chromosome)?;
        if let Some(fitness) = self.fitness {
            write!(f, " fitness = {}", fitness)?;
        }

        Ok(())
    }
}

struct GeneticAlgorithm<T: fmt::Display> {
    population: Vec<Individual<T>>,
    config: Box<dyn EvolutionConfig<T>>
}

impl<T: fmt::Display> GeneticAlgorithm<T> {
    fn evaluate(&mut self) {
        for indiv in self.population.iter_mut() {
            if let None = indiv.fitness {
                (*indiv).fitness = Some(self.config.evaluate(&indiv.chromosome));
            }
        }
    }
}

impl<T: fmt::Display> GeneticAlgorithm<T> {
    fn new(
        size: usize,
        config: Box<dyn EvolutionConfig<T>>
    ) -> Self {
        let population = (0..size).map(|_| Individual {
            chromosome: Box::new(config.create()),
            fitness: None
        }).collect();

        GeneticAlgorithm {
            population,
            config
        }
    }
}

impl<T: fmt::Display> fmt::Display for GeneticAlgorithm<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for individual in self.population.iter() {
            write!(f, "{}\n", individual)?;
        }

        Ok(())
    }
}

struct MyEvolutionConfig {
    mutation: BinaryBitMutation,
    recombination: BinaryNPointBitCrossover,
}

impl MyEvolutionConfig {
    fn new() -> Self {
        MyEvolutionConfig {
            mutation: BinaryBitMutation::new(0.1),
            recombination: BinaryNPointBitCrossover::new(2)
        }
    }
}

impl EvolutionConfig<BinaryChromosome> for MyEvolutionConfig {
    fn create(&self) -> BinaryChromosome {
        BinaryChromosome::new(16)
    }

    fn mutate(&self, parent: &BinaryChromosome) -> BinaryChromosome {
        self.mutation.mutate(parent)
    }

    fn recombine(&self, parent1: &BinaryChromosome, parent2: &BinaryChromosome) -> BinaryChromosome {
        self.recombination.recombine(parent1, parent2)
    }

    fn evaluate(&self, subject: &BinaryChromosome) -> f32 {
        // Count the number of ones
        subject.bits.iter().filter(|x| *x).count() as f32 / subject.bits.len() as f32
    }
}

fn test_creation() {
    for _ in 0..10 {
        let chromosome = BinaryChromosome::new(20);
        println!("{}", chromosome);
    }
}

fn test_mutation() {
    let len = 256;
    let chromosome = BinaryChromosome::zeroes(len);

    let prob = 0.1;
    let mutation = BinaryBitMutation::new(prob);
    let n = 1000;
    let mut total_flipped = 0;
    for _ in 0..n {
        let mutated = mutation.mutate(&chromosome);
        let flipped = mutated.bits.iter().filter(|x| *x).count();
        //println!("{} {}", mutated, flipped);
        total_flipped += flipped;
    }
    println!("flipped = {}, expected = {}", total_flipped, prob * (len * n) as f32);
}

fn test_recombination() {
    let len = 100;
    let parent1 = BinaryChromosome::zeroes(len);
    let parent2 = BinaryChromosome::ones(len);

    let max_n = 10;
    for n in 1..max_n+1 {
        let recombination = BinaryNPointBitCrossover::new(n);
        let child = recombination.recombine(&parent1, &parent2);

        println!("{}", child);
    }
}

fn test_init_population() {
    let ga_config = MyEvolutionConfig::new();
    let mut ga = GeneticAlgorithm::new(10, Box::new(ga_config));

    println!("Population:\n{}", ga);
    ga.evaluate();
    println!("Population:\n{}", ga);    
}

fn main() {
    test_creation();
    test_mutation();
    test_recombination();
    test_init_population();
}
