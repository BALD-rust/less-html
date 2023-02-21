input = """Html, // <html>
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
"""

try:
    for line in input.split("\n"):
        parts = line.split(",")
        enum = parts[0]
        tag = parts[1].split("<")[1].removesuffix(">")
        print(f"\"{tag}\" => TagKind::{enum},")
except:
    pass

for line in input.split("\n"):
    parts = line.split(",")
    enum = parts[0]
    tag = parts[1].split("<")[1].removesuffix(">")
    print(f"TagKind::{enum} => String::from(\"{tag}\")")
