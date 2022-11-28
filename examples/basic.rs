use anyhow::Result;

fn main() -> Result<()> {
    let doc = less_html::Document::from_file(std::path::Path::new("example.html"))?;
    let html = less_html::parse(&doc)?;

    println!("HTML: {:#?}", doc);
    println!("Parser result: {:#?}", html);

    Ok(())
}