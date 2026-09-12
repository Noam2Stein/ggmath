/// Defines an equality test used by assertions in generic functions.
///
/// Some functions are generic over `T`, but need to validate numeric
/// invariants. For example, when debug assertions are enabled,
/// [`Rotor::inverse`] needs to assert that `self` is normalized.
///
/// These functions cannot use ordinary equality. The equality test must be
/// approximate for floats, exact for integers, and disabled for SoA types,
/// where some lanes may intentionally be invalid when using lanewise selection.
///
/// This trait defines the [`eq_test`] operation, which has appropriate behavior
/// for each type, and is used by generic functions. Float implementations use
/// [`abs_diff_eq`] with `max_abs_diff = 2e-4`, integer implementations use
/// exact equality, and [`wide`] implementations always return `true`.
///
/// [`Rotor::inverse`]: crate::Rotor::inverse
/// [`eq_test`]: Self::eq_test
/// [`abs_diff_eq`]: FloatExt::abs_diff_eq
/// [`wide`]: crate#soa
pub trait EqTest {
    /// Returns whether `self` and `other` pass an equality test.
    ///
    /// See [`EqTest`] for more details.
    fn eq_test(&self, other: &Self) -> bool;
}
