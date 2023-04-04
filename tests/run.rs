use derive_entity::Entity;

#[derive(Entity)]
pub struct Book {
    id: u64,
    title: String,
    pages: u64,
    author: String,
}

#[test]
fn run() {
    let select_sql = Book::select();
    assert_eq!("SELECT id, title, pages, author FROM Book;", select_sql);
}
