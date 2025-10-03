//----------------------library functionnality-----------------------//
use crate::nfa::*;
use std::collections::HashMap;
use crate::graph::Rule;


type Position = (usize,usize);
enum RowRule {
    Valid(Rule),
    Invalid,
    Match, 
}
//task:
//  abid by those rules
//  - each row is unique
//  - row contains mutltiple unique rules
//  - each unique rules point to a row
pub struct DFA { 
    action_table: Vec<Vec<RowRule>>
}
impl DFA {
    ///description:
    ///Handles duplicate prefixes in nfa 
    ///
    ///parameters: 
    ///fsa-> FSA
    ///
    ///return: 
    ///TrieDFA
    ///
    pub fn determinize(fsa:FSA)->DFA {
        let action_table = vec![];
        
        let cursor = ValidCursor::init(&fsa);

        for states in cursor.next() {
            
        }

    }

    ///description
    ///
    ///
    ///parameters:
    ///
    ///
    ///return:
    ///
    ///
    pub fn minimize(self,fsa: FSA)->DFA {
        
    }
}
pub struct MinimalDFA {

}
impl MinimalDFA {

}



///description
///helper struct that goes through every states of an fsa.
///Bypasses rules and returns all possible states
///
///
struct ValidCursor<'a> {
    graph: FSA,
    states: Vec<State>
}


impl<'a> ValidCursor<'a>{

    fn init(fsa: FSA)-> Self { 
        ValidCursor {graph: fsa, states: vec![fsa.get_start()]} 
    }
    ///description
    ///Adresses splits
    fn handle_split(&self, state: &'a State, list: &mut Vec<&'a State>){
        match state {
            State::Split(id1, id2)=> {      
                self.handle_split(&self.graph.get_states()[*id1],list);
                self.handle_split(&self.graph.get_states()[*id2],list);
            }
            _ => list.push(state), 
        }
    }
    ///description
    ///isn't supposed to be used outside of dft
    ///skips any form of validation returns every possible state
    ///
    ///return 
    ///Option<Vec<state> -> if Vec is empty return none
    fn next(&mut self, id_list:&[usize])->Vec<&State> {
        let mut list: Vec<&State> = vec![];

        for id in  id_list{
            let state = &self.graph.get_states()[*id];
            match state {
                State::Split(_, _)=>{
                    self.handle_split(state,&mut list); 
                } 
                State::Out(_,id) =>list.push(state),
                _ => list.push(state)
            }
        }
        list
    }
}

