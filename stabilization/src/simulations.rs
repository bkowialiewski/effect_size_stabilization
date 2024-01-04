use rand_distr::{Distribution, Normal};
use rand::thread_rng;
use crate::utilities::{strict_stabilization, absolute_stabilization};

pub fn launch_simulation(parameters: &mut super::Parameters, vectors: &mut super::Vectors) -> (f64, f64, f64, f64) {

    // iterate over number of simulations
    for epoch in 0..parameters.n_sim {

        let (effect_size, sample_size, p_value) = main_simulation(parameters, vectors);

        vectors.effect_sizes[epoch] = effect_size;
        vectors.sample_sizes[epoch] = sample_size as f64;
        vectors.observed_power[epoch] = ((p_value < parameters.alpha_value) as i32) as f64;

    }

    (super::mean(&vectors.effect_sizes), super::mean(&vectors.sample_sizes), super::mean(&vectors.observed_power), super::sd(&vectors.effect_sizes))

}

fn main_simulation(parameters: &mut super::Parameters, vectors: &mut super::Vectors) -> (f64, usize, f64) {

    // random number generator
    let mut rng = thread_rng();
    // generate random normal
    let normal = Normal::new(parameters.effect_size, 1.0).unwrap();
    // sample size reached in this particular simulation
    let mut reached_sample = parameters.n_max-1;

    // main loop of the simulation
    for i in 0..parameters.n_max {

        // add new value to the sample
        vectors.sample[i] = normal.sample(&mut rng);
        // compute cohen's d accordingly
        vectors.all_ds[i] = super::cohen_d(&vectors.sample[0..i]);

        if i < parameters.n_min {
            continue;
        }

        let criterion = if parameters.stabilization_type == 1 {
            absolute_stabilization(parameters.criterion_stop, parameters.criterion_deviance, &vectors.all_ds[0..i])
        } else {
            strict_stabilization(parameters.criterion_stop, parameters.criterion_deviance, &vectors.all_ds[0..i])
        };

        if criterion {
            reached_sample = i;
            break;
        }

    }

    (vectors.all_ds[reached_sample], reached_sample, super::p_value(&vectors.sample[0..reached_sample]))

}
