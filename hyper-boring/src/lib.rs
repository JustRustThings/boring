//! Hyper SSL support via OpenSSL.
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use crate::cache::SessionKey;
use boring::error::ErrorStack;
use boring::ex_data::Index;
use boring::ssl::Ssl;
use once_cell::sync::OnceCell;

mod cache;
/// Hyper 1 support.
#[cfg(feature = "hyper1")]
pub mod new;
mod old;

pub use self::old::*;

fn key_index() -> Result<Index<Ssl, SessionKey>, ErrorStack> {
    static IDX: OnceCell<Index<Ssl, SessionKey>> = OnceCell::new();
    IDX.get_or_try_init(Ssl::new_ex_index).copied()
}

/// Settings for [`HttpsLayer`]
pub struct HttpsLayerSettings {
    session_cache_capacity: usize,
}

impl HttpsLayerSettings {
    /// Constructs an [`HttpsLayerSettingsBuilder`] for configuring settings
    pub fn builder() -> HttpsLayerSettingsBuilder {
        HttpsLayerSettingsBuilder(HttpsLayerSettings::default())
    }
}

impl Default for HttpsLayerSettings {
    fn default() -> Self {
        Self {
            session_cache_capacity: 8,
        }
    }
}

/// Builder for [`HttpsLayerSettings`]
pub struct HttpsLayerSettingsBuilder(HttpsLayerSettings);

impl HttpsLayerSettingsBuilder {
    /// Sets maximum number of sessions to cache. Session capacity is per session key (domain).
    /// Defaults to 8.
    pub fn set_session_cache_capacity(&mut self, capacity: usize) {
        self.0.session_cache_capacity = capacity;
    }

    /// Consumes the builder, returning a new [`HttpsLayerSettings`]
    pub fn build(self) -> HttpsLayerSettings {
        self.0
    }
}
