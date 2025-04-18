//! Decodes a base91 string
//! Performs error handling and returns a string
//! Call base91_decoder.crack to use. It returns option<String> and check with
//! `result.is_some()` to see if it returned okay.

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

/// The Base91 decoder, call:
/// `let base91_decoder = Decoder::<Base91Decoder>::new()` to create a new instance
/// And then call:
/// `result = base91_decoder.crack(input)` to decode a base91 string
/// The struct generated by new() comes from interface.rs
/// ```
/// use ciphey::decoders::base91_decoder::{Base91Decoder};
/// use ciphey::decoders::interface::{Crack, Decoder};
/// use ciphey::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decode_base91 = Decoder::<Base91Decoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decode_base91.crack("TPwJh>Io2Tv!lE", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap()[0], "hello world");
/// ```
pub struct Base91Decoder;

impl Crack for Decoder<Base91Decoder> {
    fn new() -> Decoder<Base91Decoder> {
        Decoder {
            name: "Base91",
            description: "basE91 is an advanced method for encoding binary data as ASCII characters. It is similar to UUencode or base64, but is more efficient.",
            link: "https://base91.sourceforge.net/",
            tags: vec!["base91", "decoder", "base"],
            popularity: 0.3,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Base91 with text {:?}", text);
        let decoded_text = decode_base91_no_error_handling(text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode base91 because Base91Decoder::decode_base91_no_error_handling returned None");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode base91 because check_string_success returned false on string {}",
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
fn decode_base91_no_error_handling(text: &str) -> Option<String> {
    // Runs the code to decode base91
    // Doesn't perform error handling, call from_base91
    let decoded_text = base91::slice_decode(text.as_bytes());
    Some(String::from_utf8_lossy(&decoded_text).to_string())
}

#[cfg(test)]
mod tests {
    use super::Base91Decoder;
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
    fn successful_decoding() {
        let base91_decoder = Decoder::<Base91Decoder>::new();
        let result = base91_decoder.crack("TPwJh>Io2Tv!lE", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "hello world");
    }

    #[test]
    fn base91_decode_empty_string() {
        // Base91 returns an empty string, this is a valid base91 string
        // but returns False on check_string_success
        let base91_decoder = Decoder::<Base91Decoder>::new();
        let result = base91_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base91_decode_handles_panics() {
        let base91_decoder = Decoder::<Base91Decoder>::new();
        let result = base91_decoder
            .crack("😈", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base91_handle_panic_if_empty_string() {
        let base91_decoder = Decoder::<Base91Decoder>::new();
        let result = base91_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base91_work_if_string_not_base91() {
        // You can base91 decode a string that is not base91
        // This string decodes to:
        // ```.ée¢
        // (uÖ²```
        // https://gchq.github.io/CyberChef/#recipe=From_Base91('A-Za-z0-9%2B/%3D',true)&input=aGVsbG8gZ29vZCBkYXkh
        let base91_decoder = Decoder::<Base91Decoder>::new();
        let result = base91_decoder
            .crack("hello good day!", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_some());
    }

    #[test]
    fn base91_handle_panic_if_emoji() {
        let base91_decoder = Decoder::<Base91Decoder>::new();
        let result = base91_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }
}
