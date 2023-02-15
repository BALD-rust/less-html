use std::iter::Peekable;
use crate::{ParsedHtml, StrippedHtml, Element, TagKind};
use kuchiki;
use anyhow::Result;
use crate::util::optional_append;

pub fn tag_from_str(s: &str) -> TagKind {
    match s {
        "html" => TagKind::Html,
        "meta" => TagKind::Meta,
        "title" => TagKind::Title,
        "script" => TagKind::Script,
        "head" => TagKind::Head,
        "body" => TagKind::Body,
        "div" => TagKind::Div,
        "span" => TagKind::Span,
        "input" => TagKind::Input,
        "label" => TagKind::Label,
        "table" => TagKind::Table,
        "ul" => TagKind::UnorderedList,
        "li" => TagKind::ListItem,
        "style" => TagKind::Style,
        "b" => TagKind::Bold,
        "i" => TagKind::Italic,
        "h1" => TagKind::Heading(1),
        "a" => TagKind::Link,
        "p" => TagKind::Paragraph,
        "code" => TagKind::Code,
        "br" => TagKind::LineBreak,
        _ => TagKind::Unknown
    }
}

pub fn to_html_tag(kind: &TagKind) -> String {
    match kind {
        TagKind::Html => String::from("html"),
        TagKind::Meta => String::from("meta"),
        TagKind::Title => String::from("title"),
        TagKind::Script => String::from("script"),
        TagKind::Head => String::from("head"),
        TagKind::Body => String::from("body"),
        TagKind::Div => String::from("div"),
        TagKind::Span => String::from("span"),
        TagKind::Input => String::from("input"),
        TagKind::Label => String::from("label"),
        TagKind::Table => String::from("table"),
        TagKind::UnorderedList => String::from("ul"),
        TagKind::ListItem => String::from("li"),
        TagKind::Style => String::from("style"),
        TagKind::Bold => String::from("b"),
        TagKind::Italic => String::from("i"),
        TagKind::Heading(u32) => String::from("h1"),
        TagKind::Link => String::from("a"),
        TagKind::Paragraph => String::from("p"),
        TagKind::Code => String::from("code"),
        TagKind::LineBreak => String::from("br"),
        TagKind::Unknown => String::from("div") // doesnt do anything, so this is probably a "decent" default, until we error on it.
    }
}

/// Default stripping behaviour. Does not remove any content.
pub fn passthrough(element: &kuchiki::ElementData) -> Option<Element> {
    Some(Element::Tag(tag_from_str(&element.name.local.to_string())))
}

fn parse_node<F>(node: kuchiki::NodeRef, strip_fn: &F) -> Option<Element> where F: Fn(&kuchiki::ElementData) -> Option<Element> {
    if let Some(text) = node.clone().into_text_ref() {
        return Some(Element::Text(text.take()));
    }

    if let Some(elem) = node.clone().into_element_ref() {
        return strip_fn(&elem);
    }

    None
}

fn strip_node_recursive<F>(node: kuchiki::NodeRef, strip_fn: &F) -> Option<Vec<Element>> where F: Fn(&kuchiki::ElementData) -> Option<Element> {
    // 1. Parse this node
    let this = parse_node(node.clone(), strip_fn);

    // 2. Parse children
    let children: Option<Vec<Element>> = if let Some(elem) = node.clone().into_element_ref() {
        if this.is_some() {
            Some(node.children()
                .flat_map(|node| strip_node_recursive(node.clone(), strip_fn))
                .fold(vec![], |acc, elem| [acc, elem].concat())
            )
        } else { None }
    } else { None };

    let mut result = vec![];
    optional_append(&mut result, this.as_ref().map(std::slice::from_ref));
    optional_append(&mut result, children.as_ref().map(Vec::<_>::as_slice));

    if let Some(Element::Tag(tag)) = this {
        result.push(Element::EndTag(tag));
    }

    Some(result)
}


/// Do a context free strip of a document. This means that only one element of the original HTML can be examined at a time.
pub fn context_free_strip<F>(dom: &ParsedHtml, strip_fn: &F) -> Result<StrippedHtml> where F: Fn(&kuchiki::ElementData) -> Option<Element> {
    let elems: Vec<Element> = dom.dom.children()
        .flat_map(|node| strip_node_recursive(node.clone(), strip_fn))
        .fold(vec![], |acc, elem| [acc, elem].concat());

    Ok(
        StrippedHtml {
            0: elems
        }
    )
}

pub type ElementIter<'a> = Peekable<std::slice::Iter<'a, Element>>;

#[macro_export]
macro_rules! ignore_element {
    ($elem_type:ident, $next:ident) => {
        if let less_html::Element::$elem_type(_) = $next { return Some(vec![$next.clone()]); }
    }
}

#[macro_export]
macro_rules! ignore_unit_element {
        ($elem_type:ident, $next:ident) => {
        if let less_html::Element::$elem_type = $next { return Some(vec![$next.clone()]); }
    }
}

/// Strip that can see the future. When calling strip_fn(), the iterator is always guaranteed to have a next value.
pub fn oracle_strip<F>(html: StrippedHtml, strip_fn: &F) -> Result<StrippedHtml> where F: Fn(&Element, &mut ElementIter) -> Option<Vec<Element>> {
    let mut result = vec![];
    let mut it = html.0.iter().peekable();

    while let Some(next) = it.next() {
        let items = strip_fn(next, &mut it);
        optional_append(&mut result, items.as_ref().map(Vec::<_>::as_slice));
    }

    Ok(StrippedHtml{
        0: result
    })
}