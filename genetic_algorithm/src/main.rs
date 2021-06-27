use evolutionary_alg::{
    Genotype, Phenotype, GenotypeFactory, GenotypeManipulation, GenotypeConfig, 
    Mutation, Recombination, GeneticAlgorithm
};
use evolutionary_alg::binary::{BinaryChromosome, BinaryBitMutation, BinaryNPointBitCrossover};
use evolutionary_alg::selection::RankBasedSelection;
use bit_vec::BitVec;
use std::{fmt};

struct MaxOnesPhenotype {
    bits: BitVec,
}

impl Phenotype for MaxOnesPhenotype {
    fn evaluate(&self) -> f32 {
        // Count the number of ones
        self.bits.iter().filter(|x| *x).count() as f32 / self.bits.len() as f32
    }
}

impl fmt::Display for MaxOnesPhenotype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Avoid duplication with BinaryChromosome
        for bit in self.bits.iter() {
            let char = if bit { '1' } else { '0' };
            write!(f, "{}", char)?;
        }

        Ok(())
    }
}

impl Genotype<MaxOnesPhenotype> for BinaryChromosome {
    fn express(&self) -> MaxOnesPhenotype {
        MaxOnesPhenotype {
            bits: self.bits.clone()
        }
    }
} 

struct MaxOnesConfig {
    mutation: BinaryBitMutation,
    recombination: BinaryNPointBitCrossover,
}

impl MaxOnesConfig {
    fn new() -> Self {
        MaxOnesConfig {
            mutation: BinaryBitMutation::new(0.02),
            recombination: BinaryNPointBitCrossover::new(2)
        }
    }
}

impl GenotypeFactory<MaxOnesPhenotype, BinaryChromosome> for MaxOnesConfig {
    fn create(&self) -> BinaryChromosome {
        BinaryChromosome::new(32)
    }
}

impl GenotypeManipulation<MaxOnesPhenotype, BinaryChromosome> for MaxOnesConfig {
    fn mutate(&self, target: &mut BinaryChromosome) {
        self.mutation.mutate(target);
    }

    fn recombine(&self, parent1: &BinaryChromosome, parent2: &BinaryChromosome) -> BinaryChromosome {
        self.recombination.recombine(parent1, parent2)
    }
}

impl GenotypeConfig<MaxOnesPhenotype, BinaryChromosome> for MaxOnesConfig {}

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
        let mut mutated = chromosome.clone();
        mutation.mutate(&mut mutated);
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
    let ga_config = MaxOnesConfig::new();
    let mut ga = GeneticAlgorithm::new(
        10, Box::new(ga_config), Box::new(RankBasedSelection::new(2))
    );

    ga.start();
    println!("{}", ga);
    ga.evaluate();
    println!("{}", ga);    
}

fn test_selection() {
    let ga_config = MaxOnesConfig::new();
    let mut ga = GeneticAlgorithm::new(
        20, Box::new(ga_config), Box::new(RankBasedSelection::new(2))
    );

    ga.start();

    for _ in 0..100 {
        ga.grow();
        ga.evaluate();
        if let Some(stats) = ga.get_stats() {
            println!("{:?}", stats);
        }

        ga.breed();
    }
}

fn main() {
    test_creation();
    test_mutation();
    test_recombination();
    test_init_population();
    test_selection();
}
