use parser::Block;
use parser::Block::Html;
use regex::Regex;

pub fn parse_html(lines: &[&str]) -> Option<(Block, usize)> {
    lazy_static! {
        static ref HTML: Regex = Regex::new(r"<!-- html -->([\s\S]*)<!-- html -->").unwrap();
        static ref HTML_START: Regex = Regex::new(r"<!-- html -->").unwrap();
    }

    let mut content = String::new();
    let mut line_number: usize = 0;
    let mut html_open = false;
    let html_close = false;

    for line in lines {
        if !html_open && HTML_START.is_match(line) {
            html_open = !html_open;
            content.push_str(line);
            content.push_str("\n");
            line_number += 1
        } else if !html_close && HTML_START.is_match(line) {
            content.push_str(line);
            line_number += 1;
            break;
        } else if html_open {
            content.push_str(line);
            content.push_str("\n");
            line_number += 1;
        } else {
            break;
        }
    }

    if line_number > 0 {
        return Some((Html(content), line_number));
    }

    return None;
}

#[cfg(test)]
mod test {
    use super::parse_html;
    use parser::Block::Html;

    #[test]
    fn find_html() {
        let lines = vec![
            "<!-- html -->",
            "<p>Something</p>",
            "<p>Something else</p>",
            "<!-- html -->",
        ];

        let html = lines.join("\n");

        assert_eq!(parse_html(&lines).unwrap(), (Html(html.clone()), 4));
    }
}
