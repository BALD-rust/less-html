pub mod error;
pub mod strip;
mod util;

use kuchiki;
use anyhow::Result;
use kuchiki::traits::TendrilSink;

use error::Error;


/// Original HTML document.
#[derive(Debug)]
pub struct Document {
    pub html: String
}

// We can potentially store attributes inside this enum
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TagKind {
    Html, // <html>
    Meta, // <meta>
    Title, // <title>
    Script, // <script>
    Head, // <head>
    Body, // <body>
    Div, // <div>
    Span, // <span>
    Input, // <input>
    Label, // <label>
    Table, // <table>
    UnorderedList, // <ul>
    ListItem, // <li>
    Style, // <style>
    Bold, // <b>
    Italic, // <i>
    Heading(u32), // <h{level}> .. ?
    Link, // <a>
    Paragraph, // <p>
    Code, // <code>
    LineBreak, // <br>
    Unknown,
    // TODO:
    // <dt>, <dd>, <sup>, <pre>
    // <th>, <tr>, <tb>, <td>
    // <math> ?
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
    Tag(TagKind),
    EndTag(TagKind),
    LineBreak,
    IgnoreTag // Setting the element to this will ignore the tag, but parse the children
}

#[derive(Debug)]
pub struct StrippedHtml(pub Vec<Element>);

pub fn parse(doc: &Document) -> Result<StrippedHtml> {
    let dom = kuchiki::parse_html().one(doc.html.clone());

    let elems: Vec<Element> = dom.children()
        .flat_map(|node| strip::strip_node_recursive(node.clone(), &strip_func))
        .fold(vec![], |acc, elem| [acc, elem].concat());

    Ok(
        StrippedHtml {
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
    // todo: always do passthrough pass first so we get a StrippedHTML with nice names to operate on,
    // instead of this stupidity
    Some(Element::Tag(strip::tag_from_str(&elem.name.local.to_string())))
}