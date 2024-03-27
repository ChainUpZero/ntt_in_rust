use bls12_381::{Scalar};
use std::ops::{Mul};
use ff::PrimeField;

/*
fn ntt(elements: &[u64]) -> Result<Vec<u64>, String> {
    let n = elements.len() as u64;
    let modulus = crate::small_ntt::find_modulus(elements)?;
    let omega = crate::small_ntt::find_primitive_root(n, modulus)?;
    Ok(crate::small_ntt::dft_matrix(n)
        .iter()
        .map(|row| {
            elements
                .iter()
                .zip(row)
                .map(|(elem, ij)| elem * omega.pow_mod(*ij, modulus))
                .sum::<u64>()
                % modulus
        })
        .collect::<Vec<u64>>())
}
 */

pub fn ntt_381(elements: &Vec<Scalar>) -> Vec<Scalar> {
    let n = elements.len() as u64;
    let mut matrix: Vec<Vec<Scalar>> = vec![vec![Scalar::from(0); n as usize]; n as usize];
    (0..n)
        .for_each(|x| (0..n)
            .for_each(
                |y| matrix[x as usize][y as usize] = Scalar::from(x)
                    .mul(&Scalar::from(y))));
    // println!("{:#?}", matrix);
    matrix.iter().map(|row| {
        elements.iter()
            .zip(row)
            .map(|(elem, ij)| elem * Scalar::ROOT_OF_UNITY_INV
                .pow(&<Scalar as Into<[u64;4]>>::into(*ij))).sum::<Scalar>()
    }).collect::<Vec<Scalar>>()
}

/*
pub fn i_ntt(elements: &[u64]) -> Result<Vec<u64>, String> {
    let n = elements.len() as u64;
    let modulus = find_modulus(elements)?;
    let omega = find_primitive_root(n, modulus)?;
    Ok(dft_matrix(n)
        .iter()
        .map(|row| {
            elements
                .iter()
                .zip(row)
                .map(|(elem, ij)| elem * omega.inv_pow_mod(*ij, modulus))
                .sum::<u64>()
                * n.inv_pow_mod(1, modulus)
                % modulus
        })
        .collect::<Vec<u64>>())
}
 */
pub fn i_ntt_381(elements: &Vec<Scalar>) -> Vec<Scalar> {
    let n = elements.len() as u64;
    let mut matrix: Vec<Vec<Scalar>> = vec![vec![Scalar::from(0); n as usize]; n as usize];
    (0..n)
        .for_each(|x| (0..n)
            .for_each(
                |y| matrix[x as usize][y as usize] = Scalar::from(x)
                    .mul(&Scalar::from(y))));
    matrix.iter().map(|row| {
        elements.iter()
            .zip(row)
            .map(|(elem, ij)| elem * Scalar::ROOT_OF_UNITY_INV
                .pow(&<Scalar as Into<[u64;4]>>::into(*ij))).sum::<Scalar>().invert().unwrap()
    }).collect::<Vec<Scalar>>()
}