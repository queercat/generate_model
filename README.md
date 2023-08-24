![Build](https://github.com/queercat/generate_model/actions/workflows/build.yml/badge.svg)
![Test](https://github.com/queercat/generate_model/actions/workflows/test.yml/badge.svg)

# generate_model

generate_model is a Rust macro used for generating JSON objects from structs and writing them to a directory at compile time. This is so that you can easily generate types for APIs.

## Requirements

This macro requires your struct derive from both `Dummy` (Faker) and `Deserialize` Serde. As well as using `serde_json`.

In your `Cargo.toml` you'll want to have:

```toml
serde = { version = "1.0.186", features = ["derive"] }
serde_json = "1.0.105"
fake = { version = "2.8", features = ["derive"] }
```

## Installation

```sh
cargo install generate_model
```

## Usage

Just decorate your structs with `#[generate_model]`. A folder will be created named `generated_models` (you can also specify a path with an environment variable.)

```rs
#[generate_model]
#[derive(Dummy, Serialize)]
struct TestStruct {
  a: String,
  b: u32,
}

#[generate_model]
#[derive(Dummy, Serialize)]
struct TestStruct2 {
  a: TestStruct,
  b: u32,
}
```

And in the `generated_models` folder you'll find...

TestStruct.json

```json
{
  "a": "Z4GOd",
  "b": 2390283944
}
```

TestStruct2.json

```json
{
  "a": {
    "a": "xuVpg",
    "b": 1118086824
  },
  "b": 358934898
}
```

## Configuration

To specify a non-standard path you can set the environment variable `generated_models_directory` to specify an absolute path.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
