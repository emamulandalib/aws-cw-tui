use ratatui::style::Color;
use std::time::SystemTime;

/// Common type alias for metric chart data
pub type MetricTuple<'a> = (&'a str, String, &'a Vec<f64>, Color, f64, bool);

/// Chart configuration for consistent rendering
#[derive(Debug, Clone)]
pub struct ChartConfig {
    /// Chart title
    pub title: String,
    /// Chart color
    pub color: Color,
    /// Whether to show border
    pub show_border: bool,
    /// Whether chart is focused/selected
    pub is_focused: bool,
    /// Chart area padding
    pub padding: u16,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            color: Color::Blue,
            show_border: true,
            is_focused: false,
            padding: 1,
        }
    }
}

/// Chart data container for metric visualization
#[derive(Debug, Clone)]
pub struct ChartData {
    /// Metric name
    pub name: String,
    /// Current value
    pub current_value: f64,
    /// Historical data points
    pub history: Vec<f64>,
    /// Timestamps for each data point
    pub timestamps: Vec<SystemTime>,
    /// Display color
    pub color: Color,
    /// Formatted current value string
    pub formatted_value: String,
    /// Maximum value for scaling
    pub max_value: f64,
}

/// Grid layout configuration for multiple charts
#[derive(Debug, Clone)]
pub struct GridLayout {
    pub rows: usize,
    pub cols: usize,
}

impl GridLayout {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    pub fn total_cells(&self) -> usize {
        self.rows * self.cols
    }

    /// Calculate which cell a given index falls into
    pub fn cell_position(&self, index: usize) -> (usize, usize) {
        let row = index / self.cols;
        let col = index % self.cols;
        (row, col)
    }
}

/// Chart rendering mode
#[derive(Debug, Clone, PartialEq)]
pub enum ChartMode {
    /// Small chart in grid view
    Summary,
    /// Large detailed chart
    Detail,
    /// High resolution chart with full features
    HighResolution,
}

/// Chart render context containing all necessary information
#[derive(Debug)]
pub struct ChartContext<'a> {
    pub data: &'a ChartData,
    pub config: &'a ChartConfig,
    pub mode: ChartMode,
    pub layout: Option<&'a GridLayout>,
    pub selected_index: Option<usize>,
}

impl<'a> ChartContext<'a> {
    pub fn new(data: &'a ChartData, config: &'a ChartConfig, mode: ChartMode) -> Self {
        Self {
            data,
            config,
            mode,
            layout: None,
            selected_index: None,
        }
    }

    pub fn with_grid(mut self, layout: &'a GridLayout, selected_index: usize) -> Self {
        self.layout = Some(layout);
        self.selected_index = Some(selected_index);
        self
    }
}
