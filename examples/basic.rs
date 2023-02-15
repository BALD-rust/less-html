use std::fs::File;
use std::io::Write;
use anyhow::Result;
use kuchiki::ElementData;
use less_html::{Element, StrippedHtml, strip::ElementIter, TagKind};
use less_html::strip::to_html_tag;

fn to_html(stripped: &StrippedHtml) -> String {
    stripped.0.iter().map(|element| -> String {
        match element {
            Element::Text(str) => str.clone(),
            Element::Tag(kind) => "<".to_owned() + &to_html_tag(kind) + ">",
            Element::EndTag(kind) => "</".to_owned() + &to_html_tag(kind) + ">",
            Element::LineBreak => String::from("</br>"),
            Element::IgnoreTag => String::from(""),
        }
    })
    .fold(String::from(""), |acc, elem| acc + &elem)
}


fn strip_func(elem: &kuchiki::ElementData) -> Option<Element> {
    if elem.name.local.to_string() == "head" { return None; }
    // Default behaviour for browsers is to insert a line break before and after a <div>.
    // We should implement similar behaviour.
    // For now, to simulate this in the output, we won't ignore divs.

    if elem.name.local.to_string() == "div" { return Some(Element::LineBreak); }
    // todo: always do passthrough pass first so we get a StrippedHTML with nice names to operate on,
    // instead of this stupidity
    Some(Element::Tag(less_html::strip::tag_from_str(&elem.name.local.to_string())))
}

// Should: Take every text element, and if it has a '\n' split it in a Text with the original text, and a LineBreak
fn remap_linebreaks(next: &Element, it: &mut ElementIter) -> Option<Vec<Element>> {
    less_html::ignore_unit_element!(IgnoreTag, next);
    less_html::ignore_element!(EndTag, next);
    less_html::ignore_unit_element!(LineBreak, next);

    if let Element::Text(contents) = next {
        if contents == "\n" { return Some(vec![Element::LineBreak]); }
    }

    Some(vec![next.clone()])
}

fn oracle(next: &Element, it: &mut ElementIter) -> Option<Vec<Element>> {
    less_html::ignore_element!(Text, next);
    less_html::ignore_unit_element!(IgnoreTag, next);
    less_html::ignore_unit_element!(LineBreak, next);
    less_html::ignore_element!(EndTag, next);

    match next {
        Element::Text(_) => { unimplemented!() }
        Element::Tag(kind) => {
            if *kind == TagKind::Table {
                // Todo: Write helper function to consume and remap elements until a condition is met
                let mut result = vec![];
                while let Some(child) = it.next() {
                    result.push(child.clone());
                    if let Element::EndTag(tag) = child {
                        if *tag == TagKind::Table {
                            return Some(result);
                        }
                    }
                }
            } else {
                // Todo: also add a macro for this (else_ignore!)
                return Some(vec![next.clone()]);
            }
        }
        Element::EndTag(_) => { unimplemented!() }
        Element::LineBreak => {
            while let Some(Element::LineBreak) = it.peek() { let _ = it.next(); }

            return Some(vec![Element::LineBreak]);
        }
        Element::IgnoreTag => {  unimplemented!() }
    }

    todo!()
}

fn main() -> Result<()> {
    let doc = less_html::Document::from_file(std::path::Path::new("html-files/monads.html"))?;
    let html = less_html::parse(&doc)?;

    // Default, no strip:
    // let stripped = less_html::strip::context_free_strip(&html, &less_html::strip::passthrough);

    let stripped = less_html::strip::context_free_strip(&html, &strip_func)?;
    // let stripped = less_html::strip::oracle_strip(stripped, &remap_linebreaks)?;
    let stripped = less_html::strip::oracle_strip(stripped, &oracle)?;
    println!("{:?}", stripped);

    let mut file = File::create(std::path::Path::new("output.html"))?;
    file.write(to_html(&stripped).as_bytes())?;
    Ok(())
}