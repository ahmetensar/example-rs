use core::fmt;

pub struct Book<'a> {
    title: &'a str,
    author: &'a str,
    page_count: u32,
}

impl<'a> Book<'a> {
    pub fn new(title: &'a str, author: &'a str, page_count: u32) -> Book<'a> {
        Book {
            title,
            author,
            page_count,
        }
    }
}

impl fmt::Display for Book<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Kitap: {}, Yazar: {}, {} sayfa",
            self.title, self.author, self.page_count
        )
    }
}
