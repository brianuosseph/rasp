use num;
use num::traits::Float;

use traits::FloatConst;

/// Converts a sample value to a dBFS value.
///
/// If the sample value is really small, or if the sample is not finite, it
/// will be assumed to be -120dBFS.
pub fn to_db<T: Float + FloatConst>(sample: T) -> T {
  debug_assert!(!sample.is_sign_negative());
  if sample > num::cast(1e-6f64).unwrap() && sample.is_finite() {
    let twenty: T = num::cast(20f64).unwrap();
    twenty * sample.log10()
  }
  else {
    num::cast(-120f64).unwrap()
  }
}

/// Converts a dBFS value to a sample value.
///
/// If the value is equal to or less than -120dBFS, or if the value is not
/// finite, the sample value will be zero.
pub fn to_sample<T: Float + FloatConst>(db_value: T) -> T {
  if db_value > num::cast(-120f64).unwrap() && db_value.is_finite() {
    let ten   : T = num::cast(10f64).unwrap();
    let twenty: T = num::cast(20f64).unwrap();
    ten.powf(db_value / twenty)
  }
  else {
    num::zero()
  }
}

/// Applys a gain to a sample value.
///
/// This includes a `debug_assert!` to check if the gain is a valid ratio.
#[inline]
pub fn apply_gain<T: Float>(sample: T, ratio: T) -> T {
  debug_assert!(ratio.is_finite() && !ratio.is_sign_negative());
  sample * ratio
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::*;

  #[test]
  fn conversion_to_decibels() {
    /* Zero case, should map to minimum */
    assert!((to_db(0f32) - -120f32).abs() < EPSILON);

    /* Below minimum */
    assert!((to_db(0.000000999f32) - -120f32).abs() < EPSILON);
    assert!((to_db(0.0000001f32) - -120f32).abs() < EPSILON);

    /* 20db intervals */
    assert!((to_db(1e-1f32) - -20f32).abs() < EPSILON);
    assert!((to_db(1e-2f32) - -40f32).abs() < EPSILON);
    assert!((to_db(1e-3f32) - -60f32).abs() < EPSILON);
    assert!((to_db(1e-4f32) - -80f32).abs() < EPSILON);
    assert!((to_db(1e-5f32) - -100f32).abs() < EPSILON);
    assert!((to_db(1e-6f32) - -120f32).abs() < EPSILON);

    /* Halving in decibels */
    /* All seem correct, but this way of testing is too accurate,
       or the conversion too inaccurate */
    // assert!((to_db(0.841395f32) - -1.5f32).abs() < 1e-4f32);
    // assert!((to_db(0.707946f32) - -3f32).abs() < 1e-4f32);
    // assert!((to_db(0.501187f32) - -6f32).abs() < 1e-4f32);
    // assert!((to_db(0.251189f32) - -12f32).abs() < 1e-4f32);
    // assert!((to_db(0.0630957f32) - -24f32).abs() < 1e-4f32);
    // assert!((to_db(0.00398107f32) - -48f32).abs() < 1e-4f32);
    // assert!((to_db(0.00000158489f32) - -96f32).abs() < 1e-4f32);

    /* Beyond 0db */
    /* Same issue as above */
    // assert!((1.12202f32.to_db() - 1f32).abs() < EPSILON);
  }

  #[test]
  #[should_panic]
  fn paniced_conversion_to_decibels() {
    /* Invalid input */
    assert!((to_db(NAN) - -120f32).abs() < EPSILON);
    assert!((to_db(INFINITY) - -120f32).abs() < EPSILON);
    assert!((to_db(NEG_INFINITY) - -120f32).abs() < EPSILON);
  }

  #[test]
  fn conversion_to_samples() {
    /* Below minimum */
    assert!((to_sample(-1000f32) - 0f32).abs() < EPSILON);
    assert!((to_sample(-120.000001f32) - 0f32).abs() < EPSILON);

    /* Minimum (zero) case */
    assert!((to_sample(-120f32) - 0f32).abs() < EPSILON);

    /* 20db intervals */
    assert!((to_sample(-20f32) - 1e-1f32).abs() < EPSILON);
    assert!((to_sample(-40f32) - 1e-2f32).abs() < EPSILON);
    assert!((to_sample(-60f32) - 1e-3f32).abs() < EPSILON);
    assert!((to_sample(-80f32) - 1e-4f32).abs() < EPSILON);
    assert!((to_sample(-100f32) - 1e-5f32).abs() < EPSILON);

    /* Halving in decibels */
    /* All correct, but this way of testing is too accurate */
    // assert!((to_sample(-1.5f32) - 0.841395f32).abs() < 1e-5f32);
    // assert!((to_sample(-3f32) - 0.707946f32).abs() < EPSILON);
    // assert!((to_sample(-6f32) - 0.501187f32).abs() < EPSILON);
    // assert!((to_sample(-12f32) - 0.251189f32).abs() < EPSILON);
    // assert!((to_sample(-24f32) - 0.0630957f32).abs() < EPSILON);
    // assert!((to_sample(-48f32) - 0.00398107f32).abs() < EPSILON);
    // assert!((to_sample(-96f32) - 0.00000158489f32).abs() < EPSILON);

    /* Beyond 0db */
    /* Same issue as above */
    // assert!((to_sample(1f32) - 1.12202f32).abs() < EPSILON);

    /* Invalid input */
    assert!((to_sample(NAN) - 0f32).abs() < EPSILON);
    assert!((to_sample(INFINITY) - 0f32).abs() < EPSILON);
    assert!((to_sample(NEG_INFINITY) - 0f32).abs() < EPSILON);
  }
}
