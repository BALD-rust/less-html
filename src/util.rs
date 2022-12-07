use crate::Element;

pub fn optional_append(vec: &mut Vec<Element>, elems: Option<&[Element]>) {
    if let Some(elems) = elems {
        vec.extend_from_slice(&elems);
    }
}