//! API route handlers for the backend server.
//!
//! This module contains all the HTTP endpoint handlers organized by functionality:
//! - Contact form handling
//! - Health checks
//! - Application metadata

use poem_openapi::Tags;

mod contact;
mod health;
mod meta;

use crate::settings::Settings;

#[derive(Tags)]
enum ApiCategory {
    Contact,
    Health,
    Meta,
}

pub(crate) struct Api {
    contact: contact::ContactApi,
    health: health::HealthApi,
    meta: meta::MetaApi,
}

impl From<&Settings> for Api {
    fn from(value: &Settings) -> Self {
        let contact = contact::ContactApi::from(value.clone().email);
        let health = health::HealthApi;
        let meta = meta::MetaApi::from(&value.application);
        Self {
            contact,
            health,
            meta,
        }
    }
}

impl Api {
    pub fn apis(self) -> (contact::ContactApi, health::HealthApi, meta::MetaApi) {
        (self.contact, self.health, self.meta)
    }
}
