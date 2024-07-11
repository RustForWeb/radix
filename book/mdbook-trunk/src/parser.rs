use std::ops::Range;

use anyhow::{bail, Result};
use log::debug;
use pulldown_cmark::{Event, Parser};

#[derive(Clone, Debug, PartialEq)]
pub struct Block<'a> {
    closed: bool,
    events: Vec<Event<'a>>,
    span: Range<usize>,
}

impl<'a> Block<'a> {
    pub fn new(first_event: Event<'a>, span: Range<usize>) -> Self {
        Block {
            closed: false,
            events: vec![first_event],
            span,
        }
    }
}

pub fn parse_blocks<IsStartFn, IsEndFn>(
    content: &str,
    is_start: IsStartFn,
    is_end: IsEndFn,
) -> Result<Vec<Block>>
where
    IsStartFn: Fn(&Event) -> bool,
    IsEndFn: Fn(&Event) -> bool,
{
    let mut blocks: Vec<Block> = vec![];

    for (event, span) in Parser::new(content).into_offset_iter() {
        debug!("{:?} {:?}", event, span);

        if is_start(&event) {
            if let Some(block) = blocks.last_mut() {
                if !block.closed {
                    bail!("Block is not closed. Nested blocks are not supported.");
                }
            }

            blocks.push(Block::new(event, span));
        } else if is_end(&event) {
            if let Some(block) = blocks.last_mut() {
                if !block.closed {
                    block.events.push(event);
                    block.closed = true;
                }
            } else {
                bail!("No open block.");
            }
        } else if let Some(block) = blocks.last_mut() {
            if !block.closed {
                block.events.push(event);
            }
        }
    }

    Ok(blocks)
}

#[cfg(test)]
mod test {
    use pulldown_cmark::{CodeBlockKind, CowStr, Tag, TagEnd};
    use test_log::test;

    use super::*;

    #[test]
    fn test_parse_blocks() -> Result<()> {
        let content = "\
        ```toml\n\
        key1 = \"value1\"\n\
        key2 = \"value2\"\n\
        ```";
        let expected: Vec<Block> = vec![Block {
            events: vec![
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from("toml")))),
                Event::Text(CowStr::from("key1 = \"value1\"\nkey2 = \"value2\"\n")),
                Event::End(TagEnd::CodeBlock),
            ],
            span: 0..43,
            closed: true,
        }];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) if tag == &CowStr::from("toml")),
            |event| matches!(event, Event::End(TagEnd::CodeBlock)),
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_parse_blocks_surrounded() -> Result<()> {
        let content = "\
        Some text before the code block.\n\
        \n\
        ```toml\n\
        key1 = \"value1\"\n\
        key2 = \"value2\"\n\
        ```\n\
        \n\
        Some text after the code block.";
        let expected: Vec<Block> = vec![Block {
            events: vec![
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from("toml")))),
                Event::Text(CowStr::from("key1 = \"value1\"\nkey2 = \"value2\"\n")),
                Event::End(TagEnd::CodeBlock),
            ],
            span: 34..77,
            closed: true,
        }];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) if tag == &CowStr::from("toml")),
            |event| matches!(event, Event::End(TagEnd::CodeBlock)),
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_parse_blocks_multiple() -> Result<()> {
        let content = "\
        First TOML block:\n\
        ```toml\n\
        key1 = \"value1\"\n\
        key2 = \"value2\"\n\
        ```\n\
        First non-TOML block:\n\
        ```shell\n\
        echo test\n\
        ```\n\
        Second TOML block:\n\
        ```toml\n\
        key3 = \"value3\"\n\
        key4 = \"value4\"\n\
        ```";
        let expected: Vec<Block> = vec![
            Block {
                events: vec![
                    Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from("toml")))),
                    Event::Text(CowStr::from("key1 = \"value1\"\nkey2 = \"value2\"\n")),
                    Event::End(TagEnd::CodeBlock),
                ],
                span: 18..61,
                closed: true,
            },
            Block {
                events: vec![
                    Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from("toml")))),
                    Event::Text(CowStr::from("key3 = \"value3\"\nkey4 = \"value4\"\n")),
                    Event::End(TagEnd::CodeBlock),
                ],
                span: 126..169,
                closed: true,
            },
        ];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) if tag == &CowStr::from("toml")),
            |event| matches!(event, Event::End(TagEnd::CodeBlock)),
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_parse_blocks_nested() -> Result<()> {
        let content = "*a **sentence** with **some** words*";

        let actual = parse_blocks(
            content,
            |event| {
                matches!(
                    event,
                    Event::Start(Tag::Emphasis) | Event::Start(Tag::Strong)
                )
            },
            |event| {
                matches!(
                    event,
                    Event::End(TagEnd::Emphasis) | Event::End(TagEnd::Strong)
                )
            },
        );

        assert_eq!(
            "Block is not closed. Nested blocks are not supported.",
            format!("{}", actual.unwrap_err().root_cause())
        );

        Ok(())
    }
}
