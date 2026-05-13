use std::slice::Iter;

pub enum Propnames {
    ID,
    LABEL
}

impl Propnames {
    pub fn as_str(&self) -> &'static str {
        match self {
            Propnames::ID => {"Id"}
            Propnames::LABEL => {"Label"}
        }
    }

    pub fn iterator() -> Iter<'static, Propnames> {
        static DIRECTIONS: [Propnames; 2] = [Propnames::ID, Propnames::LABEL];
        DIRECTIONS.iter()
    }
}
