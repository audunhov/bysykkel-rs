use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Deserialize, Debug)]
struct Config {
    stations: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationInformationRoot {
    #[serde(rename = "last_updated")]
    pub last_updated: i64,
    pub ttl: i64,
    pub version: String,
    pub data: StationInformationData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationInformationData {
    pub stations: Vec<StationInformation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationInformation {
    #[serde(rename = "station_id")]
    pub station_id: String,
    pub name: String,
    pub address: String,
    #[serde(rename = "cross_street")]
    pub cross_street: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(rename = "is_virtual_station")]
    pub is_virtual_station: bool,
    pub capacity: i64,
    #[serde(rename = "station_area")]
    pub station_area: StationArea,
    #[serde(rename = "rental_uris")]
    pub rental_uris: RentalUris,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationArea {
    #[serde(rename = "type")]
    pub type_field: String,
    pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RentalUris {
    pub android: String,
    pub ios: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationStatusRoot {
    #[serde(rename = "last_updated")]
    pub last_updated: i64,
    pub ttl: i64,
    pub version: String,
    pub data: StationStatusData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationStatusData {
    pub stations: Vec<StationData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationData {
    #[serde(rename = "station_id")]
    pub station_id: String,
    #[serde(rename = "is_installed")]
    pub is_installed: bool,
    #[serde(rename = "is_renting")]
    pub is_renting: bool,
    #[serde(rename = "is_returning")]
    pub is_returning: bool,
    #[serde(rename = "last_reported")]
    pub last_reported: i64,
    #[serde(rename = "num_vehicles_available")]
    pub num_vehicles_available: i64,
    #[serde(rename = "num_bikes_available")]
    pub num_bikes_available: i64,
    #[serde(rename = "num_docks_available")]
    pub num_docks_available: i64,
    #[serde(rename = "vehicle_types_available")]
    pub vehicle_types_available: Vec<VehicleTypesAvailable>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleTypesAvailable {
    #[serde(rename = "vehicle_type_id")]
    pub vehicle_type_id: String,
    pub count: i64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let config_file = include_str!("../config.json");
    let config = serde_json::from_str::<Config>(config_file)?;

    debug!("fetching station_information");

    let response =
        reqwest::get("https://gbfs.urbansharing.com/oslobysykkel.no/station_information.json")
            .await?
            .text()
            .await?;

    let station_information = serde_json::from_str::<StationInformationRoot>(&response)?;

    debug!("fetching station_status");

    let response =
        reqwest::get("https://gbfs.urbansharing.com/oslobysykkel.no/station_status.json")
            .await?
            .text()
            .await?;

    let station_status = serde_json::from_str::<StationStatusRoot>(&response)?;

    let station_infos: Vec<&StationInformation> = station_information
        .data
        .stations
        .iter()
        .filter(|station| config.stations.contains(&station.name))
        .collect();

    let station_statuses: Vec<&StationData> = station_infos
        .iter()
        .filter_map(|info| {
            station_status
                .data
                .stations
                .iter()
                .find(|s| s.station_id == info.station_id)
        })
        .collect();

    station_statuses.iter().for_each(|status| {
        let name = station_infos
            .iter()
            .find(|info| status.station_id == info.station_id)
            .unwrap()
            .name
            .clone();

        println!(
            "{}: {} sykler, {} stativ",
            name, status.num_bikes_available, status.num_docks_available
        );
    });

    Ok(())
}
