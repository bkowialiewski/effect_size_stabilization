mod stats;
mod simulations;
mod plotting;
mod utilities;

use stats::{mean, sd, cohen_d, p_value};
use simulations::launch_simulation;
use plotting::plot_heatmap;
use utilities::seq;

pub struct Parameters {

    n_sim: usize,
    n_min: usize,
    n_max: usize,

    alpha_value: f64,
    criterion_stop: usize,
    criterion_deviance: f64,
    effect_size: f64,

    effect_size_min: f64,
    effect_size_max: f64,
    effect_size_step: f64,

    criterion_min: f64,
    criterion_max: f64,
    criterion_step: f64,

    stabilization_type: i32,

}

pub struct Vectors {

    sample: Vec<f64>,
    all_ds: Vec<f64>,

    effect_sizes: Vec<f64>,
    sample_sizes: Vec<f64>,
    observed_power: Vec<f64>,

    effect_size_range: Vec<f64>,
    criterion_stop_range: Vec<f64>,
}

struct Outcomes {
    effect_sizes: Vec<Vec<f64>>,
    sample_sizes: Vec<Vec<f64>>,
    observed_power: Vec<Vec<f64>>,
    effect_size_variability: Vec<Vec<f64>>,
}

fn main() {
    
    // structs for parameters of the simulations
    let mut parameters = Parameters {

        n_sim: 10_000,
        n_min: 0,
        n_max: 10_000,

        alpha_value: 0.05,
        criterion_stop: 5,
        criterion_deviance: 0.05,
        effect_size: 0.0,

        effect_size_min: 0.0,
        effect_size_max: 1.0,
        effect_size_step: 0.01,

        criterion_min: 5.0,
        criterion_max: 100.0,
        criterion_step: 1.0,

        stabilization_type: 1,

    };

    // structs to pre-allocate memory for the vectors
    let mut vectors = Vectors {

        sample: vec![0.0; parameters.n_max],
        all_ds: vec![0.0; parameters.n_max],

        effect_sizes: vec![0.0; parameters.n_sim],
        sample_sizes: vec![0.0; parameters.n_sim],
        observed_power: vec![0.0; parameters.n_sim],

        effect_size_range: seq(parameters.effect_size_min, parameters.effect_size_max, parameters.effect_size_step),
        criterion_stop_range: seq(parameters.criterion_min, parameters.criterion_max, parameters.criterion_step),

    };

    // we get the result in "outcomes", which is a struct
    // call the core function of the simulation
    // this function launches for a certain configuration of parameters
    let outcomes = core(&mut parameters, &mut vectors);

    let path = if parameters.stabilization_type == 1 {
        "plots/absolute_stabilization/".to_string()
    } else {
        "plots/strict_stabilization/".to_string()
    };

    plot_heatmap(outcomes.effect_sizes, "Effect size", &path, "effect_size.svg", vectors.criterion_stop_range.to_vec(), vectors.effect_size_range.to_vec());
    plot_heatmap(outcomes.sample_sizes, "Sample size", &path, "sample_size.svg", vectors.criterion_stop_range.to_vec(), vectors.effect_size_range.to_vec());
    plot_heatmap(outcomes.observed_power, "Observed power", &path, "power.svg", vectors.criterion_stop_range.to_vec(), vectors.effect_size_range.to_vec());
    plot_heatmap(outcomes.effect_size_variability, "Effect size variability", &path, "variability.svg", vectors.criterion_stop_range.to_vec(), vectors.effect_size_range.to_vec());

}

fn core(parameters: &mut Parameters, vectors: &mut Vectors) -> Outcomes {

    // pre-allocate memory for the results of the simulation
    let mut outcomes = Outcomes {
        effect_sizes: vec![vec![0.0; vectors.criterion_stop_range.len()]; vectors.effect_size_range.len()],
        sample_sizes: vec![vec![0.0; vectors.criterion_stop_range.len()]; vectors.effect_size_range.len()],
        observed_power: vec![vec![0.0; vectors.criterion_stop_range.len()]; vectors.effect_size_range.len()],
        effect_size_variability: vec![vec![0.0; vectors.criterion_stop_range.len()]; vectors.effect_size_range.len()],
    };

    let total = (vectors.effect_size_range.len() * vectors.criterion_stop_range.len()) as f64;
    let mut cnt = 0.0;
    println!("Total number of simulations: {}", total);

    for i in 0..vectors.effect_size_range.len() {
        for j in 0..vectors.criterion_stop_range.len() {

            parameters.effect_size = vectors.effect_size_range[i];
            parameters.criterion_stop = vectors.criterion_stop_range[j] as usize;
            parameters.n_min = parameters.criterion_stop + 1;

            let (temp_effect_size, temp_sample_size, temp_observed_power, temp_effect_size_variability) = launch_simulation(parameters, vectors);

            outcomes.effect_sizes[i][j] = temp_effect_size;
            outcomes.sample_sizes[i][j] = temp_sample_size;
            outcomes.observed_power[i][j] = temp_observed_power;
            outcomes.effect_size_variability[i][j] = temp_effect_size_variability;
            
            cnt += 1.0;

            if cnt % 100.0 == 0.0 {
                println!("{cnt} / {total}");
            }

        }

    }

    outcomes

}


