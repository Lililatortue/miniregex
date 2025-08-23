

#[derive(Debug,Hash,PartialEq, Eq)]
pub enum Rule{
    Any,
    Equal(char),
}

impl Rule { 
    pub fn match_eq(&self,c:char)->bool{
        match &self {
            Self::Any     => true,
            Self::Equal(x)=>{*x == c},
        }
    }
}


type Id = usize;
#[derive(Debug)]
pub enum State {
    Out(Rule, Id),
    Split(Id, Id),
    Match
}
//https://github.com/BurntSushi/rsc-regexp/blob/master/idiomatic-translation/nfa.rsc
#[derive(Copy,Clone)]
//type DanglingOuts = (Id, u8); //u8 being either 0 or 1
//his style much safer cant have to potential 2 or 3 it is only ou1 or out2


#[derive(Debug)]
pub enum DanglingOuts {
    Out1(Id), 
    Out2(Id),
}


#[derive(Debug)]
pub struct Frag {
   pub adresse:Id,
   pub goto: Vec<DanglingOuts>
}


#[derive(Debug)]
/// The NFA in question
/// contains two variable
/// states acts as an arena 
/// the State in states contain one or two pointers to another adresse
/// The pointers default to 0 when first initialise
/// it also contains a variable start 
/// start is where the first condition starts
pub struct Graph {
    start: Id,
    states: Vec<State>,
    
}
impl Graph {
    pub fn init()->Graph{
        Graph{start:0,states: vec![]}
    }
    /// Quick implementation of Russ postfix parser
    /// My version uses a recursive descent to parse
    /// I keep it tho for the tests 
    pub fn new(postfix: &str)->Option<Self> {
        let mut graph = Graph {start:0 , states:vec![]};
        let mut stack:Vec<Frag> = vec![];       
        
        for byte in postfix.chars() {
            match byte {
                '.'=>{// for this regex this is append
                    let e2 = stack.pop().unwrap();
                    let e1 = stack.pop().unwrap();
                    graph.patch(&e1.goto, e2.adresse); 
                    stack.push(Frag{adresse: e1.adresse, goto: e2.goto});
                },


                '?'=>{//one or zero
                    let mut e1 = stack.pop().unwrap();//get from stack 
                    let start  = graph.malloc(State::Split(e1.adresse,0));// alloc split
                    e1.goto.push(DanglingOuts::Out2(start)); //combine outs
                    stack.push(Frag{adresse: start, goto: e1.goto})   // create new Frag
                },



                '+'=>{ //one or more
                    let e1 = stack.pop().unwrap();//get from stack
                    let start  = graph.malloc(State::Split(e1.adresse,0));// alloc
                    
                    graph.patch(&e1.goto, start);// patch out 
                    let out = vec![DanglingOuts::Out2(start)];
                    stack.push(Frag{adresse:e1.adresse, goto: out});
                },



                '*'=>{ //zero or more
                    let e1 = stack.pop().unwrap();//get from stack
                    let start  = graph.malloc(State::Split(e1.adresse,0));// alloc
                    
                    graph.patch(&e1.goto, start);// patch out 
                    let out = vec![DanglingOuts::Out2(start)];
                    stack.push(Frag{adresse: start, goto: out});
                },


                '|'=>{ // alternation
                    let e1 = stack.pop().unwrap();
                    let mut e2 = stack.pop().unwrap();
                    let start  = graph.malloc(State::Split(e1.adresse,0));// alloc
                    
                    e2.goto.extend(&e1.goto);
                    stack.push(Frag{adresse: start, goto: e2.goto})
                },


                '~'=>{
                    let start = graph.malloc(State::Out(Rule::Any,0));//points at the start
                    let out = vec![DanglingOuts::Out1(start)];
                    stack.push(Frag{adresse: start, goto: out});
                },


                _   =>{
                    let start = graph.malloc(State::Out(Rule::Equal(byte as char),0));//points at the start
                    let out = vec![DanglingOuts::Out1(start)]; 
                    stack.push(Frag{adresse: start, goto: out });
                }, 
            }
        }
        // connect the graph to accept state or match
        let e = stack.pop();
        match e {
            Some(e)=>{
                let m = graph.malloc(State::Match);
                graph.start= e.adresse;
                graph.patch(&e.goto,m);
            }
            None=> return None,
        }
        Some(graph)
    }   



    /// Adds a state to the graph
    /// and returns its index
    /// 
    fn malloc(&mut self, state: State)->Id{
        let start = self.states.len();
        self.states.push(state);
        start
    }


    /// Adds state with a rule
    /// points to index 0 when initialise
    /// 
    pub fn literal(&mut self,c: char)->Frag{
        let start = match c {
            '.'=> self.malloc(State::Out(Rule::Any,0)), 
            _ =>self.malloc(State::Out(Rule::Equal(c),0)),
        };

        let out = vec![DanglingOuts::Out1(start)];

        Frag{adresse: start, goto: out}
    }

