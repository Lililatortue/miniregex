use std::collections::{BTreeMap};
use std::fmt::{self, Formatter};
use crate::nfa::State;
use crate::utils;
use crate::{graph::Rule,Nfa};


/// Stands for: 
/// Deterministic Finite Automaton
///
/// description:
/// 
///
pub struct Dfa<'a>{
    table:Vec<(Vec<(&'a Rule,usize)>,bool)>,
    start: usize,
}
impl<'a> Dfa<'a> {
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
    pub(crate)fn init(nfa:&'a Nfa)->Self{
        let list = nfa.start_states();
        if list.is_empty() { panic!("invalid nfa it is empty") }

        let mut dfa = Dfa{table:vec![],start: 0};
        let mut bucket = Bucket::init();

        for result in list {
            match result {
                State::Match(_)=> bucket.matched = true,
                State::Out(r, i)=> bucket.map
                                         .entry(r)
                                         .or_default()
                                         .push(*i),
                _ => unreachable!("should be flatten split shouldn't exist")
            }
        }

        dfa.start = build_table(&mut dfa, nfa, bucket, BTreeMap::new());
        dfa
    }

    
    pub(crate)fn start(&self)->usize {
        self.start
    }

    pub(crate)fn row(&self,id:usize)->&(Vec<(&'a Rule,usize)>,bool){
        if self.table.len() < id {
            panic!("Out of bounds index");
        }
        &self.table[id]
    }

    pub fn minimalize(&self) {

    }
    //pub fn cursor(&self)->DFACursor<'_> {
    //    DFACursor::init(self)
    //}

    pub fn restart_cursor(&self) {

    }
    

}


//-------------------------- algorithm -----------------------//

// Recursively builds row of the table replicates subset construction algorithm
// While it travels through the NFA PostOrder traversal, it Builds the DFA in Inorder Traversal
//
//
// For a more visual representation of the logic
// pls refer to the schemas: 
// subset_construction_implementation_flowchart.drawio
//
fn build_table<'a>(
    dfa:  &mut Dfa<'a>,
    nfa:    &'a Nfa,
    bucket: Bucket<'a>,
    mut state:  BTreeMap<Vec<usize>,usize>,
) -> usize{

    let size = dfa.table.len(); 
    // Preallocate row inside the table
    // and keep a pointer to it
    // so we can add the created row variable
    // once it is done being built
    dfa.table.push((vec![],bucket.matched)); 
 
    let mut row = vec![];  

    for (rule, indexs) in bucket.map {
        // Get the list of next nodes that are attached to the current node
        // using BFS style approach. 
        // Is agnostic on wether the its a match or an out 
        let list = utils::states_from_index(nfa,&indexs);
        if list.is_empty() {continue;}
        // Collapses same rules togheter through the use of a ordered map (BTreeMap)
        // It can not be IterResult agnostic
        // the bucket treats Matches as comparables too outs
        let mut new_bucket = Bucket::init();

        for result in list.iter() {
            match result {
                State::Match(_)=>new_bucket.matched = true,
                State::Out(r, i)=>new_bucket.map
                                                 .entry(r)
                                                 .or_default()
                                                 .push(*i),
                _ => unreachable!("should be flatten out")

            }
        }        
        //let key  = new_bucket.state();
        // Checks if node was already constructed previously:
        // - if it was (if):       it creates the rule and returns the index to that previously
        //                         constructed node
        // - if it wasn't (else):  0- we add the new_bucket key to the BTreeMap
        //                         1- travels recursively inside the new_bucket to create it
        let key = new_bucket.state();

        let i = match &state.get(&key) {
            Some(i)=>**i,
            None => { 
                state.insert(key,dfa.table.len());//not size because its key to futur
                build_table(dfa, nfa, new_bucket, state.clone())
            } 
        };

        row.push((rule, i));
    }

    dfa.table[size].0 = row;
    size
}




//Plain old data that facilitates row manipulation
//
//
struct Bucket<'a>{
    map:BTreeMap<&'a Rule,Vec<usize>>,
    matched:bool
}
impl<'a> Bucket<'a> {
    fn init()->Self{
        Bucket {map:BTreeMap::new(), matched:false}
    }
    //return all the indexs concerned in the nfa
    //It used as a key to see if state as already been traversed before
    pub fn state(&self)->Vec<usize> {
        self.map.iter()
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
        let nfa = make_nfa!("a(bc)*d|def");
        println!("-------------- NFA ---------------");
        println!("{}\n",nfa);      


        println!("-------------- DFA ---------------");
        let dfa = Dfa::init(&nfa);
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
impl<'a> fmt::Display for Dfa<'a> {
    fn fmt(&self, f: &mut Formatter<'_>)->fmt::Result {
        writeln!(f,"start: {}, DFA [",self.start)?;

        for (i,(row,matched)) in self.table.iter().enumerate() {
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

