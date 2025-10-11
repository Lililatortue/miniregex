use crate::{graph::Rule, nfa::{State, NFA}};
#[derive(Debug)]
pub enum IterResult<'a>{
    Match,
    Out(&'a Rule, &'a usize)
}
///description
///allows to see nodes in proximity 
///
///
///
///
pub struct NfaBfsIter<'a>{
    nfa : &'a NFA, 
    next: Vec<usize>
}
impl<'a> NfaBfsIter<'a> {
    
    pub fn init(nfa: &'a NFA)->Self{
        NfaBfsIter { nfa:nfa , next:vec![nfa.get_start_id()] }
    }

    fn unwrap_split(&self,l:usize, r:usize)->(Vec<IterResult<'a>>,Vec<usize>) {
        let mut v = vec![];
        let mut n = vec![];
        match &self.nfa.get_states()[l] {
            State::Match=>v.push(IterResult::Match),
            State::Out(r,i)=>{
                v.push(IterResult::Out(r, i));
                n.push(*i);
            }
            State::Split(l,r)=>{
                let (result,next) = self.unwrap_split(*l, *r);
                v.extend(result);
                n.extend(next);
            },
        }

        match &self.nfa.get_states()[r] {
            State::Match=>v.push(IterResult::Match),
            State::Out(r,i)=>{
                v.push(IterResult::Out(r, i));
                n.push(*i);
            }
            State::Split(l,r)=>{
                let (result,next) = self.unwrap_split(*l, *r);
                v.extend(result);
                n.extend(next);
            }
        }
        (v,n)
    }

    pub fn from_index(nfa: &'a NFA, index: usize)->Option<Vec<IterResult<'a>>> {
            let mut next = NfaBfsIter{nfa, next:vec![index]}; 
            next.next()
    }
    pub fn from_indexs(nfa: &'a NFA, indexs: Vec<usize>)->Option<Vec<IterResult<'a>>>{
            let mut next = NfaBfsIter{nfa, next:indexs}; 
            next.next()
    }
}

impl<'a> Iterator for NfaBfsIter<'a> {
    type Item = Vec<IterResult<'a>>;
    fn next(&mut self)->Option<Self::Item> { 
        let mut futur_next = vec![];
        let result = self.next
            .iter()
            .flat_map(|i| {
                match &self.nfa.get_states()[*i]{
                    State::Match=> vec![IterResult::Match],
                    State::Out(r,i)=>{
                        futur_next.push(*i);
                        vec![IterResult::Out(&r, &i)]
                    } 
                    State::Split(l,r)=>{
                        let (result, next) = self.unwrap_split(*l, *r);
                        futur_next.extend(next);
                        result
                    },
                } 
            }).collect::<Vec<_>>();

        self.next = futur_next;
        if result.is_empty(){
            return None;
        }
            Some(result)
    }
}
