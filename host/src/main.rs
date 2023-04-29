use methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::Prover;

fn main() {
    let a: u64 = 17;
    let b: u64 = 23;
    // Make the prover.
    let mut prover =
        Prover::new(MULTIPLY_ELF).expect("Prover should be constructed from valid ELF binary");

    prover.add_input_u32_slice(&to_vec(&a).expect("should be serializable"));
    prover.add_input_u32_slice(&to_vec(&b).expect("should be serializable"));

    // Run prover & generate receipt
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    // Code for transmitting or serializing the receipt for
    // other parties to verify here

    let c: u64 = from_slice(&receipt.journal).unwrap();

    // Print an assertion
    println!(
        "Hello, world! I know the factors of {}, and I can prove it!",
        c
    );

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(&MULTIPLY_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}

#[cfg(test)]
mod tests {
    use methods::{MULTIPLY_ELF, MULTIPLY_ID};
    use risc0_zkvm::{
        serde::{from_slice, to_vec},
        Prover,
    };

    const TEST_FACTOR_ONE: u64 = 17;
    const TEST_FACTOR_TWO: u64 = 23;

    #[test]
    fn run_factors() {
        let mut prover = Prover::new(MULTIPLY_ELF).expect(
            "Prover should be constructed from valid method source code and corresponding method ID",
        );

        prover.add_input_u32_slice(&to_vec(&TEST_FACTOR_ONE).expect("should be serializable"));
        prover.add_input_u32_slice(&to_vec(&TEST_FACTOR_TWO).expect("should be serializable"));

        let receipt = prover.run().expect("Should be able to prove valid code");
        receipt
            .verify(&MULTIPLY_ID)
            .expect("Proven code should verify");

        let result: u64 = from_slice(&receipt.journal).expect(
            "Journal output should deserialize into the same types (& order) that it was written",
        );
        assert_eq!(
            result,
            TEST_FACTOR_ONE * TEST_FACTOR_TWO,
            "We expect the zkVM output to be the product of the inputs"
        )
    }
}