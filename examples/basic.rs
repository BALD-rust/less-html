use std::fs::File;
use std::io::Write;
use anyhow::Result;
use less_html::{Element, StrippedHtml};

fn to_html(stripped: &StrippedHtml) -> String {
    stripped.0.iter().map(|element| -> String {
        match element {
            Element::Text(str) => str.clone(),
            Element::Tag(name) => "<".to_owned() + name + ">",
            Element::EndTag(name) => "</".to_owned() + name + ">"
        }
    })
    .fold(String::from(""), |acc, elem| acc + &elem)
}

fn main() -> Result<()> {
    let doc = less_html::Document::from_file(std::path::Path::new("example.html"))?;
    let html = less_html::parse(&doc)?;

    println!("HTML: {:#?}", doc);
    println!("parsed: {:#?}", html);

    let stripped = less_html::strip::strip_all_recursive(&html)?;

    let mut file = File::create(std::path::Path::new("output.html"))?;
    file.write(to_html(&stripped).as_bytes())?;
    Ok(())
}