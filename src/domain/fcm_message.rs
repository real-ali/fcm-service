use std::{collections::HashMap, hash::RandomState};

use serde::{Deserialize, Serialize};

use super::{AndroidConfig, ApnsConfig, FcmNotification, FcmOptions, Target, WebpushConfig};

/// Represents an FCM message with all supported fields.
#[derive(Serialize, Deserialize, Default)]
pub struct FcmMessage {
    name: Option<String>,
    data: Option<HashMap<String, String>>,
    notification: Option<FcmNotification>,
    android: Option<AndroidConfig>,
    webpush: Option<WebpushConfig>,
    apns: Option<ApnsConfig>,
    fcm_options: Option<FcmOptions>,
    #[serde(flatten)]
    target: Target,
}

impl FcmMessage {
    /// Creates a new, empty `FcmMessage`.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Sets the message name.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Sets custom data key-value pairs.
    pub fn set_data(&mut self, data: Option<HashMap<String, String>>) {
        self.data = data;
    }

    /// Sets the notification content.
    pub fn set_notification(&mut self, notification: Option<FcmNotification>) {
        self.notification = notification;
    }

    /// Sets Android-specific configuration.
    pub fn set_android(&mut self, android: Option<AndroidConfig>) {
        self.android = android;
    }

    /// Sets Webpush-specific configuration.
    pub fn set_webpush(&mut self, webpush: Option<WebpushConfig>) {
        self.webpush = webpush;
    }

    /// Sets APNs-specific configuration.
    pub fn set_apns(&mut self, apns: Option<ApnsConfig>) {
        self.apns = apns;
    }

    /// Sets FCM options.
    pub fn set_fcm_options(&mut self, fcm_options: Option<FcmOptions>) {
        self.fcm_options = fcm_options;
    }

    /// Sets the message target (token, topic, or condition).
    pub fn set_target(&mut self, target: Target) {
        self.target = target;
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn data(&self) -> Option<&HashMap<String, String, RandomState>> {
        self.data.as_ref()
    }

    pub fn notification(&self) -> Option<&FcmNotification> {
        self.notification.as_ref()
    }

    pub fn android(&self) -> Option<&AndroidConfig> {
        self.android.as_ref()
    }

    pub fn webpush(&self) -> Option<&WebpushConfig> {
        self.webpush.as_ref()
    }

    pub fn apns(&self) -> Option<&ApnsConfig> {
        self.apns.as_ref()
    }

    pub fn fcm_options(&self) -> Option<&FcmOptions> {
        self.fcm_options.as_ref()
    }

    pub fn target(&self) -> &Target {
        &self.target
    }
}
