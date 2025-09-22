use crate::graph::*;


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
    pub fn get_states(&self)-> &Vec<State> {
        &self.states
    }

    ///description:
    ///creates a cursor
    ///A cursor allows to use the graph and check if a string matches the FSA
    ///
    ///return:
    ///FSACursor
    ///
    pub fn cursor(&self)-> FSACursor<'_>{
        let mut v = Vec::new();
        v.push(&self.states[self.start]);
        FSACursor {graph:self, rules:v }
    }

    ///description:
    ///creates a restartable cursor
    ///A restartable cursor allows u to navigate a graph through a string and if state is invalid
    ///or match the cursor goes back to the beginning
    ///
    ///return:
    ///FSARestartCursor
    ///
    pub fn restart_cursor(&self) -> FSARestartCursor<'_>{
        FSARestartCursor (self.cursor())
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

//------------------Simulating NFA------------------------//

//reference a state and its literal compares and returns a result

pub enum CursorResult {
    Match,
    Valid,
    Invalid,
}
pub struct FSACursor<'a> {
    graph: &'a FSA,
    rules:Vec<&'a State> 
}

impl<'a> FSACursor<'a> {
    
    fn handle_split(&self,state:&'a State,c:char, list:&mut Vec<&'a State>){
        match state {
            State::Split(id1, id2)=> {      
                self.handle_split(&self.graph.states[*id1],c,list);
                self.handle_split(&self.graph.states[*id2],c,list);
            }
            State::Out(rule,id)=> { 
                if rule.match_eq(c) { 
                    list.push(&self.graph.states[*id]);
                }
            }
            _ => list.push(state), 
        }
    }

    ///description:
    ///moves the FSACursor and transformes it into FSARestartCursor
    ///
    ///return: 
    ///FSARestartCursor
    ///
    pub fn restartable(self)->FSARestartCursor<'a>{
        FSARestartCursor(self)
    }

    ///description:
    ///Method that compares a character, if the comparation is equal then cursor goes to the next
    ///state, if it finds a match or if it is invalid, it keeps the state as it is 
    ///
    ///parameters:
    ///c:char -> character to compares
    ///
    ///return: 
    ///CursorResult (Valid, Invalid, Match)
    ///
    pub fn match_eq(&mut self,c:char)->CursorResult {
        let mut list = Vec::new();
        for &state in self.rules.iter() {
            match state {
                State::Split(_,_)=>{ 
                    self.handle_split(state, c, &mut list);//create states 
                }
                State::Out(rule,id)=>{
                    if rule.match_eq(c) {
                        list.push(&self.graph.states[*id])
                    }; 
                }
                State::Match=> return CursorResult::Match, 
            }
        }
        
        if list.is_empty() {
            CursorResult::Invalid
        } else {
            self.rules = list;
            CursorResult::Valid
        }
    }

    
    pub fn match_full(mut self, s:&str)->bool{
        for c in s.chars() {
            let result = self.match_eq(c);
            match result {
                CursorResult::Invalid=>return false,
                CursorResult::Match  =>return true,
                CursorResult::Valid  =>continue,
            }
        }
        //WARNING: maybe needs a last check on self.rules
        false
    }
}


///description:
///Internally mutates the cursor when an Invalid Or Match states occurs 
///
pub struct FSARestartCursor<'a>(FSACursor<'a>);

impl<'a> FSARestartCursor<'a> { 
    ///description:
    ///Method that will restart cursor to the beggining of the graph if state is invalid or a match
    ///
    ///parameters: 
    ///c: char  -> the character that will be compared 
    ///
    ///return: 
    ///CursorResult (Valid, Invalid, Match)
    ///
    pub fn match_eq(&mut self, c:char)->CursorResult{
        let state = self.0.match_eq(c);
        match state {
            CursorResult::Invalid | CursorResult::Match => {
                self.restart();
            }
            _=>(),
        }
        state
    }

    ///description:
    ///Method that will match a full string if it finds an instance of 
    ///the word it will match it
    ///
    ///parameters: 
    ///s:&str     -> str to be compared
    ///
    ///returns: bool (invalid->false, match->true)
    ///
    pub fn match_full(&mut self, s:&str)->bool{
        for c in s.chars() {
            let state = self.match_eq(c);
            match state {
                CursorResult::Invalid => return false,
                CursorResult::Match   => return true,
                CursorResult::Valid   => continue,
            }
        }
        return false;
    }

    ///description:
    ///restart the cursor to the beggining of the graph
    ///
    ///returns: void
    ///
    pub fn restart(&mut self){
        self.0 = self.0.graph.cursor(); 
    }
}


//---------------------test---------------------------//
#[cfg(test)]
mod test {
    use super::*;
    
}



