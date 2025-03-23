//! Run a brainfuck program and return its output
//! Performs error handling and returns a string
//! Call brainfuck_interpreter.crack to use. It returns Option<String> and check with
//! `result.is_some()` to see if it returned okay.
use crate::checkers::CheckerTypes;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use brainfuck_exe::Brainfuck;
use log::{debug, trace};

/// The Brainfuck interpreter, call:
/// `let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new()` to create a new instance
/// And then call:
/// `result = brainfuck_interpreter.crack(input)` to interpret a Brainfuck program
/// The struct generated by new() comes from interface.rs
/// ```
/// use ciphey::decoders::brainfuck_interpreter::{BrainfuckInterpreter};
/// use ciphey::decoders::interface::{Crack, Decoder};
/// use ciphey::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = brainfuck_interpreter.crack(">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap()[0], "Hello, World!");
/// ```
pub struct BrainfuckInterpreter;

impl Crack for Decoder<BrainfuckInterpreter> {
    fn new() -> Decoder<BrainfuckInterpreter> {
        Decoder {
            name: "Brainfuck",
            description: "Brainfuck is an esoteric programming language created in 1993 by Swiss student Urban Müller. Designed to be extremely minimalistic, the language consists of only eight simple commands, a data pointer, and an instruction pointer.",
            link: "https://en.wikipedia.org/wiki/Brainfuck",
            tags: vec!["decoder", "brainfuck"],
            popularity: 0.6,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying brainfuck with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        // Guard against text that realistically won't be a Brainfuck program
        if text.contains(',') {
            return results;
        }
        if !text.ends_with('.') || text.matches('.').count() < 5 {
            return results;
        }
        if text.matches(|c| "+-<>[]".contains(c)).count() < 20 {
            return results;
        }

        let mut buf = vec![];
        match Brainfuck::new(text).with_output_ref(&mut buf).execute() {
            Ok(_) => {
                let decoded_text = String::from_utf8(buf).unwrap_or_default();
                let checker_result = checker.check(&decoded_text);
                results.unencrypted_text = Some(vec![decoded_text]);
                results.update_checker(&checker_result);

                results
            }
            Err(e) => {
                debug!("Failed to interpret Brainfuck because of error {:?}", e);
                results
            }
        }
    }
    /// Gets all tags for this decoder
    fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }
    /// Gets the name for the current decoder
    fn get_name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::BrainfuckInterpreter;
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
    fn brainfuck_successful_decoding_regular_string() {
        let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
        let result = brainfuck_interpreter.crack(
            ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]
            <++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.",
            &get_athena_checker(),
        );
        assert_eq!(result.unencrypted_text.unwrap()[0], "Hello, World!");
    }

    #[test]
    fn brainfuck_successful_decoding_long_string() {
        let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
        let result = brainfuck_interpreter.crack("+++++++++[>++++++++>++++++++++++>+++++++++++>++++>+++++>++++++++>++++++++++>++++++++++++<<<<<<<<-]
            >++++.>+++.+++.>++.<-----.>>----.>>+.>>++++.+++.++.<<<<<<.>>+.-.<<<+++.>>+++.>>-.<.>>-.>+++++++.>-----..+++++++++.<<<<.>>----.>.>.<<<<+.
            -.>>++++.>++++.<<<<<-..+++.>>.<<<++++++++.>.+++.>++++.>>>>-.<<<+.-.>>+.<<.<----.<---.+.>---.>.>>>>.<<<<<<-.++++++.>>.<+++.-------.<+.
            >++++.>.<----.>.>>>++++++++.+++.>---.<<<<<++++.+++++++.<+++.>>.<--------.---.<.>>+.", &get_athena_checker());
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "Lorem Ipsum! Oh, Happy Day! Hello World! I hope you have a lovely day!"
        );
    }

    #[test]
    fn brainfuck_successful_decoding_string_with_trash() {
        let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
        let result = brainfuck_interpreter.crack(">++++++++[<+++++++++>trash-]<.>++++[<+++++++>-]<+.+++++++..+lots+of+garbage.
            >>++++++[<+++++++>-]<++.--some--random----rubbish---here-.>++++++[<+++++++++>-]<+.<.+++.------.--------.
            >>>++++[<++++++++>-]<+.", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "Hello, World!");
    }

    #[test]
    fn brainfuck_fail_unmatched_bracket() {
        let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
        let result = brainfuck_interpreter
            .crack("[[]", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
        let result = brainfuck_interpreter
            .crack("[+]]", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn brainfuck_fail_empty_input() {
        // This tests if brainfuck_interpreter handles an empty string
        // It should return None
        let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
        let result = brainfuck_interpreter
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
        let result = brainfuck_interpreter
            .crack("aeiou", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn brainfuck_fail_no_enough_input() {
        // This tests if brainfuck_interpreter handles an empty string
        // It should return None
        let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
        let result = brainfuck_interpreter
            .crack(">+[+]..", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
        let result = brainfuck_interpreter
            .crack(
                ">a+e[i+o]u.a-bunch-of-trash-in-between.",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn brainfuck_fail_no_print() {
        let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
        let result = brainfuck_interpreter
            .crack("+-<>[]", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn brainfuck_fail_no_print_at_end() {
        let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
        let result = brainfuck_interpreter
            .crack("+++++++++++++++++.-", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn brainfuck_fail_read() {
        let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
        let result = brainfuck_interpreter
            .crack("+-<>,.[]", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn brainfuck_successful_wrapping() {
        let brainfuck_interpreter = Decoder::<BrainfuckInterpreter>::new();
        let result =
            brainfuck_interpreter.crack("+++++++++++[---]<-....>-....", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "ÿÿÿÿÿÿÿÿ");
    }
}
