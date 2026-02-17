
# Referral codes

This is a crate largely inspired by the [referral-codes](https://www.npmjs.com/package/referral-codes) library on NPM.

The usage is similar. You can either use the default config

```rust
    let config = Config::default();
```

Or define your own

```rust
    let config = Config {
        charset: Charset::Alphanumeric,
        count: 3,
        pattern: Pattern::Length(8),
    }
```

Then, you can call `generate_one` to generate a single referral code,
or you can call `generate` to generate `count` referral codes,
according to the configuration defined above.

