## Foodle API for rust

This is a simple library to speak with the [Foodles](https://www.foodles.co/) API.

### Usage

<!--```rust
use foodle_api::Foodle;

fn main() {
    let foodle = Foodle::new("YOUR_API_KEY");
    let foodles = foodle.get_foodles().unwrap();
    println!("{:?}", foodles);
}
```-->

```rust
use foodle_api::fridge;

fn get_fridge_data() {
    let cookie_value = "YOUR_SESSIONID_COOKIE_VALUE";	
    fridge::get(cookie_value)?
}
```

### Motivation

The Foodle API was pretty simple to use, so I decided to make a library for it.
It's pretty fun to make stats about quantities of food over time, and this allow me to learn more about rust.

### License

This project is licensed under the Unlicense license - see the [LICENSE.md](LICENSE.md) file for details

