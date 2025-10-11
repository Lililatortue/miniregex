use std::collections::{BTreeMap};
use std::fmt::{self, Formatter};
use crate::{graph::Rule,NFA};
use crate::iterator::{NfaBfsIter,IterResult};


//plain old data
//used for this transformation
struct DFA<'a>(Vec<(Vec<(&'a Rule,usize)>,bool)>);



//-------------------------- algorithm -----------------------//
/// description:
/// Creates a dfa from a reference to an nfa
/// 
/// parameter:
/// &nfa
/// 
/// return:
/// Table (Vec<Vec<&'a rules, usize >>) 
/// where lifetime a is the lifetime of nfa
///
fn into_dfa<'a>(nfa: &'a NFA)->DFA<'a>{
    let Some(list) = NfaBfsIter::from_index(nfa,nfa.get_start_id()) else {
        panic!("Error: Cannot build dfa from empty nfa"); 
    };
    let mut bucket = Bucket(BTreeMap::new(),false);
    
    for result in list {
        match result {
            IterResult::Match=> bucket.1 = true,
            IterResult::Out(r, i)=> bucket.0
                                          .entry(r)
                                          .or_default()
                                          .push(*i),
        }
    }

    let mut table = DFA(vec![]);
    build_table(&mut table, nfa, bucket, &mut BTreeMap::new());
    table
}

// Recursively builds row of the table
// While it travels through the NFA PostOrder traversal, it Builds the DFA in Inorder Traversal
//
// P.S
// For a more visual representation of the logic
// pls refer to the schemas: 
// subset_construction_implementation_flowchart.drawio
//
fn build_table<'a>(
    table:  &mut DFA<'a>,
    nfa:    &'a NFA,
    bucket: Bucket<'a>,
    state:  &mut BTreeMap<Vec<usize>,usize>,
) -> Option<usize>{

    let size = table.0.len(); 
    // every leaf is a Match
    // so if bucket is empty we push an empty vec with the bool true
    if  bucket.0.is_empty() {
        table.0.push((vec![],true));
        return Some(size);
    }

    // Preallocate row inside the table
    // and keep a pointer to it
    // so we can add the created row variable
    // once it is done being built
    let mut row = vec![];  
    table.0.push((vec![],bucket.1)); 

    for (rule, indexs) in bucket.0 {
        
        // Get the list of next nodes that are attached to the current node
        // using BFS style approach. 
        // Is agnostic on wether the its a match or an out 
        let Some(list) = NfaBfsIter::from_indexs(nfa,indexs) else {
            continue
        };

        // Collapses same rules togheter through the use of a ordered map (BTreeMap)
        // It can not be IterResult agnostic
        // the bucket treats Matches as comparables too outs
        let mut new_bucket = Bucket(BTreeMap::new(),false);
        for result in list {
            match result {
                IterResult::Match=>{ 
                    new_bucket.1 = true; 
                }
                IterResult::Out(r, i)=>new_bucket.0
                                                 .entry(r)
                                                 .or_default()
                                                 .push(*i)
            }
        }
        
        let key = new_bucket.state();

        // Checks if node was already constructed previously:
        // - if it was (if):       it creates the rule and returns the index to that previously
        //                         constructed node
        // - if it wasn't (else):  1- we add the new_bucket key to the BTreeMap
        //                         2- travels recursively inside the new_bucket to create it
        //
        if let Some(&i) = state.get(&key) {

            row.push((rule,i));
        } else {

            state.insert(key.clone(),size); 
            if let Some(i) =build_table(table, nfa, new_bucket, state){
                row.push((rule, i));     
            }
        }

    };
    //adds data to row
    //creates this effect of 
    table.0[size].0 = row;
    Some(size)
}



struct Bucket<'a>(BTreeMap<&'a Rule,Vec<usize>>,bool);
impl<'a> Bucket<'a> {
    //return all the indexs concerned in the nfa
    //It used as a key to see if state as already been traversed before
    pub fn state(&self)->Vec<usize> {
        self.0.iter()
              .flat_map(|(_,i)|i.iter().map(|i|*i))
              .collect::<Vec<_>>()
    }
}


// test if behavior is okay, open to any criticisme on improvement
// isn't to be used as benchmark more as if behavior is alright
//
//
mod test {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::make_nfa;

    //visual test not a real proof that it works
    //please refer to the ones below using cursors
    #[test]
    pub fn test_dfa(){
        let nfa = make_nfa!("a(bc)*|def");
        println!("-------------- NFA ---------------");
        println!("{}\n",nfa);      


        println!("-------------- DFA ---------------");
        let dfa = into_dfa(&nfa);
        println!("{}",dfa); 
    }
    #[test]
    pub fn test_zero_or_more(){

    }
    #[test]
    pub fn test_one_or_more(){
    }
    #[test]
    pub fn test_zero_or_one(){
    }
    #[test] 
    pub fn test_crazy_string(){

    }
}


//------------------- display implimentation ------------------------//
impl<'a> fmt::Display for DFA<'a> {
    fn fmt(&self, f: &mut Formatter<'_>)->fmt::Result {
        writeln!(f,"DFA [")?;

        for (i,(row,matched)) in self.0.iter().enumerate() {
            write!(f,"{}- [ ",i)?;
            
            let mut first = true;
            for (rule,index) in row.iter() {
                if !first { write!(f,", ")?; }
                first = false;

                write!(f,"{} -> {}",rule,index)?;
            }
            if *matched {
            if !row.is_empty(){
                write!(f,", ")?;
            }
                write!(f,"Match")?;
            }
            writeln!(f," ]")?;
        }
        writeln!(f,"]")
    }

}

