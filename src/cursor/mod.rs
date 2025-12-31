mod lazydfa;
mod lazydft;


pub use lazydfa::LazyDfa;
pub use lazydft::LazyDft;
 

#[derive(Debug,PartialEq)]
pub enum DfaResult {
    Invalid,
    Valid,
    Match, 
}

