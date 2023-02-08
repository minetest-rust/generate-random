//! Generate random data.
//!
//! # Examples
//!
//! ```
//! use generate_random::GenerateRandom;
//!
//! #[derive(GenerateRandom)]
//! enum MyEnum {
//!     A,
//!     C(bool),
//!     B {
//!         x: u8,
//!     },
//!     // Providing a weight allows changing the probabilities.
//!     // This variant is now twice as likely to be generated as the others.
//!     #[weight(2)]
//!     D,
//! }
//!
//! let mut rng = rand::thread_rng();
//! let my_value = MyEnum::generate_random(&mut rng);
//! ```

/// This derive macro provides an implementation
/// of the [`trait@GenerateRandom`] trait.
///
/// Enum variants can be given a `weight` attribute
/// to change how often it is generated.
/// By default, the weight is `1`.
/// The probability of a variants is its weight
/// divided by the sum over all variants.
pub use generate_random_macro::GenerateRandom;

/// Enable randomly generating values of a type.
///
/// This trait can be implemented using the derive
/// macro of the same name: [`macro@GenerateRandom`].
pub trait GenerateRandom {
    /// Create a new random value of this type.
    fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self;
}

macro_rules! impl_generate_random {
	( $( $t:ty, )+ ) => {
		$(
			impl GenerateRandom for $t {
				fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
					rng.gen()
				}
			}
		)+
	}
}

impl_generate_random! {
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64,
}

impl<T: GenerateRandom> GenerateRandom for Option<T> {
    fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        if bool::generate_random(rng) {
            Some(T::generate_random(rng))
        } else {
            None
        }
    }
}

impl<T: GenerateRandom, const N: usize> GenerateRandom for [T; N] {
    fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        core::array::from_fn(|_| T::generate_random(rng))
    }
}

impl GenerateRandom for String {
    fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        use rand::distributions::{Alphanumeric, DistString};

        let len = rng.gen_range(0..32);
        Alphanumeric.sample_string(rng, len)
    }
}

impl<T: GenerateRandom> GenerateRandom for Vec<T> {
    fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let len = rng.gen_range(0..8);
        (0..len).map(|_| T::generate_random(rng)).collect()
    }
}

impl<T> GenerateRandom for std::collections::HashSet<T>
where
    T: GenerateRandom + std::cmp::Eq + std::hash::Hash,
{
    fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let len = rng.gen_range(0..8);
        (0..len).map(|_| T::generate_random(rng)).collect()
    }
}

impl<K, V> GenerateRandom for std::collections::HashMap<K, V>
where
    K: GenerateRandom + std::cmp::Eq + std::hash::Hash,
    V: GenerateRandom,
{
    fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let len = rng.gen_range(0..8);
        (0..len)
            .map(|_| (K::generate_random(rng), V::generate_random(rng)))
            .collect()
    }
}

impl<T: GenerateRandom> GenerateRandom for Box<T> {
    fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        Box::new(T::generate_random(rng))
    }
}

#[cfg(feature = "enumset")]
impl<T: enumset::EnumSetType + GenerateRandom> GenerateRandom for enumset::EnumSet<T> {
    fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let max = enumset::EnumSet::<T>::variant_count() * 2;
        let len = rng.gen_range(0..max);

        (0..len).map(|_| T::generate_random(rng)).collect()
    }
}

macro_rules! impl_generate_random_tuple {
	( $t0:ident $( $t:ident )* ) => {
		impl< $t0, $( $t, )* > GenerateRandom for ( $t0, $( $t, )* )
		where
			$t0: GenerateRandom,
			$(
				$t: GenerateRandom,
			)*
		{
			fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
				(
					$t0::generate_random(rng),
					$(
						$t::generate_random(rng),
					)*
				)
			}
		}
		impl_generate_random_tuple!( $( $t )* );
	};
	() => {
		impl GenerateRandom for () {
			fn generate_random<R: rand::Rng + ?Sized>(_rng: &mut R) -> Self {
				()
			}
		}
	}
}

impl_generate_random_tuple!(A B C D E F G H I J K L);

#[cfg(test)]
mod tests {
    use super::*;

    fn rng() -> impl rand::Rng {
        use rand::SeedableRng;
        rand_chacha::ChaCha8Rng::from(rand_chacha::ChaCha8Core::from_seed([37; 32]))
    }

    #[test]
    fn test_u8() {
        let mut rng = rng();
        assert_eq!(u8::generate_random(&mut rng), 55);
    }
}
