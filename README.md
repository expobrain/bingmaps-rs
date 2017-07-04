bingmaps-rs
=============

[![bingmaps on crates.io](https://img.shields.io/crates/v/bingmaps.svg)](https://crates.io/crates/bingmaps)
[![bingmaps on docs.rs](https://docs.rs/bingmaps/badge.svg)](https://docs.rs/bingmaps)

Rust API bindings for the Bing Maps v1 HTTP API.

The bindings currently support:
 * Geocoding by Search Query
 * Geocoding by Latitude / Longitude

## Usage
Put this in your `Cargo.toml`:

```toml
[dependencies]
bingmaps = "0.0.3"
```

And this in your crate root:

```rust
extern crate bingmaps;
```

## Example
```rust
use bingmaps;
use bingmaps::locations::{Location, FindPoint, EntityType, Confidence, MatchCode};
use std::env;

let key = env::var("BING_MAPS_KEY").unwrap();
let client = bingmaps::Client::new();

// Find a Location by search-term / query
let locations = Location::find_by_query(&client, "Times Square, New York", None).unwrap();

// Find a Location by Lat/Lng values OR from text (eg. FindPoint::from_str("40.75890,-73.98516");)
let params = FindPoint::from_latlng(40.758903, -73.985163);
let locations = Location::find_by_point(&client, params, None).unwrap();
println!("{:#?}", locations.next().unwrap());

/*
Location {
    name: "1551 7th Ave, New York, NY 10036",
    point: Point {coordinates: [40.75891, -73.98546]},
    bbox: [40.75504728242933, -73.9922589957156, 40.76277271757068, -73.9786610042844],
    entity_type: Address,
    address: Address {
        address_line: Some("1551 7th Ave"),
        neighborhood: None,
        locality: Some("New York"),
        postal_code: Some("10036"),
        admin_district1: Some("NY"),
        admin_district2: Some("New York Co."),
        country: Some("United States"),
        country_iso: None,
        landmark: None,
        formatted: Some("1551 7th Ave, New York, NY 10036"),
    },
    confidence: High,
    match_codes: [Good],
}
```
