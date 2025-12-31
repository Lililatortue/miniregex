use crate::{graph::*, Dfa, LazyDfa};


//basic implementation of it was taken from there
//
//https://github.com/BurntSushi/rsc-regexp/blob/master/idiomatic-translation/nfa.rsc


#[derive(Debug)]
pub enum State {
    Out(Rule, Id),
    Split(Id, Id),
    Match(Option<usize>)// for indexing
}



///description:
///FSA -> Finite state automata,
///A graph that in which as rules to connect to next node
///
#[derive(Debug)]
pub struct Nfa {
    start: Id,
    states: Vec<State>,  
}

impl Nfa {
    ///description:
    ///Default initialisation of FSA
    ///
    ///return:
    ///FSA{start: 0 , states: empty}
    ///
    pub fn init()->Nfa{
        Nfa{start:0,states: vec![]}
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

    //description
    //indexs match to allows returning of a number on a match
    //
    //
    //
    //
    pub(crate)fn index_match(&mut self, index: usize) {
        let len = self.states.len();
        match self.states[len]{
            State::Match(ref mut i)=> *i=Some(index),
            _=> unreachable!("last element of vec should always be match before indexing"),
        };
    }
    pub(crate)fn states(&self)-> &[State] {
        &self.states
    }
    pub(crate)fn start(&self)->usize{
        self.start
    }

    pub(crate)fn get(&self, index: usize)-> Option<&State> {
        self.states.get(index)
    }

    pub(crate)fn is_match(&self, index: usize)-> bool {
        if let Some(State::Match(_)) = self.states.get(index){
            return true;
        }
        false
    }

    // description:
    // Get non-epsillon start
    //
    // return:
    // Vec<&state>
    //
    // example: if s0 -> s1 - A -> s2
    //                -> s3 - B -> s3
    //          then return is s1 & s2 states
    //          
    pub(crate) fn start_states(&self)-> Vec<&State> {
        let mut list = vec![];
        let state= &self.states[self.start];
        match state {
            State::Split(left, right)=>{
                list.extend(crate::utils::states_from_index(&self, &[*left,*right]))
            }, 
            _ => {
                list.push(state);
            }
        }
        list
    }

    // description:
    // Get non-epsillon start
    //
    // return:
    // Vec<&state>
    //
    // example: if s0 -> s1 - A -> s2
    //                -> s3 - B -> s3
    //          then return is s1 & s2 states
    //          
    pub(crate) fn start_index(&self)-> Vec<usize> {
        let mut list = vec![];
        let state= &self.states[self.start];
        match state {
            State::Match(_)=>(),

            State::Out(_,next) =>list.push(*next),

            State::Split(left, right)=>{
                list.extend(crate::utils::next_index(&self, &[*left,*right]).1)
            }, 
        }
        list
    }
    /// description:
    /// Creates a lazy cursor that slowly builds a dfa's as the traverses
    /// the a string
    ///
    /// return:
    /// LazyDfa
    ///
    pub fn cursor(&self)-> LazyDfa<'_>{
        LazyDfa::new(self)
    }

    pub fn determinize(&self)->Dfa<'_> {
        Dfa::init(self)
    }
}






use std::fmt;
impl fmt::Display for Nfa {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)-> fmt::Result {
        writeln!(f,"NFA:[")?;
        for (i,state) in self.states.iter().enumerate() {
            write!(f,"{}- ",i)?;
            match state{
                State::Out(r,i)=>   writeln!(f,"[ Out {} -> {} ]",r,i)?,
                State::Split(l,r)=> writeln!(f,"[ Split {}, {} ]",l,r)?,
                State::Match(option)=>  {
                    match option {
                        Some(i)=>writeln!(f,"[ Match->{} ]",i)?,
                        None   =>writeln!(f,"[ Match ]")?,
                    }                
                }
           };
        }
        writeln!(f,"]")
    }

}


impl Graph for Nfa {
    ///description:
    ///Adds a literal to states vector.
    ///A literal is a node that isnt connected to any other node 
    ///
    ///parameters:
    ///c:char -> the rule IMPORTANT: if . it means any look at README to see supported Characters
    ///
    ///return: 
    ///Frag { adresse: (the index in states), goto:(None)}
    ///
    fn literal(&mut self,c: char)->Frag {
        let start = match c {
            '.'=> self.malloc(State::Out(Rule::Any,0)), 
             _ => self.malloc(State::Out(Rule::Equal(c),0)),
        };
        

        let out = vec![DanglingOuts::Out1(start)];

        Frag{adresse: start, goto: out}
    }

    ///description:
    ///Prepends e1 frag with e2 frag and create a Frag where the its adresse is the index of e1
    ///
    ///parameters:
    /// e1:Frag  ->  head of new frag
    /// e2:Frag  ->  tail of new frag
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
        let match_ = self.malloc(State::Match(None));
        self.patch(&e.goto, match_); 
        self.start = e.adresse;
        self
    }



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

#[cfg(test)]
pub mod test{

}



