extern crate rna_transcription;
use rna_transcription as dna;

fn main() {


    println!("dna::DNA::new(\"GCTA\"), result is: {:?}", dna::DNA::new("GCTA"));
    println!("dna::DNA::new(\"GCTA\").into_rna(), result is: {:?}", dna::DNA::new("GCTA").unwrap().into_rna());

    println!("dna::DNA::new(\"X\").err(), expected:: Some(0), result:: {:?}", dna::DNA::new("X").err());
    println!("dna::DNA::new(\"GCTX\").err(), expected:: Some(3), result:: {:?}", dna::DNA::new("GCTX").err());

}