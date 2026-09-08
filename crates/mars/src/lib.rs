//! Template crate for the `mars` Rust base.
//!
//! Replace this module with real code. It exists to prove the whole gate works:
//! documented public API, a doctest, a unit test, and a property test.

pub mod error;

pub use crate::error::{Error, Result};

/// A validated greeting for a single named recipient.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Greeting {
    name: String,
}

impl Greeting {
    /// Longest name accepted by [`Greeting::new`], in bytes.
    pub const MAX_NAME_LEN: usize = 64;

    /// Builds a greeting from `name`, trimming surrounding whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`Error::BlankName`] if `name` is empty after trimming, and
    /// [`Error::NameTooLong`] if it exceeds [`Greeting::MAX_NAME_LEN`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use mars::Greeting;
    ///
    /// let greeting = Greeting::new("  Ada  ")?;
    /// assert_eq!(greeting.render(), "Hello, Ada!");
    /// assert!(Greeting::new("   ").is_err());
    /// # Ok::<(), mars::Error>(())
    /// ```
    pub fn new(name: &str) -> Result<Self> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::BlankName);
        }
        if name.len() > Self::MAX_NAME_LEN {
            return Err(Error::NameTooLong {
                len: name.len(),
                max: Self::MAX_NAME_LEN,
            });
        }
        Ok(Self {
            name: name.to_owned(),
        })
    }

    /// The normalized name this greeting addresses.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Renders the greeting as a display string.
    #[must_use]
    pub fn render(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn rejects_blank_and_oversized_names() {
        assert!(matches!(Greeting::new("\t \n"), Err(Error::BlankName)));

        let long = "a".repeat(Greeting::MAX_NAME_LEN + 1);
        assert!(matches!(
            Greeting::new(&long),
            Err(Error::NameTooLong { .. })
        ));
    }

    proptest! {
        /// Any accepted name survives the trip through `new` unchanged once trimmed.
        ///
        /// Generated from printable ASCII so one char is one byte, matching how
        /// `MAX_NAME_LEN` is measured.
        #[test]
        fn name_roundtrips_through_greeting(raw in "[!-~]{1,64}") {
            let greeting = Greeting::new(&raw)?;
            prop_assert_eq!(greeting.name(), raw.trim());
            prop_assert!(greeting.render().contains(greeting.name()));
        }
    }
}
