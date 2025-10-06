//----------------------library functionnality-----------------------//
use crate::nfa::*;
use std::collections::HashMap;
use crate::graph::Rule;


enum DFARule<'a> {
    Valid(&'a Rule,usize),
    Match, 
}
//task:
//  abid by those rules
//  - each row is unique
//  - row contains mutltiple unique rules
//  - each unique rules point to a row
pub struct DFA<'a> {
    start: usize,
    action_table: Vec<Vec<DFARule<'a>>>
}

