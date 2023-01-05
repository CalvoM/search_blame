use indicatif::{ProgressBar, ProgressStyle};

/// Provides interface for the progress UI when searching.
pub trait ProgressRenderer {
    /// start the rendering ui process
    fn start(&mut self);
    /// ends the rendering ui process
    fn end(&mut self, end_message: String);
}

/// The default and simple Progressbar that we use
pub struct DefaultTuiProgressBar {
    pub pb: ProgressBar,
}

impl ProgressRenderer for DefaultTuiProgressBar {
    fn start(&mut self) {
        self.pb.enable_steady_tick(100);
        self.pb
            .set_style(ProgressStyle::default_spinner().tick_strings(&[
                "▰▱▱▱▱▱▱",
                "▰▰▱▱▱▱▱",
                "▰▰▰▱▱▱▱",
                "▰▰▰▰▱▱▱",
                "▰▰▰▰▰▱▱",
                "▰▰▰▰▰▰▱",
                "▰▰▰▰▰▰▰",
            ]));
    }
    fn end(&mut self, end_message: String) {
        self.pb.finish_with_message(end_message);
    }
}
