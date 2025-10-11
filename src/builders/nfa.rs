#![allow(dead_code)]
use std::collections::{BTreeMap, BTreeSet};

use crate::nfa::{NFA,State};
use crate::graph::Rule;



#[derive(Debug)]
pub(crate)struct Row<'a>(Vec<(&'a Rule, usize)>); 
///WARNING: for now doesnt work with +*? operande
///
///description
///navigates the graph in a branch segregated way
///
///
pub struct DfaBuilder {}

impl DfaBuilder {
    pub(crate)fn init()->Self {
        DfaBuilder{}
    }


    pub(crate)fn into_dfa<'a>(&mut self,nfa: &'a NFA)->(Vec<Row<'a>>,usize) {
        let mut table = vec![]; 
        let Some(start) = DfaBuilder::build_table(
                                        &mut table, 
                                        nfa,
                                        OrderedTasks::from_state(nfa, nfa.get_start()),
                                        &mut BTreeSet::new()
                                )
        else {panic!("error start")};

        (table,start)
    }



    fn build_table<'a>(
        table:   &mut Vec<Row<'a>>,
        nfa:     &'a NFA,
    mut tasks:   OrderedTasks<'a>,
        visited: &mut BTreeSet<Vec<usize>>
    ) -> Option<usize> { 

        let state = tasks.get_all_states();
        if visited.contains(&state) { 
            return Some(table.len()+1);
        }
        visited.insert(state);

        let mut row = Row (vec![]); 
        table.push(row);

        for (rule, indexs) in tasks.map.iter() {
                let new_task = OrderedTasks::from_index(nfa,&indexs);
                
                match DfaBuilder::build_table(table, nfa, new_task,visited) {
                    Some(next)=> table.last_mut()
                                      .unwrap().0
                                      .push((rule,next)),
                    None=>(),
                }
        }
        
        Some(table.len()-1) 
    }

}






enum TaskState<'a> {
    Match,
    Out(&'a Rule, &'a usize)
}
#[derive(Debug)]
pub struct OrderedTasks<'a>{
    matched: bool,
    map:BTreeMap<&'a Rule,Vec<usize>>
}

impl<'a> OrderedTasks<'a> {
    //-------------------- fonction -----------------------//
    pub(crate)fn init()->Self{
        OrderedTasks{matched: false, map: BTreeMap::new()}
    }

    pub(crate)fn make_true(&mut self){
        self.matched = true;
    }

    pub(crate)fn push(&mut self, rule:&'a Rule, id: &'a usize){
        self.map.entry(&rule).or_default().push(*id);
    }

    pub(crate)fn extend(&mut self, other:OrderedTasks<'a>) {
        if other.matched {
            self.matched = true;
        }
        for (rule, ids) in other.map.iter() {
            self.map.entry(rule)
                    .or_default()
                    .extend(ids)
        }
    }


    pub(crate)fn from_state(nfa:&'a NFA, state:&'a State)->OrderedTasks<'a> { 
        let task:OrderedTasks = match state {
            State::Match=> {
                 let mut t = OrderedTasks::init();
                     t.make_true();
                     t
            }
            State::Out(rule, id)=> {
                let mut t = OrderedTasks::init();
                    t.push(rule,id);
                    t
            }
            State::Split(left, right)=> {
                OrderedTasks::unwrap_split(nfa, *left, *right)
            }
        };
        task
    }

    pub(crate)fn from_index(nfa:&'a NFA, indexs:&Vec<usize>)->OrderedTasks<'a> {
        let mut task = OrderedTasks::init();
        for i in indexs {
            task.extend(OrderedTasks::from_state(nfa, &nfa.get_states()[*i]));
        }
        task
    } 
 

    fn unwrap_split(nfa: &'a NFA, id1:usize, id2:usize)->OrderedTasks<'a> {
        let mut task_list = OrderedTasks::init(); 
        let state0 = &nfa.get_states()[id1];
        let state1 = &nfa.get_states()[id2];  
        
        match state0 { 
            State::Match => task_list.make_true(),
            State::Out(rule,index) =>{
                task_list.push(&rule, &index);
            }
            State::Split(left, right) =>{
                task_list.extend(OrderedTasks::unwrap_split(nfa, *left, *right));
            }
        }

        match state1 { 
            State::Match => task_list.make_true(),
            State::Out(rule,index) =>{
                task_list.push(&rule, &index);
            }
            State::Split(left, right) =>{
                task_list.extend(OrderedTasks::unwrap_split(nfa, *left, *right));
            }
        }
        task_list
    } 

    fn get_all_states(&mut self)->Vec<usize> {
        let mut vec = vec![];
        for (_, indexs) in self.map.iter() {
            vec.extend(indexs); 
        }
        vec
    }
}




//--------------------Display implimentation-----------------------//

use std::fmt;
impl<'a> fmt::Display for Row<'a> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        write!(f, "[")?;
        for (i,(rule,target)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f,", ")?;
            }
            write!(f, "{} -> {}", rule,target)?;
        }
        write!(f,"]")
    }

}

mod test {
    use super::*;
    use crate::make_nfa;
    
    #[test]
    pub fn test_into_dfa(){
        let nfa = make_nfa!("a(bb*)c");
        //0[]-->for now this is match
        //1{a,b} [equal -> b -> 0, equal -> a -> 1]
        let mut builder = DfaBuilder::init();

        let (table,start)  = builder.into_dfa(&nfa);
        for (i,row) in table.iter().enumerate() {
            println!("{} {}",i,row)
        }
    }

}

