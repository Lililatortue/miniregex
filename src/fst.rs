use crate::nfa::NFA;
use crate::cursor::FSARestartCursor;
///description 
///Good for small subset of a language can find matches and return an output associated 
///Fst is non-deterministic if you want a Dft call the function optimize
///
struct Fst {
    states: Vec<NFA>,        
    lexic: Vec<String>
}

impl Fst {
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


