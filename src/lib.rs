use std::collections::HashSet;

use rand::{RngExt, distr::Distribution, seq::IteratorRandom};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReferralCodeError {
    #[error("Non feasible configuration")]
    NonFeasibleConfig,
}

#[derive(Clone)]
pub enum Charset {
    Numeric,
    Alphabetic,
    Alphanumeric,
    Custom(String),
}

impl Charset {
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

#[derive(Clone)]
pub enum Pattern {
    Length(usize),
    Pattern(String),
}

impl Pattern {
    pub fn size(&self) -> usize {
        match self {
            Self::Length(u) => *u,
            Self::Pattern(s) => s.chars().filter(|p| p == &'#').count(),
        }
    }

    pub fn pattern(&self) -> String {
        match self {
            Self::Length(size) => std::iter::repeat('#').take(*size).collect::<String>(),
            Self::Pattern(s) => s.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub pattern: Pattern,
    pub count: usize,
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
