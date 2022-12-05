use std::fs::File;
use std::io::Write;
use anyhow::Result;
use kuchiki::ElementData;
use less_html::{Element, StrippedHtml};

fn to_html(stripped: &StrippedHtml) -> String {
    stripped.0.iter().map(|element| -> String {
        match element {
            Element::Text(str) => str.clone(),
            Element::Tag(name) => "<".to_owned() + name + ">",
            Element::EndTag(name) => "</".to_owned() + name + ">",
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
    Some(Element::Tag(elem.name.local.to_string()))
}



fn main() -> Result<()> {
    let doc = less_html::Document::from_file(std::path::Path::new("monads.html"))?;
    let html = less_html::parse(&doc)?;

    // Default, no strip:
    // let stripped = less_html::strip::context_free_strip(&html, &less_html::strip::passthrough);

   let stripped = less_html::strip::context_free_strip(&html, &strip_func)?;

    let mut file = File::create(std::path::Path::new("output.html"))?;
    file.write(to_html(&stripped).as_bytes())?;
    Ok(())
}