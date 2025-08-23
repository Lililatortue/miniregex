use crate::graph::Graph;

pub mod graph;
/*
pub enum Token {
    Alt,            // Concat  | '|' Concat
    Concat,         // Atom    | Operande
    Atom(char,char),// literal | literal . Operande
    Operande(char), // Operande| '+'  '*'  '?'
    Literal(char),  // Char    | '(' Regex ')'
}
*/


pub struct Parser<'a> {
    buffer: Option<char>,
    graph: graph::Graph,
    iter : std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Iterator for Parser<'a> {
    type Item = char;

    fn next(&mut self)->Option<Self::Item> {
        self.buffer.take().or_else(||self.iter.next())
    }
}



use crate::graph::Frag;
impl<'a> Parser<'a> {
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
        match (c , self.iter.peek()) {
            ('\\', Some(c @ ('('|')')))=> {

                stack.push(self.graph.literal(*c));
            }

            (c @ ('a'..='z'|
                  'A'..='Z'|
                  '1'..='9'),_)=> 
            {

                stack.push(self.graph.literal(c));
            }

            ('(',_)=> {
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
    pub fn new(s: &'a str)->Self {
        let iter = s.chars().peekable();
        let graph = Graph::init();
        Parser {graph: graph, iter: iter, buffer:None}
    }
    pub fn parse(mut self)->Graph {
        let mut frag = self.alternation();
        let e = frag.pop().unwrap();
        self.graph.finish(e)
    }
}

       

#[cfg(test)]
mod tests {
    use crate::graph::Graph;
    #[test]
    pub fn test(){
        let graph = Parser::new("a(bb)+|b").parse();
        let test_cursor = graph.cursor();
        
        assert_eq!(false, test_cursor.match_full("abc"))

    }
    use super::*;
    
}
