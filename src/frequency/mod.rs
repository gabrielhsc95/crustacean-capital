use crate::term::Term;

pub struct Frequency {
    term: Term,
    value: f64,
}

impl Frequency {
    pub fn new(term: Term, value: f64) -> Self {
        Frequency { term, value }
    }
    pub fn period(&self) -> f64 {
        1.0 / self.value
    }

    pub fn term(&self) -> Term {
        self.term
    }
}
