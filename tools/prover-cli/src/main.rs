use ark_bn254::{Bn254, Fr};
use ark_ff::UniformRand;
use ark_groth16::{prepare_verifying_key, generate_random_parameters, ProvingKey, VerifyingKey, Proof, create_random_proof};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable};
use ark_serialize::CanonicalSerialize;
use clap::Parser;
use rand::thread_rng;
use std::fs::File; use std::io::Write;

#[derive(Parser, Debug)]
struct Args{
    #[arg(long, default_value="vk.bin")] vk: String,
    #[arg(long, default_value="proof.bin")] proof: String,
    #[arg(long, default_value="inputs.bin")] inputs: String,
}

// Simple circuit: prove knowledge of x s.t. x + x = pub
struct DoubleCircuit{ pub x: Fr, pub public: Fr }
impl ConstraintSynthesizer<Fr> for DoubleCircuit{
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let x_var = cs.new_witness_variable(|| Ok(self.x))?;
        let two_x = cs.new_witness_variable(|| Ok(self.x + self.x))?;
        let pub_var = cs.new_input_variable(|| Ok(self.public))?;
        cs.enforce_constraint(ark_relations::r1cs::lc!() + x_var + x_var, ark_relations::r1cs::lc!() + Variable::One, ark_relations::r1cs::lc!() + two_x)?;
        cs.enforce_constraint(ark_relations::r1cs::lc!() + two_x, ark_relations::r1cs::lc!() + Variable::One, ark_relations::r1cs::lc!() + pub_var)?;
        Ok(())
    }
}

fn main(){
    let args = Args::parse();
    let mut rng = thread_rng();
    let x = Fr::rand(&mut rng);
    let public = x + x;
    let circuit = DoubleCircuit{ x, public };
    let params = generate_random_parameters::<Bn254, _, _>(circuit, &mut rng).expect("gen params");
    let vk: VerifyingKey<Bn254> = params.vk.clone();
    // Prove
    let proof = create_random_proof(DoubleCircuit{ x, public }, &params, &mut rng).expect("proof");
    // Inputs
    let mut inputs_bytes = Vec::new();
    let mut le = [0u8;32];
    public.serialize_compressed(&mut inputs_bytes).ok(); // not LE 32-bytes; we'll also push LE form
    // For pallet, we use 32-byte LE field encoding expected by shim
    inputs_bytes.clear();
    public.serialize_uncompressed(&mut inputs_bytes).ok(); // includes flags; not ideal
    inputs_bytes.clear();
    // Use canonical 32-byte little endian representation via ark_ff to_bytes_le
    inputs_bytes.extend_from_slice(&public.into_bigint().to_bytes_le());

    // Serialize vk and proof with ark-serialize compressed form
    let mut vk_bytes = Vec::new();
    vk.serialize_compressed(&mut vk_bytes).unwrap();
    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes).unwrap();

    // Write files
    let mut f = File::create(&args.vk).unwrap(); f.write_all(&vk_bytes).unwrap();
    let mut f2 = File::create(&args.proof).unwrap(); f2.write_all(&proof_bytes).unwrap();
    let mut f3 = File::create(&args.inputs).unwrap(); f3.write_all(&inputs_bytes).unwrap();

    println!("Wrote {} ({} bytes), {} ({} bytes), {} ({} bytes)", args.vk, vk_bytes.len(), args.proof, proof_bytes.len(), args.inputs, inputs_bytes.len());
}
