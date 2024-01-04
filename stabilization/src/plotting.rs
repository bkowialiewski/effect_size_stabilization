use plotly::common::Title;
use plotly::{HeatMap, Plot, ImageFormat};
use plotly::common::{ColorScale, ColorScalePalette}; 
use plotly::layout::{Axis, Layout};

pub fn plot_heatmap(z: Vec<Vec<f64>>, title: &str, path: &str, f_name: &str, x_range: Vec<f64>, y_range: Vec<f64>) {

    // here define the palette color we want to use
    let custom_color_scale = ColorScale::Palette(ColorScalePalette::Viridis);

    // create the heatmap
    let trace = HeatMap::new_z(z)
        .x(x_range)
        .y(y_range)
        .color_scale(custom_color_scale);

    // create clot
    let mut plot = Plot::new();
    plot.add_trace(trace);

    // add a layout
    let layout = Layout::new()
        .title(Title::new(title))
        .x_axis(Axis::new().title(Title::new("Stopping criterion")))
        .y_axis(Axis::new().title(Title::new("Effect sizes")));

    // apply layout
    plot.set_layout(layout);

    let full_path = path.to_string() + f_name;

    // write plot
    plot.write_image(full_path, ImageFormat::SVG, 600, 600, 1.0);


}
