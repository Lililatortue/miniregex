





impl FSA {
    pub fn init()->FSA{
        FSA{start:0,states: vec![]}
    }
       
    /// Adds a state to the graph
    /// and returns its index
    fn malloc(&mut self, state: State)->Id{
        let start = self.states.len();
        self.states.push(state);
        start
    }

    pub fn get_states(&self)-> &Vec<State> {
        &self.states
    }


    pub fn cursor(&self)-> GraphCursor<'_>{
        let mut v = Vec::new();
        v.push(&self.states[self.start]);
        GraphCursor {graph:self, rules:v }
    }
}


