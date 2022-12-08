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

// We can potentially store attributes inside this enum
enum Tag {
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