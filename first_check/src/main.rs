mod stats;
mod plotting;

use rand_distr::{Distribution, Normal};
use rand::thread_rng;
use stats::cohen_d;
use plotting::scatter;

pub struct Parameters {

    n_sim: usize,
    n: usize,
    n_min: usize,
    effect_size: f64,

}

fn main() {

    let mut parameters = Parameters {

        n_sim: 500,
        n: 150,
        n_min: 4,
        effect_size: 0.5,

    };

    // results from calling the main function are stored in this variable
    let simulation_results = core(&mut parameters);

    // we plot all of that
    scatter(simulation_results, &parameters);

}

fn core(parameters: &mut Parameters) -> Vec<Vec<f64>> {

    // results are stored in a matrix, for convenience
    let mut simulation_results = vec![vec![0.0; parameters.n]; parameters.n_sim];

    for i in 0..parameters.n_sim {
        // given this set of parameters, launch a simulation
        let sample = generate_sample(parameters.n, parameters.effect_size);
        simulation_results[i] = compute_d(parameters, &sample);
    }

    simulation_results

}

fn compute_d(parameters: &mut Parameters, sample: &[f64]) -> Vec<f64> {

    let mut cohen_ds = vec![0.0; parameters.n];

    for i in parameters.n_min..parameters.n {
        cohen_ds[i] = cohen_d(&sample[0..i]);
    }

    cohen_ds

}

// this function merely generate a sample
// given a number of participants, and an effect size
fn generate_sample(n: usize, effect_size: f64) -> Vec<f64> {

    // we need these to generate random values from a normal distribution
    let mut rng = thread_rng();
    let normal = Normal::new(effect_size, 1.0).unwrap();

    // then just generate the sample
    (0..n).map(|_| normal.sample(&mut rng)).collect()

}
