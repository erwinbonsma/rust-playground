use evolutionary_alg::{ChromosomeFactory, EvolutionConfig, Mutation, Recombination, GeneticAlgorithm};
use evolutionary_alg::binary::{BinaryChromosome, BinaryBitMutation, BinaryNPointBitCrossover};
use evolutionary_alg::selection::RankBasedSelection;

struct MyEvolutionConfig {
    mutation: BinaryBitMutation,
    recombination: BinaryNPointBitCrossover,
}

impl MyEvolutionConfig {
    fn new() -> Self {
        MyEvolutionConfig {
            mutation: BinaryBitMutation::new(0.05),
            recombination: BinaryNPointBitCrossover::new(2)
        }
    }
}

impl ChromosomeFactory<BinaryChromosome> for MyEvolutionConfig {
    fn create(&self) -> BinaryChromosome {
        BinaryChromosome::new(32)
    }
}

impl EvolutionConfig<BinaryChromosome> for MyEvolutionConfig {
    fn mutate(&self, target: &mut BinaryChromosome) {
        self.mutation.mutate(target);
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
    let ga_config = MyEvolutionConfig::new();
    let mut ga = GeneticAlgorithm::new(
        10, Box::new(ga_config), Box::new(RankBasedSelection::new(2))
    );

    ga.start();
    println!("{}", ga);
    ga.evaluate();
    println!("{}", ga);    
}

fn test_selection() {
    let ga_config = MyEvolutionConfig::new();
    let mut ga = GeneticAlgorithm::new(
        10, Box::new(ga_config), Box::new(RankBasedSelection::new(2))
    );

    ga.start();
    ga.evaluate();

    for _ in 0..30 {
        ga.breed();
        ga.evaluate();
        println!("{}", ga);
    }
}

fn main() {
    test_creation();
    test_mutation();
    test_recombination();
    test_init_population();
    test_selection();
}
