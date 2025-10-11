use std::collections::{BTreeMap};
use std::fmt::{self, Formatter};
use crate::{graph::Rule,NFA};
use crate::iterator::{NfaBfsIter,IterResult};


//plain old data
//used for this transformation
pub struct DFA<'a>{
    table:Vec<(Vec<(&'a Rule,usize)>,bool)>,
    start: usize,
}
impl<'a> DFA<'a> {
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
    pub(crate)fn init(nfa:&'a NFA)->Self{
        let Some(list) = NfaBfsIter::from_index(nfa,nfa.get_start_id()) else {
            panic!("Error: Cannot build dfa from empty nfa"); 
        };
        let mut bucket = Bucket::init();
        
        for result in list {
            match result {
                IterResult::Match=> bucket.matched = true,
                IterResult::Out(r, i)=> bucket.map
                                              .entry(r)
                                              .or_default()
                                              .push(*i),
                IterResult::Epsilon(i)=>bucket.epsillon = Some(i),
            }
        }

        let mut table = DFA{table:vec![],start: 0};
        table.start = build_table(&mut table, nfa, bucket, BTreeMap::new());
        table
    }

    pub fn start(&self)->usize {
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
    pub fn cursor(&self) {

    }
    pub fn restart_cursor(&self) {

    }
    

}


//-------------------------- algorithm -----------------------//




// Recursively builds row of the table
// While it travels through the NFA PostOrder traversal, it Builds the DFA in Inorder Traversal
//
// P.S
// For a more visual representation of the logic
// pls refer to the schemas: 
// subset_construction_implementation_flowchart.drawio
//
fn build_table<'a>(
    dfa:  &mut DFA<'a>,
    nfa:    &'a NFA,
    bucket: Bucket<'a>,
    mut state:  BTreeMap<Vec<usize>,usize>,
) -> usize{

    let size = dfa.table.len(); 
    // every leaf is a Match
    // so if bucket is empty we push an empty vec with the bool true
    if  bucket.map.is_empty() { 
        state.insert(bucket.state(),size);
        dfa.table.push((vec![],true));
        return size; 
    }

    // Preallocate row inside the table
    // and keep a pointer to it
    // so we can add the created row variable
    // once it is done being built
    let mut row = vec![];  
    dfa.table.push((vec![],bucket.matched)); 

    for (rule, indexs) in bucket.map {
        
        // Get the list of next nodes that are attached to the current node
        // using BFS style approach. 
        // Is agnostic on wether the its a match or an out 
        let Some(list) = NfaBfsIter::from_indexs(nfa,indexs) else {
            println!("continue {}",rule);
            continue
        };

        println!("list {:?}", &list);
        // Collapses same rules togheter through the use of a ordered map (BTreeMap)
        // It can not be IterResult agnostic
        // the bucket treats Matches as comparables too outs
        let mut new_bucket = Bucket::init();

        for result in list {
            match result {
                IterResult::Match=>{ 
                    new_bucket.matched = true;
                }
                IterResult::Out(r, i)=>new_bucket.map
                                                 .entry(r)
                                                 .or_default()
                                                 .push(*i),
                IterResult::Epsilon(i)=>new_bucket.epsillon=Some(i),
            }
        }
        
        let key = new_bucket.state();

        // Checks if node was already constructed previously:
        // - if it was (if):       it creates the rule and returns the index to that previously
        //                         constructed node
        // - if it wasn't (else):  1- we add the new_bucket key to the BTreeMap
        //                         2- travels recursively inside the new_bucket to create it
        if let Some(&i) = state.get(&key) {
            row.push((rule,i));
        } else {
            state.insert(key,size); 
            let i = build_table(dfa, nfa, new_bucket, state.clone());
            row.push((rule, i));      
        }
    };
    //adds data to row
    //creates this effect of 
    dfa.table[size].0 = row;
    size
}



struct Bucket<'a>{
    epsillon:Option<usize>,
    map:BTreeMap<&'a Rule,Vec<usize>>,
    matched:bool
}
impl<'a> Bucket<'a> {
    fn init()->Self{
        Bucket {epsillon:None, map:BTreeMap::new(), matched:false}
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
        let nfa = make_nfa!("a(bc)*");
        println!("-------------- NFA ---------------");
        println!("{}\n",nfa);      


        println!("-------------- DFA ---------------");
        let dfa = DFA::init(&nfa);
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

