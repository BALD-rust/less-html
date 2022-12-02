use crate::{ParsedHtml, StrippedHtml, Element};
use html_parser::{Dom, Node};
use anyhow::Result;

pub fn parse_node(node: &html_parser::Node) -> Option<Element> {
    match node {
        Node::Text(str) => {
            // TODO: return nothing here, parse actual text content in Element case.
            Some(Element::Text(str.clone()))
        }
        Node::Element(elem) => {
            Some(Element::Tag(elem.name.clone()))
        }
        Node::Comment(_) => { None }
    }
}

trait IterConcat<I> where I: Iterator {
    type Item;
    fn iter_concat(&self) -> Vec<Self::Item> where I: Iterator;
}

impl<I> IterConcat<I> for I where I: Iterator {
    type Item = I::Item;

    fn iter_concat(&self) -> Vec<Self::Item> where I: Iterator {
        self.fold(vec![], |acc, elem| [acc, elem].concat())
    }
}

pub fn strip_node(node: &Node) -> Option<Vec<Element>> {
    // 1. Parse this node
    let this = parse_node(&node);
    // 2. Parse children
    let children: Option<Vec<Element>> = if let Some(Node::Element(elem)) = node {
        Some(elem.children.iter()
            .flat_map(strip_node)
            .collect()
        )
    } else { None };
    // 3. Optionally put end tag

    todo!()
}

pub fn strip_all_recursive(dom: &ParsedHtml) -> Result<StrippedHtml> {
    let elems: Vec<Element> = dom.dom.children.iter()
        .flat_map(strip_node)
        .collect();
    Ok(
        StrippedHtml {
            0: elems
        }
    )
}

pub fn strip_all(dom: &ParsedHtml) -> Result<StrippedHtml> {
    let elems: Vec<Element> = dom.dom.children.iter().map(|root| -> Vec<Option<Element>> {
        root.into_iter().map(|node| -> Vec<Option<Element>> {
            vec![parse_node(&node)]
        })
        .fold(vec![], |acc, elem| vec![acc, elem].concat().to_vec())
    })
    .fold(vec![], |acc, elem| vec![acc, elem].concat().to_vec())
    .into_iter()
    .flat_map(|e| e)
    .collect();

    Ok(
        StrippedHtml {
            0: elems,
        }
    )
}