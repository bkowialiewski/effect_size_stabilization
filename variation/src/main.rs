mod stats;
mod utilities;
mod plotting;

use rand_distr::{Distribution, Normal};
use rand::thread_rng;
use stats::{cohen_d, variance};
use utilities::seq;
use plotting::scatter;

pub struct Parameters {

    n_sim: usize,

    n: usize,
    n_min: usize,
    n_max: usize,
    n_step: usize,

    effect_size: f64,
    effect_size_min: f64,
    effect_size_max: f64,
    effect_size_step: f64,

}

fn main() {

    let mut parameters = Parameters {

        n_sim: 1_000_000,

        n: 0,
        n_min: 5,
        n_max: 100,
        n_step: 1,

        effect_size: 0.0,
        effect_size_min: 0.0,
        effect_size_max: 1.0,
        effect_size_step: 0.2,

    };

    // here we generate the ranges of parameters that we manipulate in our simulations
    // such as effect size and sample size
    let all_effect_sizes = seq(parameters.effect_size_min, parameters.effect_size_max, parameters.effect_size_step);
    let all_sample_sizes = seq(parameters.n_min as f64, parameters.n_max as f64, parameters.n_step as f64);
    // results from calling the main function are stored in this variable
    let simulation_results = core(&mut parameters, &all_effect_sizes, &all_sample_sizes);

    // we plot all of that
    scatter(all_effect_sizes, all_sample_sizes, simulation_results, &parameters);

}

fn core(parameters: &mut Parameters, all_effect_sizes: &[f64], all_sample_sizes: &[f64]) -> Vec<Vec<f64>> {

    // results are stored in a matrix, for convenience
    let mut simulation_results = vec![vec![0.0; all_sample_sizes.len()]; all_effect_sizes.len()];

    // iterate over all range of parameters, first on effect sizes
    for i in 0..all_effect_sizes.len() {

        // we need to update the parameters we manipulate accordingly
        parameters.effect_size = all_effect_sizes[i];

        // then on sample sizes
        for j in 0..all_sample_sizes.len() {

            parameters.n = all_sample_sizes[j] as usize;

            // given this set of parameters, launch a simulation
            simulation_results[i][j] = simulate(parameters);

        }
        // this allows us to track the state of the simulation - convenient
        println!("{}/{}", i+1, all_effect_sizes.len());
    }

    simulation_results

}

fn simulate(parameters: &mut Parameters) -> f64 {

    let all_ds: Vec<f64> = (0..parameters.n_sim)
        .map(
            |_| generate_sample(parameters.n, parameters.effect_size)
             )
        .collect();

    variance(&all_ds)

}

// this function merely generate a sample
// given a number of participants, and an effect size
fn generate_sample(n: usize, effect_size: f64) -> f64 {

    // we need these to generate random values from a normal distribution
    let mut rng = thread_rng();
    let normal = Normal::new(effect_size, 1.0).unwrap();

    // then just generate the sample
    let sample: Vec<f64> = (0..n).map(|_| normal.sample(&mut rng)).collect();

    cohen_d(&sample)

}
