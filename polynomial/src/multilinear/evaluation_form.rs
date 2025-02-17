use crate::multilinear::pairing_index::index_pair;
use ark_ff::{BigInteger, PrimeField};

#[derive(Clone, Debug, PartialEq)]
/// `MultilinearPolynomial` (Dense Evaluation Representation)
/// holds all evaluations over the boolean hypercube of an n_var multilinear polynomial
pub struct MultiLinearPolynomial<F: PrimeField> {
    n_vars: usize,
    evaluations: Vec<F>,
}

impl<F: PrimeField> MultiLinearPolynomial<F> {
    /// Instantiates a new `MultilinearPolynomial` after ensuring variable count
    /// aligns with evaluation len
    pub fn new(n_vars: usize, evaluations: Vec<F>) -> Result<Self, &'static str> {
        // the evaluation vec length must exactly be equal to 2^n_vars
        // this is because we might not always be able to assume the appropriate
        // element to pad the vector with.
        if evaluations.len() != (1 << n_vars) {
            return Err("evaluation vec len should equal 2^n_vars");
        }

        Ok(Self {
            n_vars,
            evaluations,
        })
    }

    /// Returns the number of variables
    pub fn n_vars(&self) -> usize {
        self.n_vars
    }

    /// Partially evaluate the `MultilinearPolynomial` at n consecutive variables
    /// e.g. f(a, b, c, d, e, f)
    /// we can pick a starting variable and supply n evaluation points
    /// f.partial_evaluate(1, [2, 3, 4])
    /// this partially evaluates 3 variables, starting at var b
    /// so b = 2, c = 3 and d = 4
    pub fn partial_evaluate(
        &self,
        initial_var: usize,
        assignments: &[F],
    ) -> Result<Self, &'static str> {
        // decided to go the consecutive partial evaluation route as opposed to the random access
        // evaluation route because consecutive partial eval is all that's needed for sumcheck and
        // gkr, and it seems random access partial evaluation will introduce additional cost (e.g. when
        // detecting duplicate assignments)
        let mut new_evaluations = self.evaluations.clone();

        // for each assignment
        // pull the evaluation pairs from the boolean hypercube
        // interpolate and evaluate the straight line given by each pair at the assignment
        for (i, assignment) in assignments.iter().enumerate() {
            let pairing_iterator = index_pair((self.n_vars - i) as u8, initial_var as u8);
            for (i, (left_pos, right_pos)) in pairing_iterator.enumerate() {
                let left = new_evaluations[left_pos];
                let right = new_evaluations[right_pos];

                new_evaluations[i] = match assignment {
                    a if a.is_zero() => left,
                    a if a.is_one() => right,
                    _ => {
                        // linear interpolation
                        // (1-r) * left + r * right
                        // left - r.left + r.right
                        // left - r (left - right)
                        left - *assignment * (left - right)
                    }
                };
            }
        }

        // truncate and return new polynomial
        let new_n_vars = self.n_vars - assignments.len();
        Ok(Self::new(
            new_n_vars,
            new_evaluations[..(1 << new_n_vars)].to_vec(),
        )?)
    }

    /// Evaluate the `MultilinearPolynomial` at n points
    pub fn evaluate(&self, assignments: &[F]) -> Result<F, &'static str> {
        if assignments.len() != self.n_vars {
            return Err("evaluate must assign to all variables");
        }

        Ok(self.partial_evaluate(0, assignments)?.evaluations[0])
    }

    /// Returns the evaluations of the `MultilinearPolynomial` as a slice
    pub fn evaluation_slice(&self) -> &[F] {
        &self.evaluations
    }

    /// Serialize the `MultilinearPolynomial`
    pub fn to_bytes(&self) -> Vec<u8> {
        self.evaluations
            .iter()
            .map(|elem| elem.into_bigint().to_bytes_be())
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }
}

#[cfg(test)]
mod tests {
    use crate::multilinear::evaluation_form::MultiLinearPolynomial;
    use ark_bls12_381::Fr;

    #[test]
    fn test_new_multilinear_poly() {
        // should not allow n_vars / evaluation count mismatch
        let poly = MultiLinearPolynomial::new(2, vec![Fr::from(3), Fr::from(1), Fr::from(2)]);
        assert_eq!(poly.is_err(), true);
        let poly = MultiLinearPolynomial::new(2, vec![Fr::from(3), Fr::from(1)]);
        assert_eq!(poly.is_err(), true);

        // correct inputs
        let poly = MultiLinearPolynomial::new(1, vec![Fr::from(3), Fr::from(1)]);
        assert_eq!(poly.is_err(), false);
        let poly =
            MultiLinearPolynomial::new(2, vec![Fr::from(3), Fr::from(1), Fr::from(2), Fr::from(5)]);
        assert_eq!(poly.is_err(), false);
    }

    #[test]
    fn test_partial_evaluate_single_variable() {
        let poly =
            MultiLinearPolynomial::new(2, vec![Fr::from(3), Fr::from(1), Fr::from(2), Fr::from(5)])
                .unwrap();
        assert_eq!(
            poly.partial_evaluate(0, &[Fr::from(5)])
                .unwrap()
                .evaluations,
            vec![Fr::from(-2), Fr::from(21)]
        );

        // get first half of evaluations by partially evaluating at 0
        assert_eq!(
            poly.partial_evaluate(0, &[Fr::from(0)])
                .unwrap()
                .evaluations,
            vec![Fr::from(3), Fr::from(1)]
        );
    }

    #[test]
    fn test_partial_evaluate_consecutive_variables() {
        // f(a, b, c) = 2ab + 3bc
        let poly = MultiLinearPolynomial::new(
            3,
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(3),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(5),
            ],
        )
        .unwrap();

        let f_of_a_evaluations = poly
            .partial_evaluate(1, &[Fr::from(2), Fr::from(3)])
            .unwrap()
            .evaluations;
        assert_eq!(f_of_a_evaluations.len(), 2);
        assert_eq!(f_of_a_evaluations, &[Fr::from(18), Fr::from(22)]);

        // TODO: add more tests, test out edge cases e.g. starting variable last var???
        // what are the edge cases
        // evaluation from the middle?
        // what part of the code might make this wrong??
        // TODO: use the other polynomial representation to generate the evaluations
    }

    #[test]
    fn test_full_evaluation() {
        // f(a, b, c) = 2ab + 3bc
        let poly = MultiLinearPolynomial::new(
            3,
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(3),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(5),
            ],
        )
        .unwrap();

        let evaluation_result = poly
            .evaluate(&[Fr::from(2), Fr::from(3), Fr::from(4)])
            .unwrap();
        assert_eq!(evaluation_result, Fr::from(48));
    }
}
