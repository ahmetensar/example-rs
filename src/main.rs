use publications::publication::{book::Book, magazine::Magazine, Publication};

fn main() {
    let publications = vec![
        Publication::Book(Book::new("Yabancı", "Albert Camus", 256)),
        Publication::Book(Book::new("1984", "George Orwell", 328)),
        Publication::Book(Book::new("Sineklerin Tanrısı", "William Golding", 224)),
        Publication::Magazine(Magazine::new("Bilim ve Teknik", 123, "Yapay Zeka")),
        Publication::Magazine(Magazine::new("National Geographic", 145, "Doğa")),
        Publication::Magazine(Magazine::new("CHIP", 230, "Teknoloji")),
    ];

    for publication in publications {
        println!("{}", publication);
    }
}
