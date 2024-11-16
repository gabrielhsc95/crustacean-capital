pub enum TermUnit {
    Day,
    Week,
    Month,
    Year,
}

pub struct Term {
    value: u8,
    unit: TermUnit,
}
