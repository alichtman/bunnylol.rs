/// Twitter command handler  
/// Supports: tw, tw @[username], tw [search terms]
use crate::utils::bunnylol_command::BunnylolCommand;
use crate::utils::url_encoding::{build_path_url, build_search_url};

pub struct TwitterCommand;

impl TwitterCommand {
    fn construct_profile_url(profile: &str) -> String {
        build_path_url("https://twitter.com", profile)
    }

    fn construct_search_url(query: &str) -> String {
        build_search_url("https://twitter.com/search", "q", query)
    }
}

impl BunnylolCommand for TwitterCommand {
    const COMMAND: &'static str = "tw";

    fn process_args(args: &str) -> String {
        if args == Self::COMMAND {
            "https://twitter.com".to_string()
        } else {
            let query = Self::get_command_args(args);

            // Check if it looks like a Twitter profile
            if query.starts_with('@') {
                Self::construct_profile_url(&query[1..])
            } else {
                Self::construct_search_url(query)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter_command_base() {
        assert_eq!(TwitterCommand::process_args("tw"), "https://twitter.com");
    }

    #[test]
    fn test_twitter_command_profile() {
        assert_eq!(
            TwitterCommand::process_args("tw @fbOpenSource"),
            "https://twitter.com/fbOpenSource"
        );
    }

    #[test]
    fn test_twitter_command_search() {
        assert_eq!(
            TwitterCommand::process_args("tw hello world"),
            "https://twitter.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        assert_eq!(
            TwitterCommand::construct_profile_url("jsjoeio"),
            "https://twitter.com/jsjoeio"
        );
    }

    #[test]
    fn test_construct_twitter_search_url() {
        assert_eq!(
            TwitterCommand::construct_search_url("hello world"),
            "https://twitter.com/search?q=hello%20world"
        );
    }
}
