use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let default_input = "".to_string();
    let input = args.get(0).unwrap_or(&default_input);

    let result = if determine_xml(input) {
        "Valid"
    } else {
        "Invalid"
    };
    println!("{}", result);
}

//feel free to add other classes/methods if you want
fn determine_xml(input: &String) -> bool {

    if input.len() == 0 {
        return true
    }

    // create a stack that keeps track any opening tags
    let mut stack = Vec::new();

    let mut i = 0;
    // saving input to chars vector
    let chars: Vec<char> = input.chars().collect();

    if chars[chars.len() - 1] != '>' {
        return false
    }

    while i < chars.len() {
        if chars[i] == '<' {
            if i + 1 < chars.len() && chars[i + 1] == '/' {
                // check if it's a closing tag(right side)
                // Closing tag
                let end = input[i + 2..].find('>').unwrap_or(chars.len() - i - 2) + i + 2;
                let tag_name = &input[i + 2..end];
                if stack.pop() != Some(tag_name.to_string()) {
                    // if it doesn't match, it's not valid, it must be the same as opening tag
                    return false;
                }
                i = end + 1;
            } else {
                // check if it's a opening tag(left side)
                // Opening tag
                let end = input[i + 1..].find('>').unwrap_or(chars.len() - i - 1) + i + 1;
                let tag_content = &input[i + 1..end];
                let space_pos = tag_content.find(' ').unwrap_or(tag_content.len());
                let tag_name = &tag_content[..space_pos];

                if tag_content.ends_with("/") {
                    // Self-closing tag, skip it
                    i = end + 1;
                } else {
                    // Normal opening tag
                    stack.push(tag_name.to_string());
                    i = end + 1;
                }
            }
        } else {
            // skip the middle part
            i += 1;
        }
    }

    return stack.is_empty()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::determine_xml;

    // You can use here to test, feel free to modify/add the test cases here.
    // You can run tests with `cargo test`.
    // You can also use other ways to test if you want.

    #[test_case("<Design><Code>hello world</Code></Design>", true ; "normal case")]
    #[test_case("<Design><Code></Code></Design>", true ; "normal case no content")]
    #[test_case("<Design><Code>   </Code></Design>", true ; "normal case with whitespace content")]

    #[test_case("<Design><Code>hello world</Code></Design><People>", false ; "no closing tag")]
    #[test_case("<Design><Code>hello world</Code></Design", false ; "missing right")]

    #[test_case("<Design>Code>hello world</Code></Design>", false ; "missing left")]

    #[test_case("<Design><Code>hello world</Code></Design></People>", false ; "no opening tag")]
    #[test_case("<Design><Code>hello world</Code> <People></People></Design>", true ; "extra closing tag")]

    #[test_case("<People><Design><Code>hello world</People></Code></Design>", false ; "non-corresponding tags")]
    #[test_case("<People age=”1”>hello world</People>", false ; "attribute is not supported")]
    #[test_case("<People>hello world</People age=”1”>", false ; "attribute not supported in the end of the tag")]
    #[test_case("<People><Design><Code>hello world</Design></Code></People>", false ; "incorrect nesting order")]

    #[test_case("<Books><Book><Title>Book 1</Title></Book><Book><Title>Book 2</Title></Book></Books>", true ; "multiple books")]
    #[test_case("<Books><Book><Title>Book 1</Title></Book><Book><Title>Book 2</Title></Book>", false ; "unclosed Books tag")]

    #[test_case("<Tags><Tag/><Tag/><Tag/></Tags>", true ; "self-closing tags")]
    #[test_case("<Tags><Tag/><Tag></Tags>", false ; "mismatched self-closing tag")]

    #[test_case("<Parent><Child></Child><Child></Child></Parent>", true ; "multiple children")]
    #[test_case("<Parent><Child></Parent>", false ; "child outside parent")]



    fn check_determine_xml(input: &'static str, expected: bool) {
        let input = input.to_string();
        assert_eq!(expected, determine_xml(&input));
    }
}
