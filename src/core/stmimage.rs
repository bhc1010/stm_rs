use crate::core::vector2::Vector2;

#[derive(Default, Debug, Clone)]
pub struct STMImage {
    lines: u32,
    size: f64,
    x_offset: f64,
    y_offset: f64,
    line_time: f64,
    bias: f64,
    // set_point: f64,
    spectroscopy: Option<Vec<STS>>,
}

impl STMImage {
    pub fn new(
        lines: u32,
        size: f64,
        x_offset: f64,
        y_offset: f64,
        line_time: f64,
        bias: f64,
        // set_point: f64,
        spectroscopy: Option<Vec<STS>>,
    ) -> Self {
        Self {
            lines,
            size,
            x_offset,
            y_offset,
            line_time,
            bias,
            // set_point,
            spectroscopy,
        }
    }
}

#[derive(Debug, Clone)]
pub struct STS {
    sts_type: STSType,
    start_voltage: f64,
    stop_voltage: f64,
    step_voltage: f64,
}

#[derive(Debug, Clone)]
enum STSType {
    Point(Vector2<f64>),
    Line(Vec<Vector2<f64>>),
}
