use crate::{cursor::CursorResult, DFA};


pub struct DFACursor<'a> {
    dfa: &'a DFA<'a>,
    current: usize
}
impl<'a> DFACursor<'a>{
    
    pub fn init(dfa:&'a DFA)->Self {
        DFACursor {dfa: dfa, current: dfa.start()}
    }

    pub fn match_eq(&mut self, c:char)->CursorResult {
        let (row,result) = self.dfa.row(self.current);
            if *result {
                return CursorResult::Match;
            }
            for (rule,next) in row {
                if rule.match_eq(c){
                    self.current = *next;
                    return CursorResult::Valid;
                }
            }
            return CursorResult::Invalid;
    }
    ///description:
    ///returns true if a str contains an instance
    ///
    ///paramater:
    ///&str
    ///
    ///return: 
    ///bool
    ///
    pub fn soft_fullmatch(&mut self,s:&str)->bool {
        for c in s.chars() {
            match self.match_eq(c) {
                CursorResult::Valid  =>continue,
                CursorResult::Invalid=>return false,
                CursorResult::Match  =>return true,
            }
        }
        return false;
    }

    ///description:
    ///returns true if a str is an instance
    ///
    ///paramaters
    ///&str
    ///
    ///return:
    ///bool
    ///
    pub fn strong_fullmatch(){

    }
}

pub struct DFARestartCursor {

}
impl DFARestartCursor {

}
