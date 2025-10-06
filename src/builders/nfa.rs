#![allow(dead_code)]
use std::collections::{HashMap};

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
///
///
pub struct DfaBuilder<'a> {
    nfa  : &'a NFA, 
    tasks: Vec<HashedTask<'a>>,
}
pub struct Test<'a>(Vec<Row<'a>>,usize);

impl<'a> DfaBuilder<'a> {
    pub(crate)fn init(nfa: &'a NFA)->Self {
        let tasks = vec![Task::from_state(nfa, nfa.get_start()).hash()];
        DfaBuilder{
            nfa:   nfa, 
            tasks: tasks,
        }
    }


    pub(crate)fn into_dfa(&mut self)->(Vec<Row<'a>>,usize) {
        let mut table = vec![];
        let start = DfaBuilder::build_table(&mut table, self.nfa, &mut self.tasks);
        (table,start)
    }

    fn build_table(
        table: &mut Vec<Row<'a>>,
        nfa:   &'a NFA,
        tasks: &mut Vec<HashedTask<'a>>
    ) -> usize {

        let mut row = Row (vec![]);

        while let Some(task) = tasks.pop() {
            for (rule, indexs) in task.map {
                let new_task = &mut vec![];
                new_task.push(Task::from_index(nfa,&indexs).hash());

                let next = DfaBuilder::build_table(table, nfa, new_task);
                row.0.push((rule,next));
            }
           
        }
        table.push(row);
        table.len() - 1

    }

}




enum TaskState<'a> {
    Match,
    Out(&'a Rule, &'a usize)
}
struct Task<'a>(Vec<TaskState<'a>>);

impl<'a> Task<'a> {
    pub(crate)fn from_state(nfa:&'a NFA, state:&'a State)->Task<'a> {
        match state {
            State::Match=> {
                return Task(vec![TaskState::Match])
            }
            State::Out(rule, id)=> {
                return Task(vec![TaskState::Out(rule, id)])
            }
            State::Split(left, right)=> {
                return Task::unwrap_split(nfa, *left, *right)
            }
        }  
    }

    pub(crate)fn from_index(nfa:&'a NFA, indexs:&Vec<&'a usize>)->Task<'a> {
        let mut task = Task(vec![]);
        for i in indexs {
            task.0.extend(Task::from_state(nfa, &nfa.get_states()[**i]).0);
        }
        task
    } 
 

    fn unwrap_split(nfa: &'a NFA, id1:usize, id2:usize)->Task<'a> {
        let mut task_list = Task (vec![]); 
        let state1 = &nfa.get_states()[id1];
        let state2 = &nfa.get_states()[id2];  
        
        match state1 { 
            State::Match => task_list.0.push(TaskState::Match),
            State::Out(rule,index) =>{
                task_list.0.push(TaskState::Out(&rule, &index));
            }
            State::Split(left, right) =>{
                task_list.0.extend(Task::unwrap_split(nfa, *left, *right).0);
            }
        }
        match state2 { 
            State::Match => task_list.0.push(TaskState::Match),
            State::Out(rule,index) =>{
                task_list.0.push(TaskState::Out(&rule, &index));
            }
            State::Split(left, right) =>{
                task_list.0.extend(Task::unwrap_split(nfa, *left, *right).0);
            }
        }
        task_list
    }

    ///description
    ///Hashes rules in task
    ///
    ///return 
    ///HashedTask (&'Rule, Vec<&usize>)
    ///
    pub fn hash(self)-> HashedTask<'a>{
        let mut map:HashMap<&Rule,Vec<&usize>> = HashMap::new();
        let mut match_state = false;

        for task in self.0.into_iter() {
            match task {
                TaskState::Match =>{
                    match_state = true;
                }
                TaskState::Out(rule, index)=>{
                    map.entry(&rule).or_default().push(&index);
                }
            }
        }
        HashedTask{matched:match_state, map:map}
    }
}

pub struct HashedTask<'a>{
    matched: bool,
    map:HashMap<&'a Rule,Vec<&'a usize>>
}

mod test {
    use super::*;
    use crate::make_nfa;
    
    #[test]
    pub fn test_into_dfa(){
        let nfa = make_nfa!("tra.n","traffic","brain");
        
        let mut builder = DfaBuilder::init(&nfa);

        let (table,start)  = builder.into_dfa();
        for row in table {
            println!("{}",row)
        }
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
