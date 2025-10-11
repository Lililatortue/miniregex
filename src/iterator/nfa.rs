use crate::{graph::Rule, nfa::{State, NFA}};
#[derive(Debug)]
pub enum IterResult<'a>{
    Match,
    Out(&'a Rule, &'a usize),
    Epsilon(usize),
}
///description
///allows to see nodes in proximity 
///
pub struct NfaBfsIter<'a>{
    nfa : &'a NFA, 
    next: Vec<usize>
}
impl<'a> NfaBfsIter<'a> {
    
    pub fn init(nfa: &'a NFA)->Self{
        NfaBfsIter { nfa:nfa , next:vec![nfa.get_start_id()] }
    }

    fn unwrap_split(&self,l:usize, r:usize)->Vec<IterResult<'a>> {
        let mut v = vec![];
        let mut n = vec![];

        for &i in &[l,r]{
            match &self.nfa.get_states()[i] {
                State::Match=>v.push(IterResult::Match),
                State::Out(r,i)=>{
                    v.push(IterResult::Out(r, i));
                    n.push(*i);
                }
                State::Split(l,r)=>{
                    v.push(IterResult::Epsilon(*l));
                    v.push(IterResult::Epsilon(*r));
                    v.extend(self.unwrap_split(*l, *r));
                },
            }
        }
        v
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
//infinite recursion doesnt work properly it still works for one off iteration

impl<'a> Iterator for NfaBfsIter<'a> {
    type Item = Vec<IterResult<'a>>;
    fn next(&mut self)->Option<Self::Item> { 
      //  let mut futur_next = vec![];
        let result = self.next
            .iter()
            .flat_map(|i| {
                match &self.nfa.get_states()[*i]{
                    State::Match=> vec![IterResult::Match],
                    State::Out(r,i)=>{
                      //  futur_next.push(*i);
                        vec![IterResult::Out(&r, &i)]
                    } 
                    State::Split(l,r)=>{ 
                        let mut result = vec![IterResult::Epsilon(*l)];
                        result.push(IterResult::Epsilon(*r));
                        result.extend(self.unwrap_split(*l, *r));
                     //   futur_next.extend(next);
                        result
                    },
                } 
            }).collect::<Vec<_>>();

        //self.next = futur_next; to prevent infinite recursion
        if result.is_empty(){
            return None;
        }
            Some(result)
    }
}


#[cfg(test)]
mod test {
    use crate::make_nfa;

    use super::*;
    #[test]
    pub fn test_from_index(){
        let nfa = make_nfa!("a(bc)*");
        println!("{}",&nfa);
        let list = NfaBfsIter::from_index(&nfa, nfa.get_start_id());
        println!("{:?}",list);
        let list = NfaBfsIter::from_index(&nfa,3_usize);
        println!("{:?}",list);
    }
}
