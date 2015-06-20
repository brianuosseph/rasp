pub mod filter;

pub use filter::Biquad;
pub use filter::Lowpass;
pub use filter::Highpass;

pub use filter::OnePole;
pub use filter::OneZero;
pub use filter::TwoPole;
pub use filter::TwoZero;

/// A linear time-invariant system that
/// processes audio samples
///
/// This includes various audio filters
/// and delays
pub trait Filter {
  /// Processes sample and stores input and output to memory
  fn tick(&mut self, sample: f64) -> f64;

  /// Resets memory of all previous input and output to zero
  fn clear(&mut self);
}
