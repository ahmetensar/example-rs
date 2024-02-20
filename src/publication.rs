pub mod book;
pub mod magazine;

use core::fmt;

use self::book::Book;
use self::magazine::Magazine;

pub enum Publication<'a> {
    Book(Book<'a>),
    Magazine(Magazine<'a>),
}

impl fmt::Display for Publication<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Publication::Book(book) => write!(f, "{}", book),
            Publication::Magazine(magazine) => write!(f, "{}", magazine),
        }
    }
}
