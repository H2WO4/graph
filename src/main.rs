#![allow(unused)]

mod json;
mod types;

use plotly::color::NamedColor;
use plotly::common::{DashType, Line, Marker, Mode};
use plotly::layout::{Axis, AxisRange, AxisType};
use plotly::{Layout, Plot, Scatter};

use crate::json::DATA;

fn main() {
    // Initialize the plot
    let mut plot = Plot::new();
    plot.set_layout(
        Layout::new()
            .title("Results")
            .width(1280)
            .height(960)
            .x_axis(Axis::new().title("Size").type_(AxisType::Log))
            .y_axis(Axis::new().title("Time").type_(AxisType::Log)),
    );

    let colors = [
        NamedColor::Red,
        NamedColor::Blue,
        NamedColor::BlueViolet,
        NamedColor::Purple,
        NamedColor::Green,
        NamedColor::Yellow,
    ];

    // Draw the data
    for (i, record) in DATA.iter().enumerate() {
        let ys = record.times.iter().collect::<Vec<_>>();
        let xs = ys
            .iter()
            .enumerate()
            .map(|(idx, _)| 16 << idx)
            .collect::<Vec<u64>>();

        let color = colors[i % colors.len()];

        let trace = Scatter::new(xs, ys)
            .name(record.name.clone())
            .mode(Mode::LinesMarkers)
            .marker(Marker::new().size(10).color(color));

        plot.add_trace(trace);
    }

    plot.write_html("results.html");
}
