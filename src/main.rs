mod dom;
mod lexer;
use lexer::{dom_parser, print_node};

fn main() {
    print_node(
        &dom_parser(
            "<a href=\"localhost\"><i>test</i><img src=\"test\" onerror=\"alert(1)\"/></a>",
        ),
        0,
    );
}
