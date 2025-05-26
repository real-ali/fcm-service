use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    light_settings::LightSettings, notification_priority::NotificationPriority,
    visibility::Visibility,
};
use crate::domain::android_config::proxy::Proxy;
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct AndroidNotification {
    title: Option<String>,
    body: Option<String>,
    icon: Option<String>,
    color: Option<String>,
    sound: Option<String>,
    tag: Option<String>,
    click_action: Option<String>,
    body_loc_key: Option<String>,
    body_loc_key_args: Option<Vec<String>>,
    title_loc_key: Option<String>,
    title_loc_key_args: Option<Vec<String>>,
    channel_id: Option<String>,
    ticker: Option<String>,
    sticky: Option<bool>,
    event_time: Option<DateTime<Utc>>,
    local_only: Option<bool>,
    notification_priority: Option<NotificationPriority>,
    default_sound: Option<bool>,
    default_light_settings: Option<bool>,
    default_vibrate_timings: Option<bool>,
    vibrate_timings: Option<Vec<String>>,
    visibility: Option<Visibility>,
    notification_count: Option<i32>,
    light_settings: Option<LightSettings>,
    image: Option<String>,
    bypass_proxy_notification: Option<bool>,
    proxy: Option<Proxy>,
}

impl AndroidNotification {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
    }

    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }

    pub fn color(&self) -> Option<&String> {
        self.color.as_ref()
    }

    pub fn sound(&self) -> Option<&String> {
        self.sound.as_ref()
    }

    pub fn tag(&self) -> Option<&String> {
        self.tag.as_ref()
    }

    pub fn click_action(&self) -> Option<&String> {
        self.click_action.as_ref()
    }

    pub fn body_loc_key(&self) -> Option<&String> {
        self.body_loc_key.as_ref()
    }

    pub fn body_loc_key_args(&self) -> Option<&Vec<String>> {
        self.body_loc_key_args.as_ref()
    }

    pub fn title_loc_key(&self) -> Option<&String> {
        self.title_loc_key.as_ref()
    }

    pub fn title_loc_key_args(&self) -> Option<&Vec<String>> {
        self.title_loc_key_args.as_ref()
    }

    pub fn channel_id(&self) -> Option<&String> {
        self.channel_id.as_ref()
    }

    pub fn ticker(&self) -> Option<&String> {
        self.ticker.as_ref()
    }

    pub fn sticky(&self) -> Option<&bool> {
        self.sticky.as_ref()
    }

    pub fn event_time(&self) -> Option<&DateTime<Utc>> {
        self.event_time.as_ref()
    }

    pub fn local_only(&self) -> Option<&bool> {
        self.local_only.as_ref()
    }

    pub fn notification_priority(&self) -> Option<&NotificationPriority> {
        self.notification_priority.as_ref()
    }

    pub fn default_sound(&self) -> Option<&bool> {
        self.default_sound.as_ref()
    }

    pub fn default_light_settings(&self) -> Option<&bool> {
        self.default_light_settings.as_ref()
    }

    pub fn default_vibrate_timings(&self) -> Option<&bool> {
        self.default_vibrate_timings.as_ref()
    }

    pub fn vibrate_timings(&self) -> Option<&Vec<String>> {
        self.vibrate_timings.as_ref()
    }

    pub fn visibility(&self) -> Option<&Visibility> {
        self.visibility.as_ref()
    }

    pub fn notification_count(&self) -> Option<&i32> {
        self.notification_count.as_ref()
    }

    pub fn light_settings(&self) -> Option<&LightSettings> {
        self.light_settings.as_ref()
    }

    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    pub fn bypass_proxy_notification(&self) -> Option<&bool> {
        self.bypass_proxy_notification.as_ref()
    }

    pub fn proxy(&self) -> Option<&Proxy> {
        self.proxy.as_ref()
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_body(&mut self, body: Option<String>) {
        self.body = body;
    }

    pub fn set_icon(&mut self, icon: Option<String>) {
        self.icon = icon;
    }

    pub fn set_color(&mut self, color: Option<String>) {
        self.color = color;
    }

    pub fn set_sound(&mut self, sound: Option<String>) {
        self.sound = sound;
    }

    pub fn set_tag(&mut self, tag: Option<String>) {
        self.tag = tag;
    }

    pub fn set_click_action(&mut self, click_action: Option<String>) {
        self.click_action = click_action;
    }

    pub fn set_body_loc_key(&mut self, body_loc_key: Option<String>) {
        self.body_loc_key = body_loc_key;
    }

    pub fn set_body_loc_key_args(&mut self, body_loc_key_args: Option<Vec<String>>) {
        self.body_loc_key_args = body_loc_key_args;
    }

    pub fn set_title_loc_key(&mut self, title_loc_key: Option<String>) {
        self.title_loc_key = title_loc_key;
    }

    pub fn set_title_loc_key_args(&mut self, title_loc_key_args: Option<Vec<String>>) {
        self.title_loc_key_args = title_loc_key_args;
    }

    pub fn set_channel_id(&mut self, channel_id: Option<String>) {
        self.channel_id = channel_id;
    }

    pub fn set_ticker(&mut self, ticker: Option<String>) {
        self.ticker = ticker;
    }

    pub fn set_sticky(&mut self, sticky: Option<bool>) {
        self.sticky = sticky;
    }

    pub fn set_event_time(&mut self, event_time: Option<DateTime<Utc>>) {
        self.event_time = event_time;
    }

    pub fn set_local_only(&mut self, local_only: Option<bool>) {
        self.local_only = local_only;
    }

    pub fn set_notification_priority(
        &mut self,
        notification_priority: Option<NotificationPriority>,
    ) {
        self.notification_priority = notification_priority;
    }

    pub fn set_default_sound(&mut self, default_sound: Option<bool>) {
        self.default_sound = default_sound;
    }

    pub fn set_default_light_settings(&mut self, default_light_settings: Option<bool>) {
        self.default_light_settings = default_light_settings;
    }

    pub fn set_default_vibrate_timings(&mut self, default_vibrate_timings: Option<bool>) {
        self.default_vibrate_timings = default_vibrate_timings;
    }

    pub fn set_vibrate_timings(&mut self, vibrate_timings: Option<Vec<String>>) {
        self.vibrate_timings = vibrate_timings;
    }

    pub fn set_visibility(&mut self, visibility: Option<Visibility>) {
        self.visibility = visibility;
    }

    pub fn set_notification_count(&mut self, notification_count: Option<i32>) {
        self.notification_count = notification_count;
    }

    pub fn set_light_settings(&mut self, light_settings: Option<LightSettings>) {
        self.light_settings = light_settings;
    }

    pub fn set_image(&mut self, image: Option<String>) {
        self.image = image;
    }

    pub fn set_bypass_proxy_notification(&mut self, bypass_proxy_notification: Option<bool>) {
        self.bypass_proxy_notification = bypass_proxy_notification;
    }

    pub fn set_proxy(&mut self, proxy: Option<Proxy>) {
        self.proxy = proxy;
    }
}
