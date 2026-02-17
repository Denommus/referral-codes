use std::collections::HashSet;

use rand::{RngExt, distr::Distribution, seq::IteratorRandom};
use thiserror::Error;

/// Error type for referral code generation operations.
///
/// This enum represents errors that can occur when generating referral codes,
/// such as when the configuration is not feasible for the requested number of codes.
#[derive(Error, Debug)]
pub enum ReferralCodeError {
    /// Indicates that the configuration cannot generate the requested number of unique codes.
    ///
    /// This error occurs when the combination of charset size and pattern length
    /// does not provide enough possible combinations to generate the requested count
    /// of unique codes.
    #[error("Non feasible configuration")]
    NonFeasibleConfig,
}

/// Character set used for generating referral codes.
///
/// Defines the set of characters that can be used when generating codes.
/// The charset determines the available character pool for random selection.
#[derive(Clone)]
pub enum Charset {
    /// Numeric characters only: 0-9 (10 characters).
    Numeric,
    /// Alphabetic characters only: a-z and A-Z (52 characters).
    Alphabetic,
    /// Alphanumeric characters: a-z, A-Z, and 0-9 (62 characters).
    Alphanumeric,
    /// Custom character set specified as a string.
    ///
    /// The string can contain any characters that should be used for code generation.
    /// Characters will be selected randomly from this string.
    Custom(String),
}

impl Charset {
    /// Returns the number of characters in this charset.
    ///
    /// # Returns
    ///
    /// The size of the character set:
    /// - `Numeric`: 10
    /// - `Alphabetic`: 52
    /// - `Alphanumeric`: 62
    /// - `Custom(s)`: length of the custom string
    ///
    /// # Examples
    ///
    /// ```
    /// use referral_codes::Charset;
    ///
    /// assert_eq!(Charset::Numeric.len(), 10);
    /// assert_eq!(Charset::Alphanumeric.len(), 62);
    /// assert_eq!(Charset::Custom("ABC".to_string()).len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        match self {
            Self::Numeric => 10,
            Self::Alphabetic => 52,
            Self::Alphanumeric => 62,
            Self::Custom(s) => s.len(),
        }
    }
}

impl Distribution<char> for Charset {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> char {
        match self {
            Self::Numeric => "0123456789".chars().choose(rng).unwrap(),
            Self::Alphabetic => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .choose(rng)
                .unwrap(),
            Self::Alphanumeric => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                .chars()
                .choose(rng)
                .unwrap(),
            Self::Custom(s) => s.chars().choose(rng).unwrap(),
        }
    }
}

/// Pattern specification for referral code generation.
///
/// Defines the structure and length of generated codes. Patterns can be specified
/// as a simple length or as a template string with placeholders.
#[derive(Clone)]
pub enum Pattern {
    /// Generate a code of the specified length using all random characters.
    ///
    /// The length determines how many characters will be randomly selected
    /// from the charset.
    Length(usize),
    /// Generate a code following a custom pattern string.
    ///
    /// The pattern string can contain:
    /// - `#` characters: replaced with random characters from the charset
    /// - Any other characters: included literally in the generated code
    ///
    /// # Examples
    ///
    /// - `"ABC###"` generates codes like "ABC123"
    /// - `"###-###"` generates codes like "123-123"
    Pattern(String),
}

impl Pattern {
    /// Returns the number of random characters that will be generated.
    ///
    /// For `Length(n)`, this returns `n`.
    /// For `Pattern(s)`, this returns the count of `#` characters in the pattern string.
    ///
    /// # Returns
    ///
    /// The number of positions that will be filled with random characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use referral_codes::Pattern;
    ///
    /// assert_eq!(Pattern::Length(8).size(), 8);
    /// assert_eq!(Pattern::Pattern("ABC###".to_string()).size(), 3);
    /// assert_eq!(Pattern::Pattern("###-###".to_string()).size(), 6);
    /// ```
    pub fn size(&self) -> usize {
        match self {
            Self::Length(u) => *u,
            Self::Pattern(s) => s.chars().filter(|p| p == &'#').count(),
        }
    }

