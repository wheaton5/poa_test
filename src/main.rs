#[macro_use]
extern crate clap;
extern crate bio;

use bio::alignment::pairwise::Scoring;
use bio::alignment::poa::*;
use clap::App;

use bio::io::fasta;

fn main() {
    println!("Hello, world!");
    let params = load_params();
    let scoring = Scoring::from_scores(-2, -2, 2, -4);
    let fasta_reader = fasta::Reader::from_file(&params.subreads_fasta).expect("could not open subreads fasta");
    let mut subreads: Vec<String> = Vec::new();
    for record in fasta_reader.records() {
        let record = record.expect("cant unwrap subread record");
        subreads.push(String::from_utf8(record.seq().to_vec()).expect("cant unwrap sting"));
    }
    let mut aligner = Aligner::new(scoring, &subreads[0].as_bytes());
    for i in 1..subreads.len() {
        aligner.global(subreads[i].as_bytes()).add_to_graph();
    }

}

#[derive(Clone)]
struct Params {
    subreads_fasta: String,
    output: String,
}

fn load_params() -> Params {
    let yaml = load_yaml!("params.yml");
    let params = App::from_yaml(yaml).get_matches();
    let fasta = params.value_of("subreads").expect("no subreads");
    let output = params.value_of("output").expect("no output");
    Params {
        subreads_fasta: fasta.to_string(),
        output: output.to_string(),
    }
}