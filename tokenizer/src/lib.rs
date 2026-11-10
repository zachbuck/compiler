
//! This crate is the first step in the compilation pipeline.
//! This does initial formatting i.e., removing comments and unnecessary whitespace. 

#![warn(missing_docs)]

pub mod output;
pub mod tokenizer;

pub(crate) mod pattern_matching;
