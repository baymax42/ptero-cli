use std::{error::Error, fmt};

use crate::{binary::Bit, context::Context};

/// Whitespace used to encode bits
pub const ASCII_ENCODING_WHITESPACE: char = ' ';

/// Possible results of data encoding
#[derive(Debug, Clone)]
pub enum EncoderResult {
    Success,
    NoDataLeft,
}

/// Base trait for all data encoders
pub trait Encoder {
    /// Method which encodes bits provided by `data` iterator into provided `line` string.
    /// It may change the line in process e.g. add some additional characters.
    ///
    /// # Arguments
    ///
    /// * `context` - context of the steganography method, can contain various needed info like pivot etc.
    /// * `data` - data iterator which return bit with each iteration
    /// * `line` - line string holder
    ///
    /// # Returns
    /// It returns whether the encoding was successful. See [EncoderResult](enum.EncoderResult.html) and [EncodingError](struct.EncodingError.html).
    ///
    fn encode(
        &mut self,
        context: &mut Context,
        data: &mut dyn Iterator<Item = Bit>,
        line: &mut String,
    ) -> Result<EncoderResult, Box<dyn Error>>;

    /// This method provides the amount of bits encoded per line by the encoder.
    fn rate(&self) -> u32;
}
/// Enum for data encoding errors types
#[derive(Debug, Clone)]
pub enum EncodingErrorKind {
    CapacityTooLow,
    NoWordsLeft,
}

/// Represents every encoding error
#[derive(Debug, Clone)]
pub struct EncodingError {
    kind: EncodingErrorKind,
}

impl EncodingError {
    /// Facade for creating [EncodingErrorKind::CapacityTooLow](enum.EncodingErrorKind.html)
    pub fn capacity_error() -> Self {
        EncodingError {
            kind: EncodingErrorKind::CapacityTooLow,
        }
    }
    /// Facade for creating [EncodingErrorKind::NoWordsLeft](enum.EncodingErrorKind.html)
    pub fn no_words_error() -> Self {
        EncodingError {
            kind: EncodingErrorKind::NoWordsLeft,
        }
    }
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            EncodingErrorKind::CapacityTooLow => write!(f, "Exceeded cover text capacity"),
            EncodingErrorKind::NoWordsLeft => write!(
                f,
                "No extra words found in cover text when tried to encode a bit"
            ),
        }
    }
}

impl Error for EncodingError {}
