mod android_notification;
mod light_settings;
mod notification_priority;
mod proxy;
mod visibility;

use serde::{Deserialize, Serialize};

pub use self::{
    android_notification::AndroidNotification,
    light_settings::{Color, LightSettings},
    notification_priority::NotificationPriority,
    proxy::Proxy,
    visibility::Visibility,
};
use crate::FcmOptions;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Priority {
    Normal,
    High,
}

/// Configuration for Android devices.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AndroidConfig {
    collapse_key: Option<String>,
    priority: Option<Priority>,
    ttl: Option<String>,
    restricted_package_name: Option<String>,
    notification: Option<AndroidNotification>,
    fcm_option: Option<FcmOptions>,
    direct_boot_ok: Option<bool>,
}

impl AndroidConfig {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    #[must_use]
    pub fn collapse_key(&self) -> Option<&String> {
        self.collapse_key.as_ref()
    }

    #[must_use]
    pub fn priority(&self) -> Option<&Priority> {
        self.priority.as_ref()
    }

    #[must_use]
    pub fn ttl(&self) -> Option<&String> {
        self.ttl.as_ref()
    }

    #[must_use]
    pub fn restricted_package_name(&self) -> Option<&String> {
        self.restricted_package_name.as_ref()
    }

    #[must_use]
    pub fn notification(&self) -> Option<&AndroidNotification> {
        self.notification.as_ref()
    }
    #[must_use]
    pub fn fcm_option(&self) -> Option<&FcmOptions> {
        self.fcm_option.as_ref()
    }

    #[must_use]
    pub fn direct_boot_ok(&self) -> Option<bool> {
        self.direct_boot_ok
    }

    pub fn set_restricted_package_name(&mut self, restricted_package_name: Option<String>) {
        self.restricted_package_name = restricted_package_name;
    }

    pub fn set_notification(&mut self, notification: Option<AndroidNotification>) {
        self.notification = notification;
    }

    pub fn set_fcm_option(&mut self, fcm_option: Option<FcmOptions>) {
        self.fcm_option = fcm_option;
    }

    pub fn set_direct_boot_ok(&mut self, direct_boot_ok: Option<bool>) {
        self.direct_boot_ok = direct_boot_ok;
    }

    pub fn set_collapse_key(&mut self, collapse_key: Option<String>) {
        self.collapse_key = collapse_key;
    }

    pub fn set_priority(&mut self, priority: Option<Priority>) {
        self.priority = priority;
    }

    pub fn set_ttl(&mut self, ttl: Option<String>) {
        self.ttl = ttl;
    }
}
