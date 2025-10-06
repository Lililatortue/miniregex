use crate::nfa::{NFA,State};
//reference a state and its literal compares and returns a result

pub enum CursorResult {
    Match,
    Valid,
    Invalid,
}
pub struct FSACursor<'a> {
    graph: &'a NFA,
    rules:Vec<&'a State> 
}

impl<'a> FSACursor<'a> {

    pub(crate)fn init(fsa:&'a NFA)->Self {
        FSACursor {graph:fsa, rules:vec![fsa.get_start()]}
    }


    fn handle_split(&self,state:&'a State,c:char, list:&mut Vec<&'a State>){
        match state {
            State::Split(id1, id2)=> {      
                self.handle_split(&self.graph.get_states()[*id1],c,list);
                self.handle_split(&self.graph.get_states()[*id2],c,list);
            }
            State::Out(rule,id)=> { 
                if rule.match_eq(c) { 
                    list.push(&self.graph.get_states()[*id]);
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
                        list.push(&self.graph.get_states()[*id])
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

    pub(crate)fn init(cursor: FSACursor<'a>)->Self {
        FSARestartCursor(cursor)
    }
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
    ///returns: 
    ///bool (invalid->false, match->true)
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


