use crate::core::field_builder::main::FieldBuilder;
use crate::core::rules::str_ref::StrAsRef;
use crate::core::rules::string::charset::Charset;
use crate::core::rules::string::cidr::Cidr;
use crate::core::rules::string::email::Email;
use crate::core::rules::string::encoding_charset::EncodingCharset;
use crate::core::rules::string::hostname::Hostname;
use crate::core::rules::string::json::Json;
use crate::core::rules::string::lowercased::Lowercased;
use crate::core::rules::string::mac_address::MacAddress;
use crate::core::rules::string::max_length::MaxLength;
use crate::core::rules::string::min_length::MinLength;
use crate::core::rules::string::no_suspicious_characters::NoSuspiciousCharacters;
use crate::core::rules::string::one_of::OneOf;
use crate::core::rules::string::password_strength::{PasswordStrength, StrengthLevel};
use crate::core::rules::string::regex_match::RegexMatch;
use crate::core::rules::string::trimmed::Trimmed;
use crate::core::rules::string::uppercased::Uppercased;
use crate::core::rules::string::url::UrlValid;
use crate::core::rules::string::uuid_valid::UuidValid;
use crate::core::rules::string::uuid_version::{UuidVersion, UuidVersionValidator};
use crate::core::rules::string::word_count::WordCount;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;

impl<'a, T, V, E> FieldBuilder<'a, T, V, E>
where
    V: StrAsRef + Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn charset(mut self, allowed: fn(char) -> bool) -> Self {
        self.rules.rules.push(Box::new(Charset { allowed }));
        self
    }

    pub fn cidr(mut self) -> Self {
        self.rules.rules.push(Box::new(Cidr {}));
        self
    }

    pub fn email(mut self) -> Self {
        self.rules.rules.push(Box::new(Email {}));
        self
    }

    pub fn encoding_charset(mut self, charset: &'static str) -> Self {
        self.rules.rules.push(Box::new(EncodingCharset { charset }));
        self
    }

    pub fn hostname(mut self) -> Self {
        self.rules.rules.push(Box::new(Hostname {}));
        self
    }

    pub fn json(mut self) -> Self {
        self.rules.rules.push(Box::new(Json {}));
        self
    }

    pub fn lowercased(mut self) -> Self {
        self.rules.rules.push(Box::new(Lowercased {}));
        self
    }

    pub fn mac_address(mut self) -> Self {
        self.rules.rules.push(Box::new(MacAddress {}));
        self
    }

    pub fn max_length(mut self, max: usize) -> Self {
        self.rules.rules.push(Box::new(MaxLength { max }));
        self
    }

    pub fn min_length(mut self, min: usize) -> Self {
        self.rules.rules.push(Box::new(MinLength { min }));
        self
    }

    pub fn no_suspicious_characters(mut self, blacklist: &'static [char]) -> Self {
        self.rules
            .rules
            .push(Box::new(NoSuspiciousCharacters { blacklist }));
        self
    }

    pub fn one_of(mut self, allowed: HashSet<String>) -> Self {
        self.rules.rules.push(Box::new(OneOf { allowed }));
        self
    }

    pub fn password_strength(mut self, level: StrengthLevel) -> Self {
        self.rules.rules.push(Box::new(PasswordStrength { level }));
        self
    }

    pub fn regex_match(mut self, pattern: Regex) -> Self {
        self.rules.rules.push(Box::new(RegexMatch { pattern }));
        self
    }

    pub fn trimmed(mut self) -> Self {
        self.rules.rules.push(Box::new(Trimmed {}));
        self
    }

    pub fn uppercased(mut self) -> Self {
        self.rules.rules.push(Box::new(Uppercased {}));
        self
    }

    pub fn url(mut self) -> Self {
        self.rules.rules.push(Box::new(UrlValid {}));
        self
    }

    pub fn uuid(mut self) -> Self {
        self.rules.rules.push(Box::new(UuidValid {}));
        self
    }

    pub fn uuid_version(mut self, version: UuidVersion) -> Self {
        self.rules
            .rules
            .push(Box::new(UuidVersionValidator { version }));
        self
    }

    pub fn word_count(mut self, min: Option<usize>, max: Option<usize>) -> Self {
        self.rules.rules.push(Box::new(WordCount { min, max }));
        self
    }
}
