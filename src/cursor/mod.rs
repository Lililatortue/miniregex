mod lazydfa;

pub use lazydfa::LazyDfa;
 

#[derive(Debug,PartialEq)]
pub enum DfaResult {
    Invalid,
    Valid,
    Match, 
}

