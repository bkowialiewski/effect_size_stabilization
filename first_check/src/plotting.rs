use plotly::{ Plot, ImageFormat, Scatter};
use plotly::common::{Mode, Title};
use plotly::layout::{Axis, Layout};

pub fn scatter(simulation_results: Vec<Vec<f64>>, parameters: &super::Parameters) {

    let mut plot = Plot::new();

    for i in 0..parameters.n_sim {

        let samples: Vec<usize> = (parameters.n_min..parameters.n).collect();

        let trace = Scatter::new(samples, simulation_results[i][parameters.n_min..parameters.n].to_vec())
            .mode(Mode::Lines)
            .line(plotly::common::Line::new().width(1.0))
            .show_legend(false);
        plot.add_trace(trace);

    }

    let layout = Layout::new()
        // .title(Title::new("test"))
        .x_axis(Axis::new().title(Title::new("Sample size")).range(vec![0, parameters.n]))
        .y_axis(Axis::new().title(Title::new("Cohen's d")).range(vec![-1.0, 2.0]));
    plot.set_layout(layout);

    let full_path = "plots/first_check.svg";

    plot.write_image(full_path, ImageFormat::SVG, 600, 600, 1.0);

}
