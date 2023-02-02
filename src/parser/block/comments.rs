use regex::Regex;

use parser::Block;
use parser::Block::Comment;

pub fn parse_comments(lines: &[&str]) -> Option<(Block, usize)> {
    lazy_static! {
        static ref COMMENT: Regex = Regex::new(r"<!--.+-->").unwrap();
    }

    if COMMENT.is_match(lines[0]) {
        return Some((Comment(lines[0].to_string()), 1));
    }

    return None;
}

#[cfg(test)]
mod test {
    use super::parse_comments;
    use parser::Block::Comment;

    #[test]
    fn finds_comments() {
        assert_eq!(
            parse_comments(&vec!["<!-- test -->"]).unwrap(),
            (Comment("<!-- test -->".to_string()), 1)
        );
    }
}