    /// Returns the pattern string representation.
    ///
    /// For `Length(n)`, this returns a string of `n` `#` characters.
    /// For `Pattern(s)`, this returns the pattern string as-is.
    ///
    /// # Returns
    ///
    /// A string representation of the pattern where `#` represents random character positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use referral_codes::Pattern;
    ///
    /// assert_eq!(Pattern::Length(3).pattern(), "###");
    /// assert_eq!(Pattern::Pattern("ABC###".to_string()).pattern(), "ABC###");
    /// ```
    pub fn pattern(&self) -> String {
        match self {
            Self::Length(size) => std::iter::repeat('#').take(*size).collect::<String>(),
            Self::Pattern(s) => s.clone(),
        }
    }
}

/// Configuration for generating referral codes.
///
/// Specifies all parameters needed to generate one or more unique referral codes.
#[derive(Clone)]
pub struct Config {
    /// The pattern that defines the structure and length of generated codes.
    pub pattern: Pattern,
    /// The number of unique codes to generate.
    pub count: usize,
    /// The character set to use when generating random characters.
    pub charset: Charset,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            pattern: Pattern::Length(8),
            count: 1,
            charset: Charset::Alphanumeric,
        }
    }
}

/// Generates a single referral code according to the given configuration.
///
/// This function generates one code by replacing `#` characters in the pattern
/// with random characters from the specified charset, while preserving any
/// literal characters in the pattern.
///
/// # Arguments
///
/// * `config` - Configuration specifying the pattern, charset, and other parameters
///
/// # Returns
///
/// A single referral code string generated according to the configuration.
///
/// # Examples
///
/// ```
/// use referral_codes::{Config, Pattern, Charset};
///
/// let config = Config {
///     pattern: Pattern::Length(8),
///     count: 1,
///     charset: Charset::Alphanumeric,
/// };
///
/// let code = referral_codes::generate_one(&config);
/// assert_eq!(code.len(), 8);
/// ```
pub fn generate_one(config: &Config) -> String {
    let mut rng = rand::rng();

    let mut result = "".to_string();

    for p in config.pattern.pattern().chars() {
        if p == '#' {
            result.push(rng.sample(&config.charset));
        } else {
            result.push(p)
        }
    }

    result
}

fn is_feasible(config: &Config) -> bool {
    config
        .charset
        .len()
        .pow(u32::try_from(config.pattern.size()).unwrap())
        >= config.count
}

/// Generates multiple unique referral codes according to the given configuration.
///
/// This function generates the specified number of unique codes by repeatedly
/// calling `generate_one` until enough unique codes have been generated.
///
/// # Arguments
///
/// * `config` - Configuration specifying the pattern, charset, and count of codes to generate
///
/// # Returns
///
/// * `Ok(Vec<String>)` - A vector of unique referral codes
/// * `Err(ReferralCodeError::NonFeasibleConfig)` - If the configuration cannot generate
///   the requested number of unique codes (i.e., the charset size raised to the power
///   of the pattern size is less than the requested count)
///
/// # Examples
///
/// ```
/// use referral_codes::{Config, Pattern, Charset};
///
/// let config = Config {
///     pattern: Pattern::Length(8),
///     count: 5,
///     charset: Charset::Alphanumeric,
/// };
///
/// let codes = referral_codes::generate(&config).unwrap();
/// assert_eq!(codes.len(), 5);
/// assert_eq!(codes[0].len(), 8);
/// ```
///
/// # Errors
///
/// Returns `ReferralCodeError::NonFeasibleConfig` if the configuration cannot
/// generate the requested number of unique codes. For example, requesting 100
/// unique codes with a pattern size of 1 and a charset of 62 characters
/// (which only provides 62 possible combinations).
pub fn generate(config: &Config) -> Result<Vec<String>, ReferralCodeError> {
    if !is_feasible(config) {
        return Err(ReferralCodeError::NonFeasibleConfig);
    }

    let mut codes = HashSet::new();

    while codes.len() < config.count {
        codes.insert(generate_one(config));
    }

    Ok(codes.into_iter().collect())
}

#[test]
fn test_generate() {
    let config = Config {
        charset: Charset::Alphanumeric,
        count: 3,
        pattern: Pattern::Length(8),
    };

    let result = generate(&config).unwrap();

    assert_eq!(3, result.len());
}

#[test]
fn test_fail_generate() {
    let config = Config {
        charset: Charset::Alphanumeric,
        count: 100,
        pattern: Pattern::Length(1),
    };

    let result = generate(&config);

    assert!(result.is_err())
}
