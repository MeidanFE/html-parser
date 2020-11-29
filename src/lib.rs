pub mod dom;
mod html_parse;

use html_parse::HtmlParser;

fn main() {
    let mut hp = HtmlParser::new("<a>12345</a>");
    println!("{:?}", hp.parse_nodes());
}
