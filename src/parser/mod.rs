use color_eyre::eyre::Result;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::collections::HashMap;

macro_rules! unexpected_parser_syntax {
    ($pair:expr) => {
        unimplemented!(
            "unexpected parser rule: {:#?}\n\n {:#?}",
            $pair.as_rule(),
            $pair
        );
    };
}

#[derive(Parser)]
#[grammar = "parser/grammar/yaml.pest"]
struct YAMLParser;

fn query_array_from_pair(pair: Pair<Rule>) -> Vec<String> {
    pair.into_inner()
        .map(|p| p.into_inner().next().unwrap().as_str().to_string())
        .collect()
}

fn query_array_multiline_from_pair(pair: Pair<Rule>) -> Vec<String> {
    pair.into_inner().map(|p| p.as_str().to_string()).collect()
}

fn query_string_from_pair(pair: Pair<Rule>) -> String {
    pair.into_inner().as_str().to_string()
}

fn query_from_pair(pair: Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::string => query_string_from_pair(pair),
        Rule::number => pair.as_str().to_string(),
        Rule::boolean => pair.as_str().to_string(),
        Rule::array => format!("{:?}", query_array_from_pair(pair)),
        Rule::array_multiline => format!("{:?}", query_array_multiline_from_pair(pair)),
        _ => unexpected_parser_syntax!(pair),
    }
}

fn statement_from_pairs(pairs: Pair<Rule>) -> HashMap<String, String> {
    let mut properties: HashMap<String, String> = HashMap::new();
    let mut current_key = String::new();

    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::pair_nested => {
                // TODO: nested...
            }
            Rule::string_key => {
                current_key = pair.as_str().to_string();
                properties
                    .entry(current_key.clone())
                    .or_default()
                    .to_string();
            }
            Rule::string | Rule::number | Rule::boolean | Rule::array | Rule::array_multiline => {
                properties.insert(current_key.clone(), query_from_pair(pair));
            }
            Rule::EOI => (),
            _ => unexpected_parser_syntax!(pair),
        }
    }
    properties
}

fn mapping_from_pairs(pairs: Pairs<Rule>) -> HashMap<String, String> {
    let mut results = Vec::<HashMap<String, String>>::with_capacity(1);
    for pair in pairs {
        match pair.as_rule() {
            Rule::mapping => results.push(statement_from_pairs(pair)),
            _ => unexpected_parser_syntax!(pair),
        }
    }
    results[0].clone()
}

pub fn parse(file: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    match YAMLParser::parse(Rule::mapping, file) {
        Ok(a) => Ok(mapping_from_pairs(a)),
        Err(mut error) => {
            error = error.renamed_rules(|rule| format!("{:?}", rule));
            panic!("Mapping parse error\n{}", error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r###"
            doe: "a deer, a female deer"
            ray: "a drop of golden sun"
            pi: 3.14159
            xmas: true
            french-hens: 3
            array-test: ["DFASf", "2222"]
            calling-birds:
                - huey
                - dewey
                - louie
                - fred
            test1: false
            hello: "hello"
            birds:
                - huey-2
                - dewey-2
        "###;
        let actual = parse(input).unwrap();

        assert!(actual.contains_key("doe"));
        assert_eq!(
            actual.get("doe"),
            Some(&"a deer, a female deer".to_string())
        );

        assert!(actual.contains_key("ray"));
        assert_eq!(actual.get("ray"), Some(&"a drop of golden sun".to_string()));

        assert!(actual.contains_key("pi"));
        assert_eq!(actual.get("pi"), Some(&"3.14159".to_string()));

        assert!(actual.contains_key("xmas"));
        assert_eq!(actual.get("xmas"), Some(&"true".to_string()));

        assert!(actual.contains_key("french-hens"));
        assert_eq!(actual.get("xmas"), Some(&"true".to_string()));

        assert!(actual.contains_key("array-test"));
        assert_eq!(
            actual.get("array-test"),
            Some(&"[\"DFASf\", \"2222\"]".to_string())
        );

        assert!(actual.contains_key("calling-birds"));
        assert_eq!(
            actual.get("calling-birds"),
            Some(&"[\"huey\", \"dewey\", \"louie\", \"fred\"]".to_string())
        );

        assert!(actual.contains_key("test1"));
        assert_eq!(actual.get("test1"), Some(&"false".to_string()));

        assert!(actual.contains_key("hello"));
        assert_eq!(actual.get("hello"), Some(&"hello".to_string()));

        assert!(actual.contains_key("birds"));
        assert_eq!(
            actual.get("birds"),
            Some(&"[\"huey-2\", \"dewey-2\"]".to_string())
        );
    }
    #[test]
    #[should_panic]
    fn test_parse_error() {
        let input = r###"
            doe: a deer, a female deer
        "###;
        parse(input).unwrap();
    }
}
