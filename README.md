# Firebase Cloud Messaging (FCM) Rust Service
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

A professional Rust library for sending Firebase Cloud Messaging (FCM) notifications with ease and efficiency.

## What is `fcm-service`?

`fcm-service` is a lightweight and efficient Rust library that enables seamless integration with Firebase Cloud Messaging (FCM). It allows developers to send push notifications to devices across platforms securely using Google Cloud authentication.

## Features

- Simple and easy-to-use API
- Secure authentication via Google Cloud Service Account
- Fully asynchronous using `tokio`
- Supports structured FCM messages
- Customizable notification payloads
- Well-tested with included unit tests

## Installation

To use `fcm-service`, add the following to your `Cargo.toml`:

```toml
[dependencies]
fcm-service = "0.2.1"
```

Alternatively, if using GitHub as the source:

```toml
[dependencies]
fcm-service = { git = "https://github.com/real-ali/fcm-service" }
```

## Usage

Here’s a quick example demonstrating how to send an FCM notification:

```rust
use fcm_service::{FcmService, FcmMessage, FcmNotification, Target};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = FcmService::new("path/to/service-account.json");

    let mut message = FcmMessage::new();
    let mut notification = FcmNotification::new();
    notification.set_title("Hello".to_string());
    notification.set_body("World".to_string());
    notification.set_image(None);
    message.set_notification(Some(notification));
    message.set_target(Target::Token("device-token".to_string()));

    service.send_notification(message).await?;
    Ok(())
}
```

## Authentication

This service uses a Google Cloud service account JSON file to authenticate API calls. Make sure you have a valid credential file from Google Cloud and provide its path when initializing the `FcmService`.

## Running Tests

To ensure everything is working correctly, run:

```sh
cargo test
```

### Example Test Cases

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile;

    #[test]
    fn test_new_service() {
        let service = FcmService::new("dummy.json");
        assert_eq!(service.credential_file, "dummy.json");
    }
}
```

## Package Information

```toml
[package]
name = "fcm-service"
version = "0.1.0"
edition = "2021"
description = "A Rust library for sending Firebase Cloud Messaging (FCM) notifications"
license = "MIT"
repository = "https://github.com/real-ali/fcm-service"
documentation = "https://docs.rs/fcm-service"
keywords = ["fcm", "firebase", "notifications", "push"]
categories = ["api-bindings", "network-programming"]
```

## Dependencies

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["macros"] }
gcp_auth = "0.12.3"

[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
tempfile = "3.10"
```

## Author

Designed & Developed by **Sayed Ali Sina Hussaini**

### Social Links

- Twitter: [real_alisina](https://twitter.com/real_alisina)
- GitHub: [real-ali](https://github.com/real-ali)

## License

This project is licensed under the MIT License.

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/valkrypton"><img src="https://avatars.githubusercontent.com/u/74088901?v=4?s=100" width="100px;" alt="Ali Tariq"/><br /><sub><b>Ali Tariq</b></sub></a><br /><a href="https://github.com/real-ali/fcm-service/commits?author=valkrypton" title="Code">💻</a> <a href="https://github.com/real-ali/fcm-service/commits?author=valkrypton" title="Documentation">📖</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!