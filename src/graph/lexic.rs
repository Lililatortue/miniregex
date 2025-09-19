use std::collections::HashMap;

use crate::graph::{Id, Rule, FSA};







///Simple way to create a lexicon with fsa 
///Acts like an fst produces an output if A Match occurs
struct Lexicon {
    lexicon: HashMap<Id,String>,
    states: Vec<FSA>,
}


impl Lexicon {
    pub fn init()->Lexicon {
        Lexicon {lexicon: HashMap::new(), states: vec![]}
    }
       
    /// Adds a state to the graph
    /// and returns its index
    fn malloc(&mut self,lexic:String, state: FSA)->Id{
        let index = self.states.len();
        self.lexicon.insert(index,lexic);
        self.states.push(state);
        index 
    }

    pub fn get_states(&self)-> &Vec<FSA> {
        &self.states
    }
    
    pub fn cursor(&self){
        
    }
    pub fn restart_cursor(){

    }

}

///Once it is invalid (no valid state) or Match State
///it stays in said state
struct LexiconCursor{

}

///Restarted automaticly if it reaches an invalid and match state, it modifies its internal state
struct LexiconRestartCursor{

}








