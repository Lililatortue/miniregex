use crate::{cursor::CursorResult, DFA};

/// description:
/// Cursor that can only be used once
/// Once it reaches the final state it only returns that state
/// therefore if it is invalid it will always be invalid
/// if it matches it will always return match
/// 
pub struct DFACursor<'a> {
    dfa: &'a DFA<'a>,
    current: usize
}
impl<'a> DFACursor<'a>{
    
    pub(crate)fn init(dfa:&'a DFA)->Self {
        DFACursor {dfa: dfa, current: dfa.start()}
    }
    /// description:
    /// Checks if a char is valid given the current state of the cursor
    /// if state is valid 
    ///paramater:
    ///char
    ///
    ///return:
    ///CursorResult {Invalid, Valid, Match}
    ///
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
    pub fn soft_fullmatch(mut self,s:&str)->bool {
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
    ///returns true if a str is an instance of the dfa
    ///
    ///paramaters
    ///&str
    ///
    ///return:
    ///bool
    ///
    pub fn strong_fullmatch(mut self, s:&str)->bool{ 
        for c in s.chars() {
            let (row,_) = self.dfa.row(self.current);

            if let Some((_, next)) = row.iter()
                                        .find(|(rule, _)| rule.match_eq(c)) {
                self.current = *next;
            } else {
                return false;
            }
        }
        let (_,result) = self.dfa.row(self.current);
        *result
    }
    pub fn restart(self){

    }
}

pub struct DFARestartCursor {

}
impl DFARestartCursor {

}
