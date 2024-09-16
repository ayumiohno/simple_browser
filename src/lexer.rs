use logos::{Lexer, Logos};

use crate::dom::*;

fn print_element(tag: &Element, depth: usize) {
    print!("{}<{}", "  ".repeat(depth), tag.tag_name);
    for (key, value) in &tag.attributes {
        print!(" {}:{}", key, value);
    }
    print!(">\n");
}

pub fn print_node(node: &Node, depth: usize) {
    match &node.node_type {
        NodeType::Element(element) => {
            print_element(&element, depth);
            for child in &node.children {
                print_node(child, depth + 1);
            }
            println!("{}</{}>", "  ".repeat(depth), element.tag_name);
        }
        NodeType::Text(text) => {
            println!("{}{}", "  ".repeat(depth), text.data);
        }
    }
}

pub fn dom_parser(input: &str) -> Node {
    let mut lex = TopLevelToken::lexer(input);
    let root = Node {
        node_type: NodeType::Element(Element {
            tag_name: "root".to_string(),
            attributes: AttrMap::new(),
        }),
        children: Vec::new(),
    };
    let mut stack = Vec::<Node>::new();
    stack.push(root);
    while let Some(token) = lex.next() {
        match token {
            Ok(TopLevelToken::OpenTagStart(element)) => {
                let new_node = Node {
                    node_type: NodeType::Element(element),
                    children: Vec::new(),
                };
                stack.push(new_node);
            }
            Ok(TopLevelToken::OpenTagEnd(_element)) => {
                let open_tag_start = stack.pop().unwrap();
                stack
                    .last_mut()
                    .unwrap()
                    .children
                    .push(Box::new(open_tag_start));
            }
            Ok(TopLevelToken::CloseTag(element)) => {
                let new_node = Node {
                    node_type: NodeType::Element(element),
                    children: Vec::new(),
                };
                stack.last_mut().unwrap().children.push(Box::new(new_node));
            }
            Ok(TopLevelToken::Text) => {
                let new_node = Node {
                    node_type: NodeType::Text(Text {
                        data: lex.slice().to_string(),
                    }),
                    children: Vec::new(),
                };
                stack.last_mut().unwrap().children.push(Box::new(new_node));
            }
            _ => {
                println!("{:?}", token);
            }
        }
    }
    return stack.pop().unwrap();
}

fn parse_element(
    lex: &mut Lexer<TopLevelToken>,
    start_bracket: &str,
    end_bracket: &str,
) -> Option<Element> {
    let slice = lex
        .slice()
        .strip_prefix(start_bracket)
        .unwrap()
        .strip_suffix(end_bracket)
        .unwrap()
        .to_string();
    let tag_name = slice.split_whitespace().next().unwrap().to_string();
    let attributes = slice.strip_prefix(&tag_name).unwrap();
    let mut attr_lex = AttributeToken::lexer(attributes);
    let mut attribute_map = AttrMap::new();
    while let Some(attr_token) = attr_lex.next() {
        match attr_token {
            Ok(AttributeToken::Key) => {
                let key = attr_lex.slice().strip_suffix("=").unwrap().to_string();
                attr_lex.next();
                let value = attr_lex.slice().to_string();
                attribute_map.insert(key, value);
            }
            _ => {
                println!("{:?}", attr_token);
            }
        }
    }
    return Some(Element {
        tag_name: tag_name,
        attributes: attribute_map,
    });
}

#[derive(Logos, Debug, PartialEq)]
enum TopLevelToken {
    #[regex("</[^>]+>", priority = 1, callback = |lex| parse_element(lex, "</", ">"))]
    OpenTagEnd(Element),
    #[regex("<[^>]+/>", priority = 2, callback = |lex| parse_element(lex, "<", "/>"))]
    CloseTag(Element),
    #[regex("<[^>/]+>", priority = 3, callback = |lex| parse_element(lex, "<", ">"))]
    OpenTagStart(Element),
    #[regex("[^<>]+", priority = 4)]
    Text,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum AttributeToken {
    #[regex("[a-zA-Z]+=", priority = 1)]
    Key,
    #[regex("\".+\"", priority = 2)]
    Value,
}
