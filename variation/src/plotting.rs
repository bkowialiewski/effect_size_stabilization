use plotly::{ Plot, ImageFormat, Scatter};
use plotly::common::{Mode, Title};
use plotly::layout::{Axis, Layout};

pub fn scatter(all_effect_sizes: Vec<f64>, all_sample_sizes: Vec<f64>, simulation_results: Vec<Vec<f64>>, parameters: &super::Parameters) {

    let mut plot = Plot::new();

    for i in 0..all_effect_sizes.len() {

        let current_effect_size = (all_effect_sizes[i] * 10.0).round() / 10.0;

        let trace = Scatter::new(all_sample_sizes.to_vec(), simulation_results[i].to_vec())
            .mode(Mode::Lines)
            .name(current_effect_size.to_string());
        plot.add_trace(trace);

    }

    let layout = Layout::new()
        .title(Title::new("Effect size standard deviation"))
        .x_axis(Axis::new().title(Title::new("Sample size")).range(vec![0, parameters.n_max]))
        .y_axis(Axis::new().title(Title::new("Standard deviation")).range(vec![0.0, 1.0]));
    plot.set_layout(layout);

    let full_path = "plots/variation.svg";

    plot.write_image(full_path, ImageFormat::SVG, 600, 600, 1.0);

}
