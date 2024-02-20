use core::fmt;

pub struct Magazine<'a> {
    title: &'a str,
    issue: u32,
    topic: &'a str,
}

impl<'a> Magazine<'a> {
    pub fn new(title: &'a str, issue: u32, topic: &'a str) -> Magazine<'a> {
        Magazine {
            title,
            issue,
            topic,
        }
    }
}

impl fmt::Display for Magazine<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dergi: {} - Sayı: {}, Konu: {}",
            self.title, self.issue, self.topic
        )
    }
}
