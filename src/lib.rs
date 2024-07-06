use clap::{Args, Parser, Subcommand};


#[derive(Parser)]
struct ParamsAligned {
command: String,
proving_system: String,
proof_file: String,
aligned_verification_data_path: String,
conn_endpoint: String
}

fn main() {
    let params = ParamsAligned::parse();
    println!(" getting parameters:  {:?}", params);
    println!("{:?}", aligned.stdout)

}
