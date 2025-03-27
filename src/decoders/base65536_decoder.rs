//! Decode a base65536 string
//! Performs error handling and returns a string
//! Call base65536_decoder.crack to use. It returns option<String> and check with
//! `result.is_some()` to see if it returned okay.

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

/// The base65536 decoder, call:
/// `let base65536_decoder = Decoder::<Base65536Decoder>::new()` to create a new instance
/// And then call:
/// `result = base65536_decoder.crack(input)` to decode a base65536 string
/// The struct generated by new() comes from interface.rs
/// ```
/// use ciphey::decoders::base65536_decoder::{Base65536Decoder};
/// use ciphey::decoders::interface::{Crack, Decoder};
/// use ciphey::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decode_base65536 = Decoder::<Base65536Decoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decode_base65536.crack("𒅓鹨𖡮𒀠啦ꍢ顡啫𓍱𓁡𠁴唬𓍪鱤啥𖥭𔐠𔕯ᔮ", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap()[0], "Sphinx of black quartz, judge my vow.");
/// ```
pub struct Base65536Decoder;

impl Crack for Decoder<Base65536Decoder> {
    fn new() -> Decoder<Base65536Decoder> {
        Decoder {
            name: "Base65536",
            description: "Base65536 is a binary encoding optimised for UTF-32-encoded text. Base65536 uses only \"safe\" Unicode code points - no unassigned code points, no whitespace, no control characters, etc.",
            link: "https://github.com/qntm/base65536",
            tags: vec!["base65536", "decoder", "base"],
            popularity: 0.1,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying base65536 with text {:?}", text);
        let decoded_text: Option<String> = decode_base65536_no_error_handling(text);

        trace!("Decoded text for base65536: {:?}", decoded_text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode base65536 because Base65536Decoder::decode_base65536_no_error_handling returned None");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode base65536 because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);

        results.update_checker(&checker_result);

        results
    }
    /// Gets all tags for this decoder
    fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }
    /// Gets the name for the current decoder
    fn get_name(&self) -> &str {
        self.name
    }
    /// Gets the description for the current decoder
    fn get_description(&self) -> &str {
        self.description
    }
    /// Gets the link for the current decoder
    fn get_link(&self) -> &str {
        self.link
    }
}

/// helper function
fn decode_base65536_no_error_handling(text: &str) -> Option<String> {
    // Runs the code to decode base65536
    // Doesn't perform error handling, call from_base65536
    if let Ok(decoded_text) = base65536::decode(text, false) {
        return Some(String::from_utf8_lossy(&decoded_text).to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::Base65536Decoder;
    use crate::{
        checkers::{
            athena::Athena,
            checker_type::{Check, Checker},
            CheckerTypes,
        },
        decoders::interface::{Crack, Decoder},
    };

    // helper for tests
    fn get_athena_checker() -> CheckerTypes {
        let athena_checker = Checker::<Athena>::new();
        CheckerTypes::CheckAthena(athena_checker)
    }

    #[test]
    fn base65536_decodes_successfully() {
        // This tests if Base65536 can decode Base65536 successfully
        let base65536_decoder = Decoder::<Base65536Decoder>::new();
        let result = base65536_decoder.crack("𒅓鹨𖡮𒀠啦ꍢ顡啫𓍱𓁡𠁴唬𓍪鱤啥𖥭𔐠𔕯ᔮ", &get_athena_checker());
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "Sphinx of black quartz, judge my vow."
        );
    }

    #[test]
    fn base65536_handles_panics() {
        // This tests if Base65536 can handle panics
        // It should return None
        let base65536_decoder = Decoder::<Base65536Decoder>::new();
        let result = base65536_decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base65536_handles_panic_if_empty_string() {
        // This tests if Base65536 can handle an empty string
        // It should return None
        let base65536_decoder = Decoder::<Base65536Decoder>::new();
        let result = base65536_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base65536_handles_panic_if_emoji() {
        // This tests if Base65536 can handle an emoji
        // It should return None
        let base65536_decoder = Decoder::<Base65536Decoder>::new();
        let result = base65536_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }
}
