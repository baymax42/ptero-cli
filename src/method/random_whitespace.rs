//! # Description
//!
//! This method puts [ASCII_WHITESPACE](crate::method::random_whitespace::ASCII_WHITESPACE) between randomly selected two words.
//! If the duplicate whitespace is present the bit 1 is encoded, otherwise 0.
use std::error::Error;

use crate::{binary::Bit, context::{Context, ContextError}, decoder::Decoder, encoder::{Capacity, Encoder, EncoderResult}};

use log::{trace};
use rand::{thread_rng, Rng};

/// Character used as the random whitespace in the method.
pub const ASCII_WHITESPACE: char = ' ';

use super::Method;

/// Unit structure representing the random whitespace method.
/// Implements both [Encoder](crate::encoder::Encoder) and [Decoder](crate::decoder::Decoder) traits.
///
/// Accepts any [Context](crate::context::Context).
#[derive(Debug, PartialEq)]
pub struct RandomWhitespaceMethod;

impl Default for RandomWhitespaceMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomWhitespaceMethod {
    pub fn new() -> Self {
        RandomWhitespaceMethod {}
    }
}

impl Capacity for RandomWhitespaceMethod {
    fn bitrate(&self) -> usize {
        1
    }
}

impl<T> Encoder<T> for RandomWhitespaceMethod
where
    T: Context,
{
    fn partial_encode(
        &self,
        context: &mut T,
        data: &mut dyn Iterator<Item = Bit>,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                let mut rng = thread_rng();
                let text = context.get_current_text_mut()?;
                let position_determinant = rng.gen_range(0, &text.len());
                let mut position = text.find(' ').unwrap_or_else(|| text.len());
                for (index, character) in text.char_indices() {
                    if index > position_determinant {
                        break;
                    }
                    if character.is_whitespace() {
                        position = index;
                    }
                }
                trace!("Putting space at position {}", position);
                text.insert_str(position, &String::from(ASCII_WHITESPACE));
                EncoderResult::Success
            }
            None => EncoderResult::NoDataLeft,
            _ => {
                trace!("Skipping double whitespace");
                EncoderResult::Success
            }
        })
    }
}

impl<D> Decoder<D> for RandomWhitespaceMethod
where
    D: Context,
{
    fn partial_decode(&self, context: &D) -> Result<Vec<Bit>, ContextError> {
        let mut seen_whitespace = false;
        for character in context.get_current_text()?.chars() {
            let is_whitespace = character == ASCII_WHITESPACE;
            if seen_whitespace && is_whitespace {
                trace!("Found two consecutive '{}' between words", ASCII_WHITESPACE,);
                return Ok(vec![Bit(1)]);
            }
            seen_whitespace = is_whitespace;
        }
        trace!("No double whitespace found");
        Ok(vec![Bit(0)])
    }
}

impl<E, D> Method<E, D> for RandomWhitespaceMethod
where
    E: Context,
    D: Context,
{
    fn method_name(&self) -> String {
        "RandomWhitespaceMethod".to_string()
    }
}
