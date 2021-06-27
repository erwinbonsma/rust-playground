use super::{Chromosome, Individual, Population, SelectionFactory, Selector};
use rand::{self, Rng};

#[derive(Clone, Copy)]
pub struct RankBasedSelection {
    group_size: usize
}

struct RankBasedSelector<T: Chromosome> {
    selection: RankBasedSelection,
    population: Population<T>
}

impl RankBasedSelection {
    pub fn new(group_size: usize) -> Self {
        RankBasedSelection {
            group_size
        }
    }
}

impl<T: Chromosome> SelectionFactory<T> for RankBasedSelection {
    fn select_from(&self, population: Population<T>) -> Box<dyn Selector<T>> {
        Box::new(
            RankBasedSelector {
                selection: self.clone(),
                population
            }
        )
    }
}

impl<T: Chromosome> RankBasedSelector<T> {
    fn select_one(&self) -> &Individual<T> {
        self.population.individuals.get(
            rand::thread_rng().gen_range(0..self.population.individuals.len())
        ).unwrap()
    }
}

impl<T: Chromosome> Selector<T> for RankBasedSelector<T> {
    fn select(&self) -> &Individual<T> {
        let mut best = self.select_one();

        for _ in 1..self.selection.group_size {
            let other = self.select_one();

            if other.fitness > best.fitness {
                best = other;
            }
        }

        best
    }
}
