use super::*;
use u8char::u8char;

use pretty_assertions::assert_eq;

#[test]
fn character_categories() {
    // This is a very non-exhaustive and mostly arbitrary set of characters
    // to test just as a signal that the property lookup code is generally
    // working. `unicode_test_table` is a more thorough test that covers
    // both individual character categorization and the segmentation
    // state machine.

    use crate::GCBProperty::*;
    use std::assert_eq; // the standard one is better than "pretty" for this test
    fn prop(c: char) -> crate::GCBProperty {
        let c = u8char::from_char(c);
        crate::CharProperties::for_u8char(c).gcb_property()
    }

    assert_eq!(prop(' '), None);
    assert_eq!(prop('\r'), CR);
    assert_eq!(prop('\n'), LF);
    assert_eq!(prop('\t'), Control);
    assert_eq!(prop('\u{200D}'), ZWJ);
    assert_eq!(prop('\u{1F1E6}'), RegionalIndicator);
    assert_eq!(prop('\u{1F9D1}'), ExtendedPictographic);
    assert_eq!(prop('\u{1F33E}'), ExtendedPictographic);
    assert_eq!(prop('\u{0C41}'), SpacingMark);
}

#[test]
fn crlf() {
    use State::*;
    let got: Vec<_> = transitions(&[
        CharProperties::None,
        CharProperties::CR,
        CharProperties::LF,
        CharProperties::None,
    ])
    .collect();
    assert_eq!(
        got,
        &[
            (true, CharProperties::None, Base),
            (true, CharProperties::CR, Base),
            (false, CharProperties::LF, Base),
            (true, CharProperties::None, Base)
        ]
    );
}

#[test]
fn emoji_flags() {
    use State::*;
    let got: Vec<_> = transitions(&[
        CharProperties::None,
        CharProperties::RegionalIndicator,
        CharProperties::None,
        CharProperties::RegionalIndicator,
        CharProperties::RegionalIndicator,
        CharProperties::None,
        CharProperties::RegionalIndicator,
        CharProperties::RegionalIndicator,
        CharProperties::RegionalIndicator,
        CharProperties::None,
        CharProperties::RegionalIndicator,
        CharProperties::RegionalIndicator,
        CharProperties::RegionalIndicator,
        CharProperties::RegionalIndicator,
        CharProperties::None,
    ])
    .collect();
    assert_eq!(
        got,
        &[
            (true, CharProperties::None, Base),
            (true, CharProperties::RegionalIndicator, AwaitEmojiFlag),
            (true, CharProperties::None, Base),
            (true, CharProperties::RegionalIndicator, AwaitEmojiFlag),
            (false, CharProperties::RegionalIndicator, Base),
            (true, CharProperties::None, Base),
            (true, CharProperties::RegionalIndicator, AwaitEmojiFlag),
            (false, CharProperties::RegionalIndicator, Base),
            (true, CharProperties::RegionalIndicator, AwaitEmojiFlag),
            (true, CharProperties::None, Base),
            (true, CharProperties::RegionalIndicator, AwaitEmojiFlag),
            (false, CharProperties::RegionalIndicator, Base),
            (true, CharProperties::RegionalIndicator, AwaitEmojiFlag),
            (false, CharProperties::RegionalIndicator, Base),
            (true, CharProperties::None, Base),
        ]
    );
}

