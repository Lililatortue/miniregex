use crate::graph::fsa::*;

///description 
///Good for small subset of a language can find matches and return an output associated 
///Fst is non-deterministic if you want a Dft call the function optimize
///
struct Fst {
    states: Vec<FSA>,        
    lexic: Vec<String>
}

impl Fst {
    pub fn default()->Self{
        Fst { states: vec![], lexic:vec![] }
    }

    pub fn add_lexic(&mut self,output: String, input: FSA  ) {
        self.states.push(input);
        self.lexic.push(output); 
    }

    pub fn get_fsa(&self)-> &[FSA]{
        &self.states       
    }
    pub fn get_lexic(&self)-> &[String] {
        &self.lexic  
    }

    pub fn into_dft(self)->Dft{
        Dft.init(self)
    }


    pub fn cursor(&self)-> FstCursor<'_> {
        let mut cursors = vec![];
        for (pos,state) in self.get_fsa().iter().enumerate() {  
            cursors.push((pos,state.restart_cursor()))
        } 
        FstCursor {cursors: cursors, fst: &self }
    }
}

pub struct FstCursor<'a> {
    cursors: Vec<(usize,FSARestartCursor<'a>)>,
    fst : &'a Fst
}

enum FstCursorResult {
    Valid,
    Invalid,
    Match
}

impl<'a> FstCursor<'a> {
    
    ///description
    ///finds matchs in a string and finds valid snippets and returns there output
    ///On match it resets the cursor
    ///
    ///after finishing trying to find inputs it resets the cursor internally
    ///
    ///parameters
    ///s -> characters to be match
    ///
    ///return 
    ///position of match and string
    ///
    ///exemple
    ///todo ( ) -> keyword & lpar & rpar 
    ///todo( )  -> keyword+lpar & rpar
    ///to(do )  ->  rpar
    ///
    fn soft_fullmatch(&mut self, s: &str)->Vec<&String>{
        let mut definition= vec![];
        
        for  (_,c) in s.char_indices(){// for now doesnt return position
            for cursor in self.cursors.iter_mut() {
                match cursor.1.match_eq(c) {
                    CursorResult::Valid | CursorResult::Invalid=> (),
                    CursorResult::Match   => {
                        definition.push(&self.fst.get_lexic()[cursor.0])
                    }
                }
            }
        };
        self.cursors.iter_mut().for_each(|c| c.1.restart());

        definition
    }
}


///description 
///determise the Fst so one state at the time
pub struct Dft {
    fst: Fst,
    action_table: Vec<Vec<usize>>,
}

impl Dft {
    pub fn init(fst:Fst)->Self {
        let queue = fst.states;
        //get fsa rules 
        for fsa in queue.iter(){
            //if one or zero state stop
            if queue.len()<=1 {break;}

            for fsa.
        }


    }
}
