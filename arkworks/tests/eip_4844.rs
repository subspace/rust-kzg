#[cfg(test)]
pub mod tests {
    #[cfg(not(feature = "minimal-spec"))]
    use kzg_bench::tests::eip_4844::compute_and_verify_kzg_proof_within_domain_test;
    use kzg_bench::tests::eip_4844::{
        blob_to_kzg_commitment_test, bytes_to_bls_field_test,
        compute_and_verify_blob_kzg_proof_fails_with_incorrect_proof_test,
        compute_and_verify_blob_kzg_proof_test,
        compute_and_verify_kzg_proof_fails_with_incorrect_proof_test,
        compute_and_verify_kzg_proof_round_trip_test, compute_kzg_proof_test, compute_powers_test,
        verify_kzg_proof_batch_fails_with_incorrect_proof_test, verify_kzg_proof_batch_test,
    };
    use rust_kzg_arkworks::eip_4844::{
        blob_to_kzg_commitment, blob_to_polynomial, bytes_to_blob, compute_blob_kzg_proof,
        compute_kzg_proof, compute_powers, evaluate_polynomial_in_evaluation_form,
        load_trusted_setup, verify_blob_kzg_proof, verify_blob_kzg_proof_batch, verify_kzg_proof,
    };
    use rust_kzg_arkworks::kzg_proofs::{FFTSettings, KZGSettings};
    use rust_kzg_arkworks::kzg_types::{ArkG1, ArkG2, FsFr};
    use rust_kzg_arkworks::utils::PolyData;

    #[test]
    pub fn bytes_to_bls_field_test_() {
        bytes_to_bls_field_test::<FsFr>();
    }

    #[test]
    pub fn compute_powers_test_() {
        compute_powers_test::<FsFr>(&compute_powers);
    }

    #[test]
    #[ignore]
    pub fn blob_to_kzg_commitment_test_() {
        blob_to_kzg_commitment_test::<FsFr, ArkG1, ArkG2, PolyData, FFTSettings, KZGSettings>(
            &load_trusted_setup,
            &blob_to_kzg_commitment,
        );
    }

    #[test]
    #[ignore]
    pub fn compute_kzg_proof_test_() {
        compute_kzg_proof_test::<FsFr, ArkG1, ArkG2, PolyData, FFTSettings, KZGSettings>(
            &load_trusted_setup,
            &compute_kzg_proof,
            &blob_to_polynomial,
            &evaluate_polynomial_in_evaluation_form,
        );
    }

    #[test]
    #[ignore]
    pub fn compute_and_verify_kzg_proof_round_trip_test_() {
        compute_and_verify_kzg_proof_round_trip_test::<
            FsFr,
            ArkG1,
            ArkG2,
            PolyData,
            FFTSettings,
            KZGSettings,
        >(
            &load_trusted_setup,
            &blob_to_kzg_commitment,
            &bytes_to_blob,
            &compute_kzg_proof,
            &blob_to_polynomial,
            &evaluate_polynomial_in_evaluation_form,
            &verify_kzg_proof,
        );
    }

    #[cfg(not(feature = "minimal-spec"))]
    #[test]
    #[ignore]
    pub fn compute_and_verify_kzg_proof_within_domain_test_() {
        compute_and_verify_kzg_proof_within_domain_test::<
            FsFr,
            ArkG1,
            ArkG2,
            PolyData,
            FFTSettings,
            KZGSettings,
        >(
            &load_trusted_setup,
            &blob_to_kzg_commitment,
            &bytes_to_blob,
            &compute_kzg_proof,
            &blob_to_polynomial,
            &evaluate_polynomial_in_evaluation_form,
            &verify_kzg_proof,
        );
    }

    #[test]
    #[ignore]
    pub fn compute_and_verify_kzg_proof_fails_with_incorrect_proof_test_() {
        compute_and_verify_kzg_proof_fails_with_incorrect_proof_test::<
            FsFr,
            ArkG1,
            ArkG2,
            PolyData,
            FFTSettings,
            KZGSettings,
        >(
            &load_trusted_setup,
            &blob_to_kzg_commitment,
            &bytes_to_blob,
            &compute_kzg_proof,
            &blob_to_polynomial,
            &evaluate_polynomial_in_evaluation_form,
            &verify_kzg_proof,
        );
    }

    #[test]
    #[ignore]
    pub fn compute_and_verify_blob_kzg_proof_test_() {
        compute_and_verify_blob_kzg_proof_test::<
            FsFr,
            ArkG1,
            ArkG2,
            PolyData,
            FFTSettings,
            KZGSettings,
        >(
            &load_trusted_setup,
            &blob_to_kzg_commitment,
            &bytes_to_blob,
            &compute_blob_kzg_proof,
            &verify_blob_kzg_proof,
        );
    }

    #[test]
    #[ignore]
    pub fn compute_and_verify_blob_kzg_proof_fails_with_incorrect_proof_test_() {
        compute_and_verify_blob_kzg_proof_fails_with_incorrect_proof_test::<
            FsFr,
            ArkG1,
            ArkG2,
            PolyData,
            FFTSettings,
            KZGSettings,
        >(
            &load_trusted_setup,
            &blob_to_kzg_commitment,
            &bytes_to_blob,
            &compute_blob_kzg_proof,
            &verify_blob_kzg_proof,
        );
    }

    #[test]
    #[ignore]
    pub fn verify_kzg_proof_batch_test_() {
        verify_kzg_proof_batch_test::<FsFr, ArkG1, ArkG2, PolyData, FFTSettings, KZGSettings>(
            &load_trusted_setup,
            &blob_to_kzg_commitment,
            &bytes_to_blob,
            &compute_blob_kzg_proof,
            &verify_blob_kzg_proof_batch,
        );
    }

    #[test]
    #[ignore]
    pub fn verify_kzg_proof_batch_fails_with_incorrect_proof_test_() {
        verify_kzg_proof_batch_fails_with_incorrect_proof_test::<
            FsFr,
            ArkG1,
            ArkG2,
            PolyData,
            FFTSettings,
            KZGSettings,
        >(
            &load_trusted_setup,
            &blob_to_kzg_commitment,
            &bytes_to_blob,
            &compute_blob_kzg_proof,
            &verify_blob_kzg_proof_batch,
        );
    }
}
