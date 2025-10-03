use crate::graph::*;
use crate::cursor::{FSACursor,FSARestartCursor};

#[derive(Debug)]
pub enum State {
    Out(Rule, Id),
    Split(Id, Id),
    Match
}
//https://github.com/BurntSushi/rsc-regexp/blob/master/idiomatic-translation/nfa.rsc

///description:
///FSA -> Finite state automata,
///A graph that in which as rules to connect to next node
///
#[derive(Debug)]
pub struct FSA {
    start: Id,
    states: Vec<State>,  
}

impl FSA {
    ///description:
    ///Default initialisation of FSA
    ///
    ///return:
    ///FSA{start: 0 , states: empty}
    ///
    pub fn init()->FSA{
        FSA{start:0,states: vec![]}
    }
       
    ///description:
    ///Adds the the state to the states and returns its index
    ///
    ///return:
    ///Id ~usize~ -> index of the vec
    ///
    fn malloc(&mut self, state: State)->Id{
        let start = self.states.len();
        self.states.push(state);
        start
    }

    ///return:
    ///&vec-> the vec itself
    ///
    pub fn get_states(&self)-> &[State] {
        &self.states
    }
    pub fn get_start(&self)-> &State {
        &self.states[self.start]
    }
    ///description:
    ///creates a cursor
    ///A cursor allows to use the graph and check if a string matches the FSA
    ///
    ///return:
    ///FSACursor
    ///
    pub fn cursor(&self)-> FSACursor<'_>{
        FSACursor::init(self)
    }

    ///description:
    ///creates a restartable cursor
    ///A restartable cursor allows u to navigate a graph through a string and if state is invalid
    ///or match the cursor goes back to the beginning
    ///
    ///return:
    ///FSARestartCursor
    ///
    pub fn restart_cursor(&self) -> crate::nfa::FSARestartCursor<'_>{
        FSARestartCursor::init(self.cursor())
    }

    ///description
    ///transformes nfa into dfa
    ///
    ///parameters self
    ///
    pub(crate)fn determinize(self)->crate::dfa::DFA {
    
    }

    pub(crate) fn into_iter(self)->self {
        IntoIterNFA (self);
    }
}

pub(crate) struct IntoIterNFA(FSA);
impl Iterator for IntoIterNFA {
    type Item = Vec<State>;
    ///description
    ///Never returns invalid just returns states Out and States Match
    ///
    ///parameters
    ///
    ///
    ///return
    ///
    ///
    fn next(&mut self)->Option<Self::Item> {
        let list = vec![];


        if list.is_empty(){ 
            None
        } else {
            Some(list)
        }
    }
}



impl Graph for FSA {
    ///description:
    ///Adds a literal to states vector.
    ///A literal is a node that isnt connected to any other node 
    ///
    ///parameters:
    ///c:char -> the rule IMPORTANT: if . it means any look at README to see supported Char
    ///
    ///return: 
    ///Frag { adresse: (the index in states), goto:(None)}
    ///
    fn literal(&mut self,c: char)->Frag {
        let start = match c {
            '.'=> self.malloc(State::Out(Rule::Any,0)), 
            _  => self.malloc(State::Out(Rule::Equal(c),0)),
        };

        let out = vec![DanglingOuts::Out1(start)];

        Frag{adresse: start, goto: out}
    }

    ///description:
    ///Prepends e1 frag with e2 frag and create a Frag where the its adresse is the index of e1
    ///
    ///parameters:
    /// e1:Frag  ->  head of new frag
    /// e2: Frag ->  tail of new frag
    ///
    ///return: Frag {start: e1.start, out: e2.outs }
    ///
    fn concatenation(&mut self,e1: Frag, e2: Frag) -> Frag{
        self.patch(&e1.goto,e2.adresse);
        Frag{adresse:e1.adresse, goto:e2.goto}
    }

    ///description:
    ///Creates a conditional branch where either the Frag is repeated once or more times to be valid
    ///
    ///parameters:
    ///e1: Frag -> the pattern that will be impose
    ///
    ///return: 
    ///new Frag with the condition
    ///
    fn one_or_more(&mut self,e1:Frag)-> Frag {//+
        let split_adresse = self.malloc(State::Split(e1.adresse, 0));
                
        self.patch(&e1.goto, split_adresse);   
        let out = vec![DanglingOuts::Out2(split_adresse)];
        Frag{adresse:e1.adresse, goto: out}    
    }


    ///description:
    ///Creates a conditional branch where a Frag can be repeated present 
    ///
    ///parameters:
    ///e1:Frag -> the pattern that will be impose
    ///
    ///return:
    ///new Frag with the conditional branch
    ///
    fn zero_or_more(&mut self,e1:Frag)-> Frag {//*
        let start = self.malloc(State::Split(e1.adresse, 0));
        
        self.patch(&e1.goto, start);
        let out = vec![DanglingOuts::Out2(start)];
        Frag{adresse: start, goto:out}
    }

    ///description:
    ///
    ///
    ///parameters:
    ///
    ///return:
    ///
    fn one_or_zero(&mut self,mut e1: Frag)->Frag {//?
        let start = self.malloc(State::Split(e1.adresse, 0));
        
        e1.goto.push(DanglingOuts::Out2(start));
        Frag{adresse: start,goto:e1.goto}
    }
    
    ///description:
    ///
    ///parameters:
    ///
    ///return:
    ///
    fn alternation(&mut self,mut e1:Frag, e2:Frag)->Frag {

        let start = self.malloc(State::Split(e1.adresse, e2.adresse));

        e1.goto.extend(e2.goto);
        Frag{adresse: start, goto: e1.goto}
    }
    

    ///description:
    ///
    ///parameters:
    ///
    ///return:
    ///
    fn finish(mut self, e: Frag)->Self {
        let match_ = self.malloc(State::Match);
        self.patch(&e.goto, match_); 
        self.start = e.adresse;
        self
    }


    //https://github.com/BurntSushi/rsc-regexp/blob/master/idiomatic-translation/nfa.rsc
    //his approach i prefer it more its explicit

    ///description:
    ///
    ///parameters:
    ///
    ///return:
    /// 
    fn patch(&mut self, out:&[DanglingOuts], target: Id) {
        for out in out.iter() {
            match out {
                DanglingOuts::Out1(id)=> match self.states[*id] {
                    State::Out(_,ref mut id) => {
                       *id = target;
                    }
                    State::Split(ref mut id1,_) => {
                        *id1 = target;
                    }
                    _=>panic!("Cant be Match")
                }
                DanglingOuts::Out2(id)=> match self.states[*id] {
                    State::Split(_,ref mut id2)=>{
                        *id2 = target;
                    }
                    _=>panic!("out2 can only be acces by split")
                }
            }
        }
    }
}




//---------------------test---------------------------//
#[cfg(test)]
mod test {
    use super::*;
    
}



