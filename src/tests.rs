use ark_bn254::Fr;

use crate::{
    snark::{prove, setup, verify},
    types::{ConstraintSystem, Input, LinearCombination},
    utils::{init_public_params, reset_profile},
};

#[test]
fn test_r1cs_example() {
    init_public_params();
    reset_profile();

    run_r1cs_example(300, 500);
}

fn run_r1cs_example(num_inputs: usize, num_constraints: usize) {
    use ark_std::UniformRand;
    use ark_std::Zero;
    let mut rng = rand::thread_rng();

    assert!(num_inputs <= num_constraints + 2);

    let primary_input_size = num_inputs;
    let auxiliary_input_size = 2 + num_constraints - num_inputs;
    let num_variables = primary_input_size + auxiliary_input_size;
    let mut cs = ConstraintSystem::new(primary_input_size, auxiliary_input_size);

    let mut a = Fr::rand(&mut rng);
    let mut b = Fr::rand(&mut rng);

    let mut assignments = vec![];
    assignments.push(a);
    assignments.push(b);

    for i in 0..(num_constraints - 1) {
        let la;
        let lb;
        let lc;
        let tmp;
        if i % 2 != 0 {
            la = LinearCombination::new(&[(i + 1, Fr::from(1))]);
            lb = LinearCombination::new(&[(i + 2, Fr::from(1))]);
            lc = LinearCombination::new(&[(i + 3, Fr::from(1))]);
            tmp = a * b;
        } else {
            la = LinearCombination::new(&[(i + 1, Fr::from(1)), (i + 2, Fr::from(1))]);
            lb = LinearCombination::new(&[(0, Fr::from(1))]);
            lc = LinearCombination::new(&[(i + 3, Fr::from(1))]);
            tmp = a + b;
        }
        assignments.push(tmp);
        a = b;
        b = tmp;

        cs.add_constraint(&la, &lb, &lc);
    }

    let mut fin = Fr::zero();
    let mut la = vec![];
    let mut lb = vec![];
    for i in 1..num_variables {
        la.push((i, Fr::from(1)));
        lb.push((i, Fr::from(1)));
        fin += assignments[i - 1];
    }
    let lc = LinearCombination::new(&[(num_variables, Fr::from(1))]);
    cs.add_constraint(
        &LinearCombination::new(&la),
        &LinearCombination::new(&lb),
        &lc,
    );
    assignments.push(fin * fin);

    assert_eq!(assignments.len(), num_variables);

    let primary_input = Input::from_fr(&assignments[..num_inputs]);
    let auxiliary_input = Input::from_fr(&assignments[num_inputs..]);

    assert!(cs.is_satisfied(&primary_input, &auxiliary_input));

    let key = setup(&cs);
    let proof = prove(&key, &primary_input, &auxiliary_input);
    let res = verify(&key, &primary_input, &proof);
    assert!(res);

    println!("Done");
}
