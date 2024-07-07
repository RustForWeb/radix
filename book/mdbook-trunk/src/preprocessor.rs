use mdbook::{
    book::{Book, Chapter},
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};

pub struct TrunkPreprocessor;

impl TrunkPreprocessor {
    pub fn new() -> Self {
        Self
    }

    fn process_chapter(&self, _chapter: &mut Chapter) {}
}

impl Default for TrunkPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for TrunkPreprocessor {
    fn name(&self) -> &str {
        "trunk"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        let mut book = book.clone();

        book.for_each_mut(|section| match section {
            BookItem::Chapter(chapter) => self.process_chapter(chapter),
            BookItem::Separator => {}
            BookItem::PartTitle(_) => {}
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}
