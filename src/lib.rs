mod error;

use html_parser as html;
use anyhow::Result;

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
    dom: html::Dom,
}

pub fn parse(doc: &Document) -> Result<ParsedHtml> {
    Ok(ParsedHtml {
        dom: html::Dom::parse(&doc.html)?
    })
}