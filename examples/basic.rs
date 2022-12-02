use anyhow::Result;

fn main() -> Result<()> {
    let doc = less_html::Document::from_file(std::path::Path::new("yep.html"))?;
    let html = less_html::parse(&doc)?;

    println!("HTML: {:#?}", doc);
    println!("parsed: {:#?}", html);

    let stripped = less_html::strip::strip_all(&html)?;
    println!("stripped: {:#?}", stripped);

    Ok(())
}