    /// Grabs two frags and connects them togheter
    /// grabs pointer of frag 1 and connects it to index of frag 2
    ///
    pub fn concatenation(&mut self,e1: Frag, e2: Frag) -> Frag{
        self.patch(&e1.goto,e2.adresse);
        Frag{adresse:e1.adresse, goto:e2.goto}
    }
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn one_or_more(&mut self,mut e1:Frag)-> Frag {//+
        let split_adresse = self.malloc(State::Split(e1.adresse, 0));
                
        self.patch(&e1.goto, split_adresse);   
        let out = vec![DanglingOuts::Out2(split_adresse)];
        Frag{adresse:e1.adresse, goto: out}    
    }


    ///
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn zero_or_more(&mut self,mut e1:Frag)-> Frag {//*
        let start = self.malloc(State::Split(e1.adresse, 0));
        
        self.patch(&e1.goto, start);
        let out = vec![DanglingOuts::Out2(start)];
        Frag{adresse: start, goto:out}
    }


    ///
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn one_or_zero(&mut self,mut e1: Frag)->Frag {//?
        let start = self.malloc(State::Split(e1.adresse, 0));
        
        e1.goto.push(DanglingOuts::Out2(start));
        Frag{adresse: start,goto:e1.goto}
    }
    

    ///
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn alternation(&mut self,mut e1:Frag, e2:Frag)->Frag {

        let start = self.malloc(State::Split(e1.adresse, e2.adresse));

        e1.goto.extend(e2.goto);
        Frag{adresse: start, goto: e1.goto}
    }
    

    ///
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn finish(mut self, e: Frag)->Graph {
        let match_ = self.malloc(State::Match);
        self.patch(&e.goto, match_); 
        self.start = e.adresse;
        self
    }

    pub fn get_states(&self)-> &Vec<State> {
        &self.states
    }










    //https://github.com/BurntSushi/rsc-regexp/blob/master/idiomatic-translation/nfa.rsc
    //his approach i prefer it more its explicit 
    fn patch(&mut self, out:&[DanglingOuts], target: Id) {
        for &out in out.iter() {
            match out {
                DanglingOuts::Out1(id)=> match self.states[id] {
                    State::Out(_,ref mut id) => {
                       *id = target;
                    }
                    State::Split(ref mut id1,_) => {
                        *id1 = target;
                    }
                    _=>panic!("Cant be Match")
                }
                DanglingOuts::Out2(id)=> match self.states[id] {
                    State::Split(_,ref mut id2)=>{
                        *id2 = target;
                    }
                    _=>panic!("out2 can only be acces by split")
                }
            }
        }
    }


    pub fn cursor(&self)-> GraphCursor<'_>{
        let mut v = Vec::new();
        v.push(&self.states[self.start]);
        GraphCursor {graph:self, rules:v }
    }
}

//------------------Simulating NFA------------------------//

//reference a state and its literal compares and returns a result
pub struct GraphCursor<'a> {
    graph: &'a Graph,
    rules:Vec<&'a State> 
}
enum CursorResult {
    Match,
    Valid,
    Invalid,
}

impl<'a> GraphCursor<'a> {
    
    fn handle_split(&self,state:&'a State,c:char, list:&mut Vec<&'a State>){
        match state {
            State::Split(id1, id2)=> {
            
                self.handle_split(&self.graph.states[*id1],c,list);
                self.handle_split(&self.graph.states[*id2],c,list);
            }
            State::Out(rule,id)=> {
                if rule.match_eq(c) {
               
                    list.push(&self.graph.states[*id]);
                }
            }
            _ => list.push(state), 
        }
    }


    pub fn match_eq(&mut self,c:char)->CursorResult {
        let mut list = Vec::new();
        for &state in self.rules.iter() {
            match state {
                State::Split(_,_)=>{ 
                    self.handle_split(state, c, &mut list);//create states 
                }
                State::Out(rule,id)=>{
                    if rule.match_eq(c) {
                        list.push(&self.graph.states[*id])
                    }; 
                }
                State::Match=> return CursorResult::Match, 
            }
        }
        
        if list.is_empty() {
            CursorResult::Invalid
        } else {
            self.rules = list;
            CursorResult::Valid
        }
    }


    pub fn match_full(mut self, s:&str)->bool{
        for c in s.chars() {
            let result = self.match_eq(c);
            match result {
                CursorResult::Valid=>{ 
                    continue;
                },
                CursorResult::Invalid=>{
                    return false; 
                },
                CursorResult::Match=>{
                    return true;
                },
            }
        }
        for s in self.rules {
            match s {
                State::Match => return true, 
                _=> continue,
            }
        }
        false
    }
}




//---------------------test---------------------------//
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_build(){
        let graph = Graph::new("abb*c...").unwrap();
        let ab_cursor = graph.cursor();
 
      //  println!("{:?}",graph.states);
        
        assert_eq!(true, ab_cursor.match_full("abc"))

    }
}

