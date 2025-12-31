use std::rc::Rc;
use std::{collections::HashMap};
use crate::cursor::DfaResult;
use crate::nfa::{Nfa};
use crate::utils;
use crate::{graph::Rule};


type Key = Vec<usize>;
type Value = HashMap<Rule, Option<Rc<Key>>>;    // valid rules -> next states


enum Cache{
    Hit(bool),                        
    Miss(Option<Rc<Key>>),
}

//this struct does not keep reference to current step outside variable
pub struct LazyDfa<'ctx>{
    reference: &'ctx Nfa,
    cache  : HashMap<Rc<Key>,(bool, Value)>,
    key    : Option<Rc<Key>>,
    start  : Option<Rc<Key>>
}


impl<'ctx> LazyDfa<'ctx> { 

    pub fn new(nfa: &'ctx Nfa)-> Self { 
        LazyDfa { 
            reference: nfa,
            cache: HashMap::new(),
            key:   None, 
            start: None,
        }
    }

    pub fn next(& mut self, c: char)-> DfaResult {
        
        // Check if cache as the result. If it finds the value and it has a value registered
        // It mutates self.key to the next key and returns Hit.
        let option_ptr = match self.check_cache(&c)                                 
        {
            Cache::Hit(matched) =>
                if matched 
                {
                    self.key = self.start.clone();
                    return DfaResult::Match
                } 
                else 
                {
                    return DfaResult::Valid
                },
            Cache::Miss(option_ptr) => option_ptr,
        }; 
        
        //if the program reaches this point it means the program hasn't found a the result in the
        //cache
        if let Some(ptr) = option_ptr
        {
            return self.mutate_cache(&ptr, &c);
        }
        else if let Some(ptr) = self.key.clone() 
        {
            return self.mutate_cache(&ptr, &c)
        }
        self.init(&c)
    }
    




    fn check_cache(&mut self, c: &char)-> Cache {   
        let Some(key) = &self.key else {return Cache::Miss(None)};

        //check if cache contains state  
        let Some((matched, next_key)) = self.cache.get_mut(key)
        else { return Cache::Miss(None) };
        
        //check if rule was previously computed
        if let Some(Some(next_key)) = next_key.get(&Rule::Equal(*c))
                                              .or_else(|| next_key.get(&Rule::Any))
        {
            self.key = Some(next_key.clone());
            return Cache::Hit(*matched)
        }         
        //return current state and a miss
        return Cache::Miss(Some(key.clone()));
    }





    fn mutate_cache(&mut self, ptr: &Rc<Vec<usize>>, c: &char)-> DfaResult {

        let (matched, next_key) = utils::index_from_condition(&self.reference,&ptr, &c);
    
        //guard
        if next_key.is_empty() {
            self.key = self.start.clone();
            return DfaResult::Invalid 
        };
        let key = Rc::new(next_key); 
            
        //increment next
        self.key = Some(key.clone());

        //update cache
        let cache = self.cache
                        .entry(ptr.clone())
                        .or_default();
        cache.0 = matched;
        cache.1.insert(Rule::Equal(*c),None);
            
        //return result
        if matched
        {            
            self.key = self.start.clone(); // reset it 
            return DfaResult::Match;
        }
        return DfaResult::Valid;
    }
    

    fn init(&mut self, c: &char)->DfaResult{
        
        let nfa = &self.reference;
        let first_key = vec![nfa.start()];
        let (matched, next_key) = utils::index_from_condition(nfa, &first_key, c);
        

        if first_key.is_empty() {
            println!("init invalid");
            return DfaResult::Invalid
        };
        let first_key = Rc::new(first_key);
        let next_key = Rc::new(next_key);
        

        let cache = self.cache
                        .entry(first_key.clone())
                        .or_default();
        cache.0 = matched;
        cache.1.insert(Rule::Equal(*c), None);


        self.start = Some(first_key.clone()); 
        self.key = Some(next_key);

        //return result
        if matched
        {
            return DfaResult::Match;
        }
        return DfaResult::Valid;
    }
    


}



#[cfg(test)]
mod test {
    use crate::make_nfa;

    use super::*;

    #[test]
    pub fn orchastrate(){
        let nfa  = make_nfa!("dog|cat|fish");

        let mut lazy = LazyDfa::new(&nfa);
        
        assert_eq!(DfaResult::Valid,  lazy.next('d'));
        assert_eq!(DfaResult::Valid,  lazy.next('o'));
        assert_eq!(DfaResult::Match,  lazy.next('g'));
        
        assert_eq!(DfaResult::Valid,  lazy.next('c'));
        assert_eq!(DfaResult::Invalid,lazy.next('c'));

        assert_eq!(DfaResult::Valid,  lazy.next('f'));
        assert_eq!(DfaResult::Valid,  lazy.next('i'));
        assert_eq!(DfaResult::Valid,  lazy.next('s'));
        assert_eq!(DfaResult::Match,  lazy.next('h'));
    }

}
