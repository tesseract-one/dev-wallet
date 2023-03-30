<p align="center">
	<a href="http://tesseract.one/">
		<img alt="Tesseract" src ="./.github/logo.svg" height=256/>
	</a>
</p>

### **Tesseract Developer Wallet** is designed to help you test your dApp integration with Wallets through [Tesseract](https://github.com/tesseract-one/) **dApp/Wallet integration** protocol

## Prerequisites

### Common prerequisites

Install Rust Environment (nightly) from [here](https://www.rust-lang.org/tools/install).

### Android prerequisites

1. Install Android toolchanins:

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

2. Install [Android Studio](https://developer.android.com/studio)
3. Install [Android NDK](https://developer.android.com/studio/projects/install-ndk#default-version) (no need for CMAKE)

### iOS prerequisites

1. Install iOS toolchanins:

```bash
rustup target add aarch64-apple-ios-sim aarch64-apple-ios x86_64-apple-ios
```

2. Install Xcode from the App Store

## Installation

### Installation on Android

1. Checkout [this repo](https://github.com/tesseract-one/dev-wallet) and [Tesseract.android](https://github.com/tesseract-one/Tesseract.android) side by side.
2. Open dev-waller folder in Android Studio.
3. Check `local.properties.example` and copy the ones you need for your environment (i.e. your NDK version) into your `local.properties`.
4. Run it on a desired device (or emulator).

### Installation on iOS

1. Checkout [this repo](https://github.com/tesseract-one/dev-wallet).
2. Open `ios/Developer Wallet.xcodeproj` in Xcode.
3. Run the **Developer Wallet**.

## How to use

Just call **Tesseract** methods from within your dApp and it will show you the option to choose Developer Wallet when you try to request a public key or sign a transaction.

## Supported protocols

* [Substrate/Polkadot](https://github.com/tesseract-one/Tesseract.rs/tree/master/protocols/substrate)
* [Tesseract Test Protocol](https://github.com/tesseract-one/Tesseract.rs/tree/master/protocols/test)
* Please, crate a github issue to request your blockchain support or contact us by e-mail

## Setting up VSCode

The way to make things workable in VSCode, use these settings (`.vscode/settings.json`):

```json
{
    "rust-analyzer.cargo.target": "x86_64-apple-ios",
    "rust-analyzer.cargo.features": [
        "substrate"
    ]
}
```

or alternatively use one of the other targets depending on your needs:

```bash
#Android
aarch64-linux-android
armv7-linux-androideabi
i686-linux-android
x86_64-linux-android
#iOS
aarch64-apple-ios-sim
aarch64-apple-ios
x86_64-apple-ios
```

Also, you can switch off the protocol features when not working on them.

## License

Tesseract.rs can be used, distributed and modified under [the Apache 2.0 license](LICENSE).
