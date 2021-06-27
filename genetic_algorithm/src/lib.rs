use std::{clone, fmt, slice};
use rand::{self, Rng};

pub trait Chromosome : 'static + fmt::Display + clone::Clone {}

pub trait Mutation {
    type Chromosome;
    
    fn mutate(&self, target: &mut Self::Chromosome);
}

pub trait Recombination {
    type Chromosome;

    fn recombine(
        &self, parent1: &Self::Chromosome, parent1: &Self::Chromosome
    ) -> Self::Chromosome;
}

pub trait ChromosomeFactory<T: Chromosome> {
    fn create(&self) -> T;
}

pub trait EvolutionConfig<T: Chromosome>: ChromosomeFactory<T> {
    fn mutate(&self, target: &mut T);
    fn recombine(&self, parent1: &T, parent2: &T) -> T;
    fn evaluate(&self, subject: &T) -> f32;
}

pub struct Individual<T: Chromosome> {
    chromosome: Box<T>,
    fitness: Option<f32>,
}

impl<T: Chromosome> Individual<T> {
    pub fn new(chromosome: Box<T>) -> Self {
        Individual {
            chromosome,
            fitness: None
        }
    }
}

impl<T: Chromosome> fmt::Display for Individual<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.chromosome)?;
        if let Some(fitness) = self.fitness {
            write!(f, " fitness = {}", fitness)?;
        }

        Ok(())
    }
}

pub struct Population<T: Chromosome> {
    individuals: Vec<Individual<T>>,
}

impl<T: Chromosome> Population<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Population {
            individuals: Vec::with_capacity(capacity)
        }
    }
 
    pub fn populate(&mut self, size: usize, chromosome_factory: &(dyn EvolutionConfig<T>)) {
        while self.individuals.len() < size {
            self.individuals.push(
                Individual::new(Box::new(chromosome_factory.create()))
            );
        }
    }

    pub fn add(&mut self, individual: Individual<T>) {
        self.individuals.push(individual);
    }

    pub fn size(&self) -> usize {
        self.individuals.len()
    }

    pub fn iter(&self) -> slice::Iter<'_, Individual<T>> {
        self.individuals.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, Individual<T>> {
        self.individuals.iter_mut()
    }
}

impl<T: Chromosome> fmt::Display for Population<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut best: Option<f32> = None;
        let mut sum: f32 = 0f32;
        let mut num: usize = 0;

        for individual in self.individuals.iter() {
            write!(f, "{}\n", individual)?;

            if let Some(fitness) = individual.fitness {
                sum += fitness;
                num += 1;
                best = Some(
                    match best {
                        None => fitness,
                        Some(current_best) => current_best.max(fitness)
                    }
                )
            }
        }

        if let Some(best_fitness) = best {
            write!(f, "best = {}, avg. = {}", best_fitness, sum / (num as f32))?;
        }

        Ok(())
    }
}

pub trait Selector<T: Chromosome> {
    fn select(&self) -> &Individual<T>;
}

pub trait SelectionFactory<T: Chromosome> {
    fn select_from(&self, population: Population<T>) -> Box<dyn Selector<T>>;
}

pub struct GeneticAlgorithm<T: Chromosome> {
    pop_size: usize,
    recombination_prob: f32,
    mutation_prob: f32,
    selection: Box<dyn SelectionFactory<T>>,
    config: Box<dyn EvolutionConfig<T>>,
    population: Option<Population<T>>,
}

impl<T: Chromosome> GeneticAlgorithm<T> {
    pub fn new(
        pop_size: usize,
        config: Box<dyn EvolutionConfig<T>>,
        selection: Box<dyn SelectionFactory<T>>

    ) -> Self {
        GeneticAlgorithm {
            pop_size,
            config,
            recombination_prob: 0.8,
            mutation_prob: 0.8,
            selection,
            population: None,
        }
    }

    pub fn start(&mut self) {
        let mut population = Population::with_capacity(self.pop_size);
        population.populate(self.pop_size, &*(self.config));

        self.population = Some(population);
    }

    pub fn evaluate(&mut self) {
        if let Some(population) = &mut self.population {
            for indiv in population.iter_mut() {
                if let None = indiv.fitness {
                    (*indiv).fitness = Some(self.config.evaluate(&indiv.chromosome));
                }
            }
        }
    }

    pub fn breed(&mut self) {
        let old_population = self.population.take();
        let selector = (*self.selection).select_from(old_population.unwrap());
        let mut population = Population::with_capacity(self.pop_size);

        while population.size() < self.pop_size {
            let mut chromosome = Box::new(
                if rand::thread_rng().gen::<f32>() < self.recombination_prob {
                    let parent1 = selector.select();
                    let parent2 = selector.select();
                    self.config.recombine(&parent1.chromosome, &parent2.chromosome)
                } else {
                    let parent = selector.select();
                    (*parent.chromosome).clone()
                }
            );

            if rand::thread_rng().gen::<f32>() < self.mutation_prob {
                self.config.mutate(&mut chromosome)
            }

            population.add(Individual::new(chromosome))
        }

        self.population = Some(population);
    }
}

impl<T: Chromosome> fmt::Display for GeneticAlgorithm<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(population) = &self.population {
            write!(f, "Population:\n{}", population)?;
        }

        Ok(())
    }
}

pub mod selection;
pub mod binary;