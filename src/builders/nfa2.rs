use std::collections::{BTreeMap};
use crate::{graph::Rule,NFA};
use crate::iterator::{NfaBfsIter,IterResult};



//plain old data
//used for this transformation
#[derive(Debug)]
enum Row<'a> {
    Match,
    Out(&'a Rule,usize),
}
type Table<'a> = Vec<Vec<Row<'a>>>;



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
pub fn into_dfa<'a>(nfa: &'a NFA)->Table<'a>{
    let Some(list) = NfaBfsIter::from_index(nfa,nfa.get_start_id()) else {
        panic!("Error: Cannot build dfa from empty nfa"); 
    };
    let mut bucket = Bucket(BTreeMap::new());
    
    for result in list {
        match result {
            IterResult::Match=>(),
            IterResult::Out(r, i)=> bucket.0
                                          .entry(r)
                                          .or_default()
                                          .push(*i),
        }
    }

    let mut table:Table = vec![];
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
    table:  &mut Table<'a>,
    nfa:    &'a NFA,
    bucket: Bucket<'a>,
    state:  &mut BTreeMap<Vec<usize>,usize>,
) -> Option<usize>{
    let size = table.len(); 
    // Checks
    //
    //
    if  bucket.0.is_empty() {
        return Some(size-1);
    }

    // Preallocate row inside the table
    // and keep a pointer to it
    // so we can add the created row variable
    // once it is done being built
    let mut row = vec![];  
    table.push(vec![]); 

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
        //
        //
        // P.S FOR NOW IT IS SOMETHING I NEED TO FIX
        // FIXME: { 
        //  Make function handle IterResult::Match the appropriate way
        //  it needs to behave the same as outs 
        //  right now treated as different
        //
        // }
        let mut new_bucket = Bucket(BTreeMap::new());
        for result in list {
            match result {
                IterResult::Match=>{ 
                 
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

            row.push(Row::Out(rule,i)); 
        } else {

            state.insert(key.clone(),size); 
            if let Some(i) =build_table(table, nfa, new_bucket, state){
                row.push(Row::Out(rule, i));     
            }
        }

    };
    //adds data to row
    //creates this effect of 
    table[size] = row;
    Some(size)
}



struct Bucket<'a>(BTreeMap<&'a Rule,Vec<usize>>);

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
    //please refer to the ones below
    #[test]
    pub fn test_dfa(){
        let nfa = make_nfa!("a(bc)*|def");
        println!("-------------- NFA ---------------");
        println!("{}\n\n",nfa);      


        println!("-------------- DFA ---------------");
        let table = into_dfa(&nfa);
        for row in table {
            println!("{:?}",row)
        }
    }

    //
    //
    #[test]
    pub fn test_zero_or_more(){
    }
    //
    //
    #[test]
    pub fn test_one_or_more(){
    }
    //
    //
    #[test]
    pub fn test_zero_or_one(){
    }
    //
    //
    #[test] 
    pub fn test_crazy_string(){

    }
}

