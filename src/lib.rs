pub mod error;
pub mod strip;
mod util;

use std::str::FromStr;
use kuchiki;
use anyhow::Result;
pub use flat_html::{Element, FlatHtml, TagKind};
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

pub fn parse(doc: &Document) -> Result<FlatHtml> {
    let dom = kuchiki::parse_html().one(doc.html.clone());

    let elems: Vec<Element> = dom.children()
        .flat_map(|node| strip::strip_node_recursive(node.clone(), &strip_func))
        .flatten()
        .collect();

    Ok(
        FlatHtml {
            0: elems
        }
    )
}

fn strip_func(elem: &kuchiki::ElementData) -> Option<Element> {
    if elem.name.local.to_string() == "head" { return None; }

    // Default behaviour for browsers is to insert a line break before and after a <div>.
    // We should implement similar behaviour.
    // For now, to simulate this in the output, we won't ignore divs.
    if elem.name.local.to_string() == "div" { return Some(Element::LineBreak); }
    // Parse the string to a tag
    Some(Element::Tag(TagKind::from_str(&elem.name.local.to_string()).ok()?))
}