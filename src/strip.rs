use std::iter::Peekable;
use std::str::FromStr;
use crate::{ParsedHtml, Element};

use kuchiki;
use anyhow::Result;
use flat_html::FlatHtml;
use flat_html::TagKind;
use crate::util::optional_append;

/// Default stripping behaviour. Does not remove any content.
pub fn passthrough(element: &kuchiki::ElementData) -> Option<Element> {
    Some(Element::Tag(TagKind::from_str(&element.name.local.to_string()).ok()?))
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

pub(crate) fn strip_node_recursive<F>(node: kuchiki::NodeRef, strip_fn: &F) -> Option<Vec<Element>> where F: Fn(&kuchiki::ElementData) -> Option<Element> {
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
pub fn context_free_strip<F>(dom: &ParsedHtml, strip_fn: &F) -> Result<FlatHtml> where F: Fn(&kuchiki::ElementData) -> Option<Element> {
    let elems: Vec<Element> = dom.dom.children()
        .flat_map(|node| strip_node_recursive(node.clone(), strip_fn))
        .fold(vec![], |acc, elem| [acc, elem].concat());

    Ok(
        FlatHtml {
            0: elems
        }
    )
}

pub type ElementIter<'a> = Peekable<std::slice::Iter<'a, Element>>;

/// Do not strip away this element, instead continue the oracle.
#[macro_export]
macro_rules! keep_element {
    ($elem_type:ident, $next:ident) => {
        if let less_html::Element::$elem_type(_) = $next { return Some(vec![$next.clone()]); }
    }
}

/// Do not strip away this element, instead continue the oracle.
#[macro_export]
macro_rules! keep_unit_element {
        ($elem_type:ident, $next:ident) => {
            if let less_html::Element::$elem_type = $next { return Some(vec![$next.clone()]); }
        }
}

#[macro_export]
macro_rules! keep_this {
    ($next:ident) => {
        return Some(vec![$next.clone()]);
    }
}

/// Strip that can see the future. When calling strip_fn(), the iterator is always guaranteed to have a next value.
pub fn oracle_strip<F>(html: FlatHtml, strip_fn: &F) -> Result<FlatHtml> where F: Fn(&Element, &mut ElementIter) -> Option<Vec<Element>> {
    let mut result = vec![];
    let mut it = html.0.iter().peekable();

    while let Some(next) = it.next() {
        let items = strip_fn(next, &mut it);
        optional_append(&mut result, items.as_ref().map(Vec::<_>::as_slice));
    }

    Ok(FlatHtml {
        0: result
    })
}