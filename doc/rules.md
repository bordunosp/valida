# 📏 Valida Rules Reference

This document provides a comprehensive list of available validators grouped by category.

## 🧩 Common Validators

| Validator   | Description                                                             |
|-------------|-------------------------------------------------------------------------|
| `not_empty` | Ensures the field (e.g., list, map, string) contains at least one item. |
| `not_none`  | OptionSome(...)None                                                     |

## 🔢 Numeric Validators

| Validator           | Description                                                    |
|---------------------|----------------------------------------------------------------|
| `greater_than(min)` | Validates that the value is strictly greater than min.         |
| `less_than(max)`    | Validates that the value is strictly less than max.            |
| `min_value(max)`    | Value must be greater than or equal to min.                    |
| `max_value(min)`    | Value must be less than or equal to max.                       |
| `negative`          | Accepts only values strictly less than zero.                   |
| `negative_or_zero`  | Accepts negative values and zero.                              |
| `positive`          | Accepts only values strictly greater than zero.                |
| `positive_or_zero`  | Accepts positive values and zero.                              |
| `range(min, max)`   | Validates that the value lies inclusively between min and max. |

## 📚 Collection/Slice Validators

| Validator               | Description                                                  |
|-------------------------|--------------------------------------------------------------|
| `each_rule(rule)`       | Applies the provided rule to each element in the collection. |
| `exact_items(expected)` | Ensures that the collection contains exactly expected items. |
| `max_items(max)`        | Collection must contain no more than max items.              |
| `min_items(min)`        | Collection must contain at least min items.                  |

## 🔠 String Validators

| Validator                             | Description                                                                      |
|---------------------------------------|----------------------------------------------------------------------------------|
| `charset(allowed)`                    | Checks that all characters belong to the defined charset (e.g., ASCII, Latin-1). |
| `cidr`                                | Validates if a string is a valid IPv4 or IPv6 CIDR block.                        |
| `email`                               | Ensures string matches a general email pattern (user@domain).                    |
| `encoding_charset(charset)`           | Validates that the string is properly encoded using the specified charset.       |
| `hostname`                            | Validates domain/hostnames per RFC standards (e.g., RFC 1123).                   |
| `json`                                | Checks whether the string is syntactically valid JSON.                           |
| `lowercased`                          | Ensures the string contains only lowercase letters.                              |
| `mac_address`                         | Validates standard MAC address formats (00:1A:2B:...).                           |
| `max_length(max)`                     | Fails if the string exceeds max characters.                                      |
| `min_length(min)`                     | Fails if the string is shorter than min characters.                              |
| `no_suspicious_characters(blacklist)` | Rejects strings with potentially unsafe or forbidden characters.                 |
| `one_of(allowed)`                     | Validates that the string matches one of the allowed values.                     |
| `password_strength(level)`            | Enforces specific password complexity rules (e.g., mix of case, symbols).        |
| `regex_match(pattern)`                | Validates that the string matches the given regex pattern.                       |
| `trimmed`                             | Rejects strings with leading or trailing whitespace.                             |
| `uppercased`                          | Ensures the string contains only uppercase letters.                              |
| `url`                                 | Validates general URL format (https://...).                                      |
| `uuid_valid`                          | Checks whether the string is a valid UUID.                                       |
| `uuid_version(version)`               | Ensures UUID conforms to a specific version (e.g., v4).                          |
| `word_count(min, max)`                | Validates that the number of words lies within the specified range.              |