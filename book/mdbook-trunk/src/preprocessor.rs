use log::debug;
use mdbook::{
    book::{Book, Chapter},
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd};

use crate::parser::parse_blocks;

pub struct TrunkPreprocessor;

impl TrunkPreprocessor {
    pub fn new() -> Self {
        Self
    }

    fn is_start_event(event: &Event) -> bool {
        if let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) = event {
            let tags = tag
                .split(',')
                .map(|tag| tag.trim().to_lowercase())
                .collect::<Vec<_>>();
            tags.len() >= 2 && tags[0] == "toml" && tags[1] == "trunk"
        } else {
            false
        }
    }

    fn is_end_event(event: &Event) -> bool {
        matches!(event, Event::End(TagEnd::CodeBlock))
    }

    fn process_chapter(&self, chapter: &mut Chapter) -> Result<(), mdbook::errors::Error> {
        let blocks = parse_blocks(
            &chapter.content,
            TrunkPreprocessor::is_start_event,
            TrunkPreprocessor::is_end_event,
        );

        debug!("{:?}", blocks);

        Ok(())
    }
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

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> Result<Book, mdbook::errors::Error> {
        let mut book = book.clone();

        for section in &mut book.sections {
            if let BookItem::Chapter(chapter) = section {
                self.process_chapter(chapter)?;
            }
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

#[cfg(test)]
mod test {
    use mdbook::preprocess::CmdPreprocessor;

    use super::*;

    #[test]
    fn test_run() {
        let input_json = r##"[
            {
                "root": "/path/to/book",
                "config": {
                    "book": {
                        "authors": ["AUTHOR"],
                        "language": "en",
                        "multilingual": false,
                        "src": "src",
                        "title": "TITLE"
                    },
                    "preprocessor": {
                        "nop": {}
                    }
                },
                "renderer": "html",
                "mdbook_version": "0.4.21"
            },
            {
                "sections": [
                    {
                        "Chapter": {
                            "name": "Chapter 1",
                            "content": "# Chapter 1\n{{#trunk example.rs}}\n",
                            "number": [1],
                            "sub_items": [],
                            "path": "chapter_1.md",
                            "source_path": "chapter_1.md",
                            "parent_names": []
                        }
                    }
                ],
                "__non_exhaustive": null
            }
        ]"##
        .as_bytes();

        let (ctx, book) = CmdPreprocessor::parse_input(input_json).unwrap();
        let expected_book = book.clone();

        let result = TrunkPreprocessor::new().run(&ctx, book);
        assert!(result.is_ok());

        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }
}
