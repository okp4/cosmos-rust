//! Transaction builder.

use super::Body;
use crate::Any;
use tendermint::block;

/// Transaction [`Body`] builder which simplifies incrementally assembling and
/// signing a transaction.
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// Transaction body in-progress.
    body: Body,
}

impl Builder {
    /// Create a new transaction builder in the default state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a message to the transaction.
    pub fn msg(&mut self, msg: impl Into<Any>) -> &mut Self {
        self.body.messages.push(msg.into());
        self
    }

    /// Add multiple messages to the transaction.
    pub fn msgs(&mut self, msgs: impl IntoIterator<Item = Any>) -> &mut Self {
        self.body.messages.extend(msgs);
        self
    }

    /// Set the transaction memo.
    // TODO(tarcieri): error if the memo is already set?
    pub fn memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.memo = memo.into();
        self
    }

    /// Set the timeout height.
    // TODO(tarcieri): error if the timeout height is already set?
    pub fn timeout_height(&mut self, height: impl Into<block::Height>) -> &mut Self {
        self.body.timeout_height = height.into();
        self
    }

    /// Add an extension option.
    pub fn extension_option(&mut self, option: impl Into<Any>) -> &mut Self {
        self.body.extension_options.push(option.into());
        self
    }

    /// Add a non-critical extension option.
    pub fn non_critical_extension_option(&mut self, option: impl Into<Any>) -> &mut Self {
        self.body.extension_options.push(option.into());
        self
    }
}

impl From<Builder> for Body {
    fn from(builder: Builder) -> Body {
        builder.body
    }
}

impl From<&Builder> for Body {
    fn from(builder: &Builder) -> Body {
        builder.body.clone()
    }
}