#[test]
fn unicode_test_table() {
    let mut failures = 0;
    for test in crate::properties::test_table::UNICODE_GRAPHEME_CLUSTER_TESTS {
        let input = str::from_utf8(test.input).expect("invalid UTF-8 in test input");
        let mut remain = input;
        let mut state = State::Base;
        let mut prev: CharProperties = CharProperties::None;
        let mut got: Vec<Box<[u8]>> = Vec::new();
        let mut current: Vec<u8> = Vec::new();
        loop {
            let (Some(next), rest) = u8char::from_string_prefix(remain) else {
                break;
            };
            let next_props = crate::CharProperties::for_u8char(next);
            let (boundary, next_state) = state.transition(prev, next_props);
            if boundary {
                if !current.is_empty() {
                    let boxed = current.clone().into_boxed_slice();
                    got.push(boxed);
                    current.clear();
                }
            }
            current.extend_from_slice(next.as_bytes());
            remain = rest;
            prev = next_props;
            state = next_state;
        }
        if !current.is_empty() {
            let boxed = current.clone().into_boxed_slice();
            got.push(boxed);
            current.clear();
        }
        if !result_matches(&got, test.expected) {
            println!("- test failed: {}", test.desc);
            println!("  input: {:x?}", test.input);
            println!("  got:   {:x?}", got);
            println!("  want:  {:x?}", test.expected);
            failures += 1;
        }
    }
    if failures != 0 {
        panic!("{failures} tests failed");
    }

    fn result_matches(got: &Vec<Box<[u8]>>, want: &[&[u8]]) -> bool {
        if got.len() != want.len() {
            return false;
        }
        for (got, want) in got.iter().zip(want.iter().copied()) {
            if got.len() != want.len() {
                return false;
            }
            for (got, want) in got.iter().zip(want) {
                if got != want {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn emoji_extend() {
    use State::*;
    let got: Vec<_> = transitions(&[
        CharProperties::None,
        //
        CharProperties::ExtendedPictographic,
        CharProperties::None,
        //
        CharProperties::ExtendedPictographic,
        CharProperties::ExtendedPictographic,
        CharProperties::None,
        //
        CharProperties::ExtendedPictographic,
        CharProperties::ZWJ,
        CharProperties::ExtendedPictographic,
        CharProperties::None,
        //
        CharProperties::ExtendedPictographic,
        CharProperties::Extend,
        CharProperties::ExtendedPictographic,
        CharProperties::None,
        //
        CharProperties::ExtendedPictographic,
        CharProperties::Extend,
        CharProperties::ZWJ,
        CharProperties::ExtendedPictographic,
        CharProperties::None,
        //
        CharProperties::ExtendedPictographic,
        CharProperties::Extend,
        CharProperties::Extend,
        CharProperties::ZWJ,
        CharProperties::ExtendedPictographic,
        CharProperties::None,
        //
        CharProperties::ExtendedPictographic,
        CharProperties::Extend,
        CharProperties::Extend,
        CharProperties::ZWJ,
        CharProperties::Extend,
        CharProperties::ExtendedPictographic,
        CharProperties::None,
    ])
    .collect();
    assert_eq!(
        got,
        &[
            (true, CharProperties::None, Base),
            //
            (true, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (true, CharProperties::None, Base),
            //
            (true, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (true, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (true, CharProperties::None, Base),
            //
            (true, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (false, CharProperties::ZWJ, GB11AfterZWJ),
            (false, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (true, CharProperties::None, Base),
            //
            (true, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (false, CharProperties::Extend, GB11BeforeZWJ),
            (true, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (true, CharProperties::None, Base),
            //
            (true, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (false, CharProperties::Extend, GB11BeforeZWJ),
            (false, CharProperties::ZWJ, GB11AfterZWJ),
            (false, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (true, CharProperties::None, Base),
            //
            (true, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (false, CharProperties::Extend, GB11BeforeZWJ),
            (false, CharProperties::Extend, GB11BeforeZWJ),
            (false, CharProperties::ZWJ, GB11AfterZWJ),
            (false, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (true, CharProperties::None, Base),
            //
            (true, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (false, CharProperties::Extend, GB11BeforeZWJ),
            (false, CharProperties::Extend, GB11BeforeZWJ),
            (false, CharProperties::ZWJ, GB11AfterZWJ),
            (false, CharProperties::Extend, Base),
            (true, CharProperties::ExtendedPictographic, GB11BeforeZWJ),
            (true, CharProperties::None, Base),
        ]
    );
}

fn transitions(
    cats: &[CharProperties],
) -> impl Iterator<Item = (bool, CharProperties, State)> + use<'_> {
    struct Iter<'a> {
        remain: &'a [CharProperties],
        state: State,
        prev: CharProperties,
    }
    impl<'a> Iterator for Iter<'a> {
        type Item = (bool, CharProperties, State);

        fn next(&mut self) -> Option<Self::Item> {
            let Some((next, remain)) = self.remain.split_first() else {
                return None;
            };
            let prev = self.prev;
            let next = *next;
            let (split, next_state) = self.state.transition(prev, next);
            self.remain = remain;
            self.state = next_state;
            self.prev = next;
            Some((split, next, next_state))
        }
    }

    Iter {
        remain: cats,
        state: State::Base,
        prev: CharProperties::None,
    }
}
