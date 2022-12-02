use crate::{ParsedHtml, StrippedHtml, Element};
use html_parser::{Dom, Node};
use anyhow::Result;

pub fn strip_node_default(element: &html_parser::Element) -> Option<Element> {
    Some(Element::Tag(element.name.clone()))
}

pub fn parse_node<F>(node: &html_parser::Node, strip_fn: &F) -> Option<Element> where F: Fn(&html_parser::Element) -> Option<Element> {
    match node {
        Node::Text(str) => {
            Some(Element::Text(str.clone()))
        }
        Node::Element(elem) => {
            strip_fn(&elem)
        }
        Node::Comment(_) => { None }
    }
}

fn optional_append(vec: &mut Vec<Element>, elems: Option<&[Element]>) {
    if let Some(elems) = elems {
        vec.extend_from_slice(&elems);
    }
}

pub fn strip_node_recursive<F>(node: &Node, strip_fn: &F) -> Option<Vec<Element>> where F: Fn(&html_parser::Element) -> Option<Element> {
    // 1. Parse this node
    let this = parse_node(&node, strip_fn);

    // 2. Parse children
    let children: Option<Vec<Element>> = if let Node::Element(elem) = node {
        if this.is_some() {
            Some(elem.children.iter()
                .flat_map(|node| strip_node_recursive(node, strip_fn))
                .fold(vec![], |acc, elem| [acc, elem].concat())
            )
        } else { None }
    } else { None };

    let mut result = vec![];
    optional_append(&mut result, this.as_ref().map(std::slice::from_ref));
    optional_append(&mut result, children.as_ref().map(Vec::<_>::as_slice));

    if let Some(Element::Tag(tag)) = &this {
        result.push(Element::EndTag(tag.clone()));
    }

    Some(result)
}

pub fn strip_all_recursive<F>(dom: &ParsedHtml, strip_fn: &F) -> Result<StrippedHtml> where F: Fn(&html_parser::Element) -> Option<Element> {
    let elems: Vec<Element> = dom.dom.children.iter()
        .flat_map(|node| strip_node_recursive(node, strip_fn))
        .fold(vec![], |acc, elem| [acc, elem].concat());
    Ok(
        StrippedHtml {
            0: elems
        }
    )
}

// pub fn strip_all(dom: &ParsedHtml) -> Result<StrippedHtml> {
//     let elems: Vec<Element> = dom.dom.children.iter().map(|root| -> Vec<Option<Element>> {
//         root.into_iter().map(|node| -> Vec<Option<Element>> {
//             vec![parse_node(&node)]
//         })
//         .fold(vec![], |acc, elem| vec![acc, elem].concat().to_vec())
//     })
//     .fold(vec![], |acc, elem| vec![acc, elem].concat().to_vec())
//     .into_iter()
//     .flat_map(|e| e)
//     .collect();
//
//     Ok(
//         StrippedHtml {
//             0: elems,
//         }
//     )
// }