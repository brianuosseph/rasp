/// An integrator used to average a signal.
/// 
/// A `LeakyIntegrator` is a specific type of `OnePole` filter, where the
/// input signal gain, `b0`, and the feedback gain, `a1`, are complements such
/// that `a1 = 1 - b0`, as long as `0 <= a1 < 1`. Because of this relationship
/// the filter equation can be changed to `y[n] = x[n] + a1 * (y[n-1] - x[n])`
/// and integrator only uses one gain `a1`, or `alpha`.
pub struct LeakyIntegrator {
  /// The feedback gain in the integrator (a1)
  alpha: f32,
  /// The integrator delayed sample memory
  y_z1: f32
}

impl LeakyIntegrator {
  /// Creates a new `LeakyIntegrator`.
  ///
  /// The integrator will be initalized in a state that does not alter the
  /// input signal, with `alpha` set to zero.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_mut)]
  /// use std::f32::EPSILON;
  /// use rasp::analysis::LeakyIntegrator;
  ///
  /// let mut integrator: LeakyIntegrator = LeakyIntegrator::new();
  /// assert!((integrator.get_alpha() - 0f32).abs() < EPSILON);
  /// ```
  pub fn new() -> Self {
    LeakyIntegrator {
      alpha: 0f32,
      y_z1: 0f32
    }
  }

  /// Returns the `alpha` gain of the integrator.
  ///
  /// The internal gain is called `alpha` because of the relationship between
  /// the input and feedback gains of the integrator where `a1 = 1 - b0`.
  pub fn get_alpha(&self) -> f32 {
    self.alpha
  }

  /// Sets the `alpha` gain of the integrator, where `0 <= alpha < 1`.
  ///
  /// If the new `gain` does not satisfy the constraint for `alpha`, then
  /// the current `alpha` remains unchanged.
  ///
  /// The internal gain is called `alpha` because of the relationship between
  /// the input and feedback gains of the integrator where `a1 = 1 - b0`. If
  /// this property is not held, then the integrator will fails.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::f32::EPSILON;
  /// use rasp::analysis::LeakyIntegrator;
  ///
  /// let mut integrator = LeakyIntegrator::new();
  /// integrator.set_alpha(0.99f32);
  /// assert!((integrator.get_alpha() - 0.99f32).abs() < EPSILON);
  ///
  /// // Alpha doesn't update if it's not a ratio less than 1
  /// integrator.set_alpha(1f32);
  /// assert!((integrator.get_alpha() - 0.99f32).abs() < EPSILON);
  /// integrator.set_alpha(-0.01f32);
  /// assert!((integrator.get_alpha() - 0.99f32).abs() < EPSILON);
  /// ```
  pub fn set_alpha(&mut self, gain: f32) {
    if gain >= 0f32 && gain < 1f32 {
      self.alpha = gain;
    }
  }

  /// Processes input `value` and outputs calculated sample.
  pub fn tick(&mut self, value: f32) -> f32 {
    self.y_z1 = value + self.alpha * (self.y_z1 - value);
    self.y_z1
  }

  /// Resets internal memory to zero.
  pub fn clear(&mut self) {
    self.y_z1 = 0f32;
  }

  /// Returns the last output of the integrator.
  pub fn last_out(&self) -> f32 {
    self.y_z1
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::*;

  #[test]
  fn new() {
    let integrator = LeakyIntegrator::new();

    assert!((integrator.last_out() - 0f32).abs() < EPSILON);
    assert!((integrator.get_alpha() - 0f32).abs() < EPSILON);
  }

  #[test]
  fn gain() {
    let mut integrator = LeakyIntegrator::new();

    integrator.set_alpha(0.5f32);
    assert!((integrator.get_alpha() - 0.5f32).abs() < EPSILON);
  }

  #[test]
  fn memory() {
    let mut integrator = LeakyIntegrator::new();
    assert!((integrator.last_out() - 0f32).abs() < EPSILON);

    integrator.set_alpha(0.5f32);
    let mut output = integrator.tick(1f32);

    assert!((output - 0.5f32).abs() < EPSILON);
    assert!((integrator.last_out() - 0.5f32).abs() < EPSILON);

    integrator.clear();
    assert!((integrator.last_out() - 0f32).abs() < EPSILON);

    output = integrator.tick(1f32);
    assert!((output - 0.5f32).abs() < EPSILON);
    assert!((integrator.last_out() - 0.5f32).abs() < EPSILON);
  }

  #[test]
  fn tick() {
    let mut integrator = LeakyIntegrator::new();
    let expected = vec![0.5f32, 0.75f32, 0.875f32, 0.9375f32, 0.96875f32];

    integrator.set_alpha(0.5f32);

    for case in expected.iter() {
      let output = integrator.tick(1f32);
      assert!((output - case).abs() < EPSILON);
    }
  }
}
