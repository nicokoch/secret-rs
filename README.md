# secret-rs
Rust bindings for [libsecret](https://wiki.gnome.org/Projects/Libsecret).

I am happy about any type of contribution!

## Usage
To use this library in your project, add the following to your Cargo.toml:
```
[dependencies.secret]
git = "https://github.com/nicokoch/secret-rs/"
```

Documentation can be found [here](https://nicokoch.github.io/secret-rs/secret/)

A lot of features from libsecret are missing, so feel free to send me a pull request if you need a certain feature.

##Todos
- Write more TestCases
- Bind to [SecretPrompt](https://developer.gnome.org/libsecret/unstable/SecretPrompt.html) (especially [this](https://developer.gnome.org/libsecret/unstable/SecretService.html#secret-service-prompt-sync))
- Provide bindings to the optional [easy](https://developer.gnome.org/libsecret/unstable/libsecret-Password-storage.html) API
- [SecretSchema](https://developer.gnome.org/libsecret/unstable/libsecret-SecretSchema.html)
- Prettyfy docs
- Prettyfy code
- Github issues
