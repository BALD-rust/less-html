pub mod error;
pub mod strip;

use kuchiki;
use anyhow::Result;
use kuchiki::traits::TendrilSink;

use error::Error;


/// Original HTML document.
#[derive(Debug)]
pub struct Document {
    pub html: String
}

impl Document {
    pub fn from_string(html: String) -> Result<Document> {
        Ok(Document {
            html
        })
    }

    pub fn from_file(path: &std::path::Path) -> Result<Document> {
        if !path.is_file() { return Err(anyhow::Error::from(Error::InvalidPath(path.to_str().unwrap().to_string()))); };

        Ok(Document {
            html: std::fs::read_to_string(&path)?
        })
    }
}

#[derive(Debug)]
pub struct ParsedHtml {
    dom: kuchiki::NodeRef
}

#[derive(Debug, Clone)]
pub enum Element {
    Text(String),
    Tag(String),
    EndTag(String),
    LineBreak,
    IgnoreTag // Setting the element to this will ignore the tag, but parse the children
}

#[derive(Debug)]
pub struct StrippedHtml(pub Vec<Element>);

pub fn parse(doc: &Document) -> Result<ParsedHtml> {
    Ok(ParsedHtml {
        dom: kuchiki::parse_html().one(doc.html.clone())
    })
}