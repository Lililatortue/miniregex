use std::collections::HashMap;

use crate::graph::{Id, fsa::{FSA,FSACursor}};



///description
///Simple way to create a one row lexicon with fsa
///Is meant to be able to which discern FSA matched
///It is not meant to enforce semantics rules rather be a simple fst
/// * + should not be used (but can be if your a free spirit) it is meant for words
///
struct Lexicon {
    lexicon: Vec<String>,
    fsa: Vec<FSA>,
}

impl Lexicon {

    pub fn init()->Lexicon {
        Lexicon {lexicon: vec![], fsa: vec![]}
    }
       
    /// Adds a state to the graph
    /// and returns its index
    pub fn malloc(&mut self,lexic:String, state: FSA)->Id{
        let index = self.fsa.len();
        self.lexicon.push(lexic);
        self.fsa.push(state);
        index 
    }
    

    pub fn get_fsa(&self)-> &[FSA] {
        &self.fsa
    }
    
    pub fn cursor(&self)->LexiconCursor<'_> {
        let outputs = self.fsa.iter()
                              .enumerate()
                              .map(|(pos, item)| (pos, item.cursor()))
                              .collect();
        LexiconCursor {lexicon: self, outputs: outputs}
    }



    pub fn restart_cursor(&self)->LexiconRestartCursor<'_> { 
        LexiconRestartCursor(self.cursor())
    }

}


pub enum LexiconCursorResult<'a> {
    Valid,
    Invalid,
    Match(&'a str),
}
///Once it is invalid (no valid state) or Match State
///it stays in said state
pub struct LexiconCursor<'a>{
    lexicon: &'a Lexicon,
    outputs : Vec<(usize,FSACursor<'a>)>,//index of the states also key of hashmap
}

impl<'a> LexiconCursor<'a> {
    pub fn match_eq(&self, c:char)->LexiconCursorResult {
                
    }

}
///Restarted automaticly if it reaches an invalid and match state, it modifies its internal state
pub struct LexiconRestartCursor<'a>(LexiconCursor<'a>);








