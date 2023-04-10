use crate::{
    field::{
        fields::montgomery_backed_prime_fields::{IsModulus, U256PrimeField},
        traits::IsTwoAdicField,
    },
    unsigned_integer::element::{UnsignedInteger, U256},
};

#[derive(Clone, Debug)]
pub struct MontgomeryConfigStark252PrimeField;
impl IsModulus<U256> for MontgomeryConfigStark252PrimeField {
    const MODULUS: U256 =
        U256::from("800000000000011000000000000000000000000000000000000000000000001");
}

pub type Stark252PrimeField = U256PrimeField<MontgomeryConfigStark252PrimeField>;

impl IsTwoAdicField for Stark252PrimeField {
    const TWO_ADICITY: u64 = 48;
    // Change this line for a new function like `from_limbs`.
    const TWO_ADIC_PRIMITVE_ROOT_OF_UNITY: U256 = UnsignedInteger {
        limbs: [
            219038664817244121,
            2879838607450979157,
            15244050560987562958,
            16338897044258952332,
        ],
    };
}

#[cfg(test)]
mod u256_two_adic_prime_field_tests {
    use proptest::{
        collection, prelude::any, prop_assert_eq, prop_compose, proptest, strategy::Strategy,
    };

    use super::Stark252PrimeField;
    use crate::{
        fft::helpers::log2,
        field::{
            element::FieldElement,
            traits::{IsTwoAdicField, RootsConfig},
        },
        polynomial::Polynomial,
    };

    type F = Stark252PrimeField;
    type FE = FieldElement<F>;

    prop_compose! {
        fn powers_of_two(max_exp: u8)(exp in 1..max_exp) -> usize { 1 << exp }
        // max_exp cannot be multiple of the bits that represent a usize, generally 64 or 32.
        // also it can't exceed the test field's two-adicity.
    }
    prop_compose! {
        fn field_element()(num in any::<u64>().prop_filter("Avoid null coefficients", |x| x != &0)) -> FE {
            FE::from(num)
        }
    }
    prop_compose! {
        fn field_vec(max_exp: u8)(vec in collection::vec(field_element(), 2..1<<max_exp).prop_filter("Avoid polynomials of size not power of two", |vec| vec.len().is_power_of_two())) -> Vec<FE> {
            vec
        }
    }
    prop_compose! {
        fn poly(max_exp: u8)(coeffs in field_vec(max_exp)) -> Polynomial<FE> {
            Polynomial::new(&coeffs)
        }
    }

    proptest! {
        // Property-based test that ensures FFT eval. in the FFT friendly field gives same result as a naive polynomial evaluation.
        #[test]
        fn test_fft_evaluation_is_correct_in_u256_fft_friendly_field(poly in poly(8)) {
            let order = log2(poly.coefficients().len()).unwrap();
            let twiddles = F::get_powers_of_primitive_root(order, poly.coefficients.len(), RootsConfig::Natural).unwrap();

            let fft_eval = poly.evaluate_fft().unwrap();
            let naive_eval = poly.evaluate_slice(&twiddles);

            prop_assert_eq!(fft_eval, naive_eval);
        }
    }
}