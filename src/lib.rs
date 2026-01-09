mod nfa;
pub use nfa::Nfa;
mod dfa;
pub use dfa::Dfa;


mod graph;

pub mod cursor;
pub use cursor::{LazyDfa};

mod utils;
use graph::*;
/*
pub enum Token {
    Alt,            // Concat  | '|' Concat
    Concat,         // Atom    | Operande
    Atom(char,char),// literal | literal . Operande
    Operande(char), // Operande| '+'  '*'  '?'
    Literal(char),  // Char    | '(' Regex ')'
}
*/

///description:
///takes multiple strings and creates one big nfa
///
///parameters:
///one or more strings
///
///return
///NFA
///
#[macro_export]
macro_rules! make_nfa {
    ($ ( $x:expr ),+ $(,)?) => {{
        
            let mut s = String::new();
            $(
                if !s.is_empty() {

                    s.push('|')
                }
                s.push_str($x);
            )+
           $crate::Parser::new(&s,$crate::nfa::Nfa::init()).parse()      
    }};
}

#[macro_export]
macro_rules! make_dfa {
    ($ ($x:expr),+ $(,)?)=> {{
        $crate::make_nfa!($($x),+).determinize();

    }};
}


//#[macro_export]
//macro_rules! make_lexical_transducer {
    //($ (($x: expr, $y:expr)),+ ) => {
    //   let mut FSA
  //  };
//}

pub struct Parser<'a, T: Graph> {
    buffer: Option<char>,
    graph: T,
    iter : std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a, T:Graph> Iterator for Parser<'a, T> {
    type Item = char;

    fn next(&mut self)->Option<Self::Item> {
        self.buffer.take().or_else(||self.iter.next())
    }
}




use crate::graph::Frag;


impl<'a, T: Graph> Parser<'a, T> {
//------------------------logic------------------------//
    fn alternation(&mut self)->Vec<Frag> {
        let mut stack = vec![];

        self.concatenation(&mut stack);
        loop {
            let Some(c) = self.peek() else {return stack};
            match c {
                '|' =>{
                    self.next();
                    
                    stack.extend(self.alternation());
         
                    let Some(e2) = stack.pop() else {panic!("error 1")};
                    let Some(e1) = stack.pop() else {panic!("error 2")};
                    stack.push(self.graph.alternation(e1, e2));
                    break;
                    },
                ')' =>{return stack;}
                 _  => break,
            }
        }
        stack
    }

    fn concatenation(&mut self,stack:&mut Vec<Frag>) {           
        self.operande(stack);

        loop { 
            if stack.len()>=2 {
                let Some(e2) = stack.pop() else {panic!("not enough in stack to concat")};
                let Some(e1) = stack.pop() else {panic!("not enough in stack to concat")};
                stack.push(self.graph.concatenation(e1,e2));         
            } else {
                break;
            }
        }
    }

    fn operande(&mut self, stack:&mut Vec<Frag>) {
        loop { 

            let Some(c) = self.peek() else {break};

            match c {
                '|'=>{
                    break;
                }
                ')'=>{
                    return;
                }
                '*'=>{
                    self.next();
                    let Some(e1) = stack.pop() else {panic!("not enough in stack to concat")};
                    stack.push(self.graph.zero_or_more(e1));
                }
                '+'=>{
                    self.next();
                    let Some(e1) = stack.pop() else {panic!("not enough in stack to concat")};

                    stack.push(self.graph.one_or_more(e1));
                }
                '?'=>{
                    self.next();
                    let Some(e1) = stack.pop() else {panic!("not enough in stack to concat")};

                    stack.push(self.graph.one_or_zero(e1));
                }
                _      =>{
                    self.literal(stack);
                }
            }
        }

    }
    fn literal(&mut self, stack: &mut Vec<Frag>) {

        let Some(c)=self.next() else {return};
        match c {
            '\\'=> {
                let p = match self.peek() {
                    Some(c@ ('('|')'|'{'|'}'|'*'))=> *c,
                    _ =>{
                        stack.push(self.graph.literal(c));
                        return
                    }
                };
                self.next();
                stack.push(self.graph.literal(p));
            }

            c @ ('a'..='z'|
                 'A'..='Z'|
                 '1'..='9'|
                 '/' | '.' 
                 )=> {
                stack.push(self.graph.literal(c));
            }

            '('=> {
                stack.extend(self.alternation());
                match self.next() {
                    Some(')') => return,
                    _ => panic!("missing closing parenthesis"),
                }
            }

            c @ _=>{println!("Bad token{:?}",c);panic!()},
        }
    }


//----------------------helper-----------------------//
    pub fn peek(&mut self)-> Option<&char> {
        self.buffer.as_ref().or_else(||self.iter.peek()) 
    }

//-----------------------init------------------------//
    pub fn new(s: &'a str, graph:T)->Self {
        let iter = s.chars().peekable();
        Parser {graph: graph, iter: iter, buffer:None}
    }

    //return finish graph
    pub fn parse(mut self)->T {
        let mut frag = self.alternation();
        let Some(e) = frag.pop() else {panic!("couldnt parse it")};
        self.graph.finish(e)
    }

    //returns unfinish graph, no match will be made if the 
    pub fn get_frag(mut self)->T {
        let _ = self.alternation();
        self.graph
    }
}

       

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    pub fn test_nfa_cursor(){
    }
    
}
