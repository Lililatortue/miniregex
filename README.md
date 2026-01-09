# Intro 
Regex automata project inspired by the two source below
For a better performance in every situation regex use the regex-automata library.

russ cox:
source: https://swtch.com/~rsc/regexp/regexp1.html

andrew gallant:
source: https://github.com/BurntSushi/rsc-regexp/tree/master/idiomatic-translation


## Overview

Small regex library in safe rust, that supports ASCII characters

## Instruction

Use the command make_nfa!(String + ) a small in memory regex

The nfa as two main function:
- determinize() which eagerly creates the dfa
- lazy() which returns a lazy dfa ( iterator )

P.S: lazy() should always be used since a dfa can become extremely big extremely quick



## Future Modification

- [ ] implementation of cursor for dfa
- [ ] mmap implementation for lazydfa ( soon )
- [ ] SIMD optimisation   ( later date )
