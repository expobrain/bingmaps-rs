use crate::common::CultureCode;
use crate::client::Client;
use crate::error::Error;
use crate::response::Response;
use serde_urlencoded as urlencoded;
use std::collections::HashMap;

// TODO: Implement custom serialize/deserialize for Array to Struct mapping
pub type LatLng = (f64, f64);
// TODO: Use rectangle type
// type SouthWestNorthEast = (f64, f64, f64, f64);

// NOTE: Not GeoJSON, points are "(Lat, Lng)" not "(Lng, Lat)"
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Point {
    // pub type: String // <-- Always Point for Location
    #[serde(rename = "coordinates")]
    pub latlng: LatLng,
}

// TODO: Check with Microsoft Devs, but probably just make this a string-- not an enum
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum EntityType {
    Address,
    Neighborhood,
    PopulatedPlace,
    Postcode1,
    AdminDivision1,
    AdminDivision2,
    CountryRegion,

    // Missing in MSDN documentation, but exists in the wild
    Postcode2,
    Postcode3,
    RoadBlock,
    RoadIntersection,
    HigherEducationFacility,
    Stadium,
    TouristStructure,
    Airport,
    Park,
    Lake,
    River,
    Island,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Address {
    #[serde(rename = "addressLine")]
    pub address_line: Option<String>,
    #[serde(rename = "neighborhood")]
    pub neighborhood: Option<String>,
    #[serde(rename = "locality")]
    pub locality: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    #[serde(rename = "adminDistrict")]
    pub admin_district1: Option<String>,
    #[serde(rename = "adminDistrict2")]
    pub admin_district2: Option<String>,
    #[serde(rename = "countryRegion")]
    pub country: Option<String>,
    #[serde(rename = "countryRegionIso2")]
    pub country_iso: Option<String>,
    #[serde(rename = "landmark")]
    pub landmark: Option<String>,
    #[serde(rename = "formattedAddress")]
    pub formatted: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum Confidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum MatchCode {
    Good,
    Ambiguous,
    UpHierarchy,
}

#[derive(Default, Serialize, Clone)]
pub struct FindPoint {
    pub point: String,
    pub include_entity_types: Vec<EntityType>,
    pub include_neighborhood: bool,
    pub include_ciso2: bool,
}
impl FindPoint {
    pub fn from_latlng(lat: f64, lng: f64) -> FindPoint {
        let mut params = FindPoint::default();
        params.point = format!("{:.5},{:.5}", lat, lng);
        params
    }
    pub fn from_str(latlng: &str) -> FindPoint {
        let mut params = FindPoint::default();
        params.point = latlng.to_owned();
        params
    }
}

// TODO: Maybe use references to ContextParams, instead of the full thing? --- or implement Copy/Clone
#[derive(Default, Serialize, Clone)]
pub struct ContextParams {
    pub culture: Option<CultureCode>,
    pub user_map_view: Option<Vec<f64>>, // TODO: Define a struct
    pub user_location: Option<LatLng>, // TODO: Define a struct
    pub user_ip: Option<String>, // TODO: maybe just use &str?

    // TODO: Convert user_region to enum of ISO 3166-2
    // See https://en.wikipedia.org/wiki/ISO_3166-2
    pub user_region: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    pub name: String,
    pub point: Point,
    /// A geographic area that contains the location.
    /// The box is defined by [South Latitude, West Longitude, North Latitude, East Longitude].
    pub bbox: Vec<f64>,
    #[serde(rename = "entityType")]
    pub entity_type: EntityType,
    pub address: Address,
    pub confidence: Confidence,
    #[serde(rename = "matchCodes")]
    pub match_codes: Vec<MatchCode>,
}

impl Location {
    /// Gets the location information associated with latitude and longitude coordinates.
    pub async fn find_by_point(client: &Client, find: FindPoint, opts: Option<ContextParams>) -> Result<Vec<Location>, Error> {
        let path = format!("/Locations/{}", find.point);

        // Build optional params
        let entity_types: String;
        let culture: String;
        let user_map_view: String;
        let user_location: String;
        let mut params = HashMap::<&str, &str>::new();
        if !find.include_entity_types.is_empty() {
            let types: Vec<String> = find.include_entity_types.iter().map(|el| format!("{:?}", el)).collect();
            entity_types = types.join(",");
            params.insert("include_entity_types", &entity_types);
        }
        if find.include_neighborhood {
            params.insert("inclnb", "1");
        }
        if find.include_ciso2 {
            params.insert("incl", "ciso2");
        }
        if let Some(ref ctx) = opts {
            if let Some(ref c) = ctx.culture {
                culture = urlencoded::to_string(&c).map_err(Error::from)?;
                params.insert("c", &culture);
            }
            if let Some(ref umv) = ctx.user_map_view {
                user_map_view = umv.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
                params.insert("umv", &user_map_view);
            }
            if let Some(ref ul) = ctx.user_location {
                user_location = format!("{},{}", ul.0, ul.1);
                params.insert("ul", &user_location);
            }
            if let Some(ref uip) = ctx.user_ip { params.insert("uip", &uip); }
            if let Some(ref ur) = ctx.user_region { params.insert("ur", &ur); }
        }

        // Make request and process response
        let response: Response<Location> = client.get(&path, &mut params).await?;
        let resource_set = response.resource_sets.into_iter().next();
        if let Some(set) = resource_set {
            Ok(set.resources)
        } else {
            Ok(Vec::new())
        }
    }

    /// Gets latitude and longitude coordinates that correspond to location information provided as a query string.
    pub async fn find_by_query(client: &Client, query: &str, opts: Option<ContextParams>) -> Result<Vec<Location>, Error> {
        let culture: String;
        let user_map_view: String;
        let user_location: String;
        let mut params = HashMap::new();
        params.insert("q", query);
        if let Some(ref ctx) = opts {
            if let Some(ref c) = ctx.culture {
                culture = urlencoded::to_string(&c).map_err(Error::from)?;
                params.insert("c", &culture);
            }
            if let Some(ref umv) = ctx.user_map_view {
                user_map_view = umv.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
                params.insert("umv", &user_map_view);
            }
            if let Some(ref ul) = ctx.user_location {
                user_location = format!("{},{}", ul.0, ul.1);
                params.insert("ul", &user_location);
            }
            if let Some(ref uip) = ctx.user_ip { params.insert("uip", &uip); }
            if let Some(ref ur) = ctx.user_region { params.insert("ur", &ur); }
        }

        // Make request and process response
        let response: Response<Location> = client.get("/Locations", &mut params).await?;
        let resource_set = response.resource_sets.into_iter().next();
        if let Some(set) = resource_set {
            Ok(set.resources)
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json as json;
    use super::Point;

    #[test]
    fn serialize_deserialize_point() {
        let example = r#"{"coordinates":[39.739763,-104.987068]}"#;

        let point: Point = json::from_str(example).unwrap();
        assert_eq!(format!("{:?}", point), "Point { latlng: (39.739763, -104.987068) }");

        let ser = json::to_string(&point).unwrap();
        assert_eq!(ser, example);
    }
}
