use error::Error;
use client::Client;
use response::Response;
use std::collections::HashMap;

// TODO: Maybe use a GeoJson crate here
#[derive(Debug, Deserialize)]
pub struct Point {
    // pub type: String // <-- Always Point for Location
    pub coordinates: Vec<f64>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EntityType {
    Address,
    Neighborhood,
    PopulatedPlace,
    Postcode1,
    AdminDivision1,
    AdminDivision2,
    CountryRegion,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    #[serde(rename = "address_line")]
    pub address_line: Option<String>,
    #[serde(rename = "neighborhood")]
    pub neighborhood: Option<String>,
    #[serde(rename = "locality")]
    pub locality: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    #[serde(rename = "adminDistrict1")]
    pub admin_district_1: Option<String>,
    #[serde(rename = "adminDistrict2")]
    pub admin_district_2: Option<String>,
    #[serde(rename = "countryRegion")]
    pub country: Option<String>,
    #[serde(rename = "countryRegionIso2")]
    pub country_iso: Option<String>,
    #[serde(rename = "landmark")]
    pub landmark: Option<String>,
    #[serde(rename = "formattedAddress")]
    pub formatted: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Confidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum MatchCode {
    Good,
    Ambiguous,
    UpHeirarchy,
}

#[derive(Default, Serialize)]
pub struct FindPoint {
    pub point: String,
    pub include_entity_types: Vec<EntityType>,
    pub include_neighborhood: bool,
    pub include_ciso2: bool,
    pub max_results: Option<u32>,
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

#[derive(Debug, Deserialize)]
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
    pub fn find_by_point(client: &Client, find: FindPoint) -> Result<Vec<Location>, Error> {
        let path = format!("/Locations/{}", find.point);
        let mut params = HashMap::with_capacity(0);

        // Make request and process response
        let response: Response<Location> = client.get(&path, &mut params)?;
        let resource_set = response.resource_sets.into_iter().next();
        if let Some(set) = resource_set {
            Ok(set.resources)
        } else {
            Ok(Vec::new())
        }
    }

    /// Gets latitude and longitude coordinates that correspond to location information provided as a query string.
    pub fn find_by_query(client: &Client, query: &str) -> Result<Vec<Location>, Error> {
        let mut params = HashMap::new();
        params.insert("q", query);

        // Make request and process response
        let response: Response<Location> = client.get("/Locations", &mut params)?;
        let resource_set = response.resource_sets.into_iter().next();
        if let Some(set) = resource_set {
            Ok(set.resources)
        } else {
            Ok(Vec::new())
        }
    }
}
