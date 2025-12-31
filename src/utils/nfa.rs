use crate::{nfa::{State, Nfa}};

//description
//Gets all states related to indexs it goes through epsillon states
//
//
//
pub(crate)fn states_from_index<'a,'b>(nfa: &'a Nfa, list: &'b [usize])->Vec<&'a State>{
    let mut v = vec![];

    for i in list.iter(){
        let Some(state) = nfa.get(*i) else {panic!("invalid index")};

        if let State::Split(left, right) = state {
            v.extend(states_from_index(nfa, &[*left, *right])) 
        } else {
            v.push(state); 
        }
    }
    v
}


//description
//Gets all states related to indexs it goes through epsillon states
//
//
//
pub(crate)fn next_index<'a,'b>(nfa: &'a Nfa, list: &'b [usize])->(bool, Vec<usize>) {
    let mut v = vec![];
    let mut b = false;    

    for i in list.iter() {
        let Some(state) = nfa.get(*i) else {panic!("invalid index")};

        match state {
            State::Match(_) => b = true,
            State::Out(_, next)=> v.push(*next),
            State::Split(left,right)=> { 
                let (b2, v2) = next_index(nfa, &[*left,*right]);
                v.extend(v2);
                b = b || b2;
            }
        }
    }
    (b, v)
}



//description
//Gets all states related to indexs it goes through epsillon states
//
//
//
pub(crate)fn index_from_condition<'a,'b>(nfa: &'a Nfa, list: &'b [usize], condition: &char)->(bool, Vec<usize>) {
    let mut v = vec![];
    let mut b = false;    

    for i in list.iter() {
        let Some(state) = nfa.get(*i) else {panic!("invalid index")};
        
        //so match state is always the next state
        //visual example
        //s1 - a > s2 - b > match
        //therefore checking next state is required
        match state {
            State::Match(_) => (),
            State::Out(rule, next) if rule.match_eq(*condition)=> {
                v.push(*next);
                b = b || nfa.is_match(*next);
            }
            State::Split(left,right)=> { 
                let (b2, v2) = index_from_condition(nfa, &[*left,*right], condition);
                v.extend(v2);
                b = b || b2
            }
            _=> continue,
        }
    }
    (b, v)
}





