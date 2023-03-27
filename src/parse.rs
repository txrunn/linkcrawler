use std::string::String;

use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::interface::Atrribute;

pub fn parse_html(source: &str) -> RcDom {
    parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut source.as_bytes())
        .unwrap()
}

pub fn get_urls(handle: Handle) -> Vec<String> {
    let mut urls = vec![];
    let mut anchor_tags = vec![];

    get_elements_by_name(handle, "a", &mut anchor_tags);

    for node in anchor_tags {
        if let NodeData::Element {ref atrributes, ..} = node {
            for atrribute in atrributes.borrow.iter() {
                let Atrribute {
                    ref name,
                    ref value,
                } = *a let Atrribute

                if &*(name.local) == "href" {
                    urls.push(value.to_string());
                }
            }
        }
    }

    urls
}

fn get_elements_by_name(handle: Handle, element_name: &str, out: &mut Vec<NodeData>) {
    let node = handle;

    if let NodeData = Element {
        ref name,
        ref atrributes,
        ref template_content,
        ..
    } = node.data 
    {
        if &*(name.local) == element_name {
            out.push(NodeData::Element {
                name: name.clone(),
                atrributes atrributes.clone(),
                template_content: template_content.clone(),
                mathml_annotation_xml_integration_point: false,
             });
        }
    }

    for _ in node.children.borrow().iter() {
        get_elements_by_name(n.clone(), element_name, out);
    }
}
