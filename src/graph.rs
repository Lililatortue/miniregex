pub mod lexic;
pub mod fsa;
pub trait Graph: Sized {
    ///description:
    ///
    ///
    ///expected behavior: 
    ///Should create a Frag with its rule enforced and allocated it in the states vector
    ///
    ///parameters:
    ///c:char -> is the rule 
    ///
    ///return: 
    ///the Frag created
    ///
    fn literal(&mut self,c: char)->Frag;

    ///description:
    ///
    ///
    ///expected behavior:
    ///Should unite both e1:Frag ends to e2:Frag start
    ///then create a new Frag which as e1.start as its start and e2 ends as its ends
    ///
    ///parameters:
    ///e1:Frag -> the head
    ///e2:Frag -> the tail
    ///
    ///return:
    ///the Frag created
    ///
    fn concatenation(&mut self,e1: Frag, e2: Frag) -> Frag;

    
    ///description:
    ///
    ///
    ///parameters:
    ///
    ///
    ///return:
    ///
    fn one_or_more(&mut self,e1:Frag)->Frag;


    ///description:
    ///
    ///
    ///parameters:
    ///
    ///
    ///return:
    ///
    fn zero_or_more(&mut self,e1:Frag)-> Frag;


    ///description:
    ///
    ///
    ///parameters:
    ///
    ///
    ///return:
    ///
    fn one_or_zero(&mut self,e1: Frag)->Frag;
    

    ///description:
    ///
    ///
    ///parameters:
    ///
    ///
    ///return:
    ///
    fn alternation(&mut self,e1:Frag, e2:Frag)->Frag;
    

    ///description:
    ///
    ///
    ///parameters:
    ///
    ///
    ///return:
    ///
    fn finish(self, e: Frag)->Self;
    
    ///description:
    ///
    ///
    ///parameters:
    ///
    ///return:
    ///
    fn patch(&mut self, out:&[DanglingOuts], target: Id);

}



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

#[derive(Debug)]
pub enum DanglingOuts {
    Out1(Id), 
    Out2(Id),
}

type Id = usize;
#[derive(Debug)]
pub struct Frag {
   pub adresse:Id,
   pub goto: Vec<DanglingOuts>
}


