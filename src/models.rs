use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// ENDPOINT CONSTANTS
// ============================================================================

/// #[serde(skip_serializing_if = "Option::is_none")]

pub mod endpoints {
    pub const VERSION: &str = "/api/version";
    pub const CONNECTION: &str = "/api/connection";
    pub const PRINTER_PROFILES: &str = "/api/printerprofiles";
    pub const PRINTER: &str = "/api/printer";
    pub const PRINTER_SD: &str = "/api/printer/sd";
    pub const PRINTER_PRINTHEAD: &str = "/api/printer/printhead";
    pub const PRINTER_TOOL: &str = "/api/printer/tool";
    pub const PRINTER_BED: &str = "/api/printer/bed";
    pub const PRINTER_ERROR: &str = "/api/printer/error";
    pub const JOB: &str = "/api/job";
    pub const SYSTEM_COMMANDS: &str = "/api/system/commands";
    pub const SYSTEM_REBOOT: &str = "/api/system/commands/core/reboot";
    pub const SYSTEM_RESTART: &str = "/api/system/commands/core/restart";
    pub const SYSTEM_SHUTDOWN: &str = "/api/system/commands/core/shutdown";
    pub const SYSTEM_CHANGE_EXPOSURE: &str = "/api/system/commands/custom/changeexposure";
    pub const SYSTEM_RESIN_REFILL: &str = "/api/system/commands/custom/resinrefill";
    pub const ACCESS_USERS: &str = "/api/access/users";
    pub const SETTINGS: &str = "/api/settings";
    pub const SETTINGS_APIKEY: &str = "/api/settings/apikey";
    pub const SETTINGS_SN: &str = "/api/settings/sn";
    pub const LOGS: &str = "/api/logs";
}

// ============================================================================
// VERSION ENDPOINT TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python: Option<Vec<PythonPackage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonPackage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python: Option<String>,
    #[serde(rename = "DESCRIPTION", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "OS", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}

// ============================================================================
// CONNECTION ENDPOINT TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<ConnCurrent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ConnOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<ConnConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<ConnStates>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnCurrent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baudrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "printerProfile", skip_serializing_if = "Option::is_none")]
    pub printer_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baudrates: Option<Vec<u32>>,
    #[serde(rename = "printerProfiles", skip_serializing_if = "Option::is_none")]
    pub printer_profiles: Option<Vec<PrinterProfileOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterProfileOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnStates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printer: Option<ConnectionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<ConnectionState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baudrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "printerProfile", skip_serializing_if = "Option::is_none")]
    pub printer_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<ConnConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionResponse200 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// ============================================================================
// PRINTER PROFILES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterProfilesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<HashMap<String, PrinterProfile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(rename = "heatedBed", skip_serializing_if = "Option::is_none")]
    pub heated_bed: Option<bool>,
    #[serde(rename = "heatedChamber", skip_serializing_if = "Option::is_none")]
    pub heated_chamber: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extruder: Option<ExtruderInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtruderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsets: Option<Vec<Vec<f64>>>,
}

// ============================================================================
// PRINTER STATE TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum State {
    Operational,
    Printing,
    Pausing,
    Paused,
    Cancelling,
    Error,
    Offline,
    Busy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<TemperatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sd: Option<SDState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<PrinterState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<serde_json::Value>, // Can be FDM, Mini, or SL1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool0: Option<TemperatureData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bed: Option<TemperatureData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chamber: Option<TemperatureData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<PrinterFlags>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterFlags {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelling: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pausing: Option<bool>,
    #[serde(rename = "sdReady", skip_serializing_if = "Option::is_none")]
    pub sd_ready: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(rename = "closedOrError", skip_serializing_if = "Option::is_none")]
    pub closed_or_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_state: Option<LinkState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LinkState {
    Idle,
    Busy,
    Printing,
    Paused,
    Finished,
    Stopped,
    Error,
    Attention,
    Ready,
}

// ============================================================================
// TELEMETRY TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryFDM {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_bed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_nozzle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(rename = "z-height", skip_serializing_if = "Option::is_none")]
    pub z_height: Option<f64>,
    #[serde(rename = "print-speed", skip_serializing_if = "Option::is_none")]
    pub print_speed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_z: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryMini {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_bed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_nozzle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySL1 {
    #[serde(rename = "fanUvLed", skip_serializing_if = "Option::is_none")]
    pub fan_uv_led: Option<u32>,
    #[serde(rename = "fanBlower", skip_serializing_if = "Option::is_none")]
    pub fan_blower: Option<u32>,
    #[serde(rename = "fanRear", skip_serializing_if = "Option::is_none")]
    pub fan_rear: Option<u32>,
    #[serde(rename = "coverClosed", skip_serializing_if = "Option::is_none")]
    pub cover_closed: Option<bool>,
    #[serde(rename = "tempAmbient", skip_serializing_if = "Option::is_none")]
    pub temp_ambient: Option<f64>,
    #[serde(rename = "tempCpu", skip_serializing_if = "Option::is_none")]
    pub temp_cpu: Option<f64>,
    #[serde(rename = "tempUvLed", skip_serializing_if = "Option::is_none")]
    pub temp_uv_led: Option<f64>,
}

// ============================================================================
// PRINTHEAD CONTROL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintheadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jog: Option<JogCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home: Option<HomeCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<SpeedCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedrate: Option<FeedrateCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_steppers: Option<DisableSteppersCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JogCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedrateCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableSteppersCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}

// ============================================================================
// TOOL CONTROL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TargetCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<OffsetCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<SelectCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extrude: Option<ExtrudeCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retract: Option<RetractCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flowrate: Option<FlowrateCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffsetCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsets: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtrudeCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetractCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowrateCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

// ============================================================================
// BED CONTROL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<f64>,
}

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub title: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// ============================================================================
// JOB TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<Progress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "estimatedPrintTime", skip_serializing_if = "Option::is_none")]
    pub estimated_print_time: Option<u32>,
    #[serde(rename = "averagePrintTime", skip_serializing_if = "Option::is_none")]
    pub average_print_time: Option<u32>,
    #[serde(rename = "lastPrintTime", skip_serializing_if = "Option::is_none")]
    pub last_print_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<JobFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value, // For FDM/SLA specific fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<FileRefs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<u32>,
    #[serde(rename = "layerHeight", skip_serializing_if = "Option::is_none")]
    pub layer_height: Option<f64>,
    #[serde(rename = "exposureTime", skip_serializing_if = "Option::is_none")]
    pub exposure_time: Option<u32>,
    #[serde(rename = "exposureTimeFirst", skip_serializing_if = "Option::is_none")]
    pub exposure_time_first: Option<f64>,
    #[serde(rename = "exposureTimeCalibration", skip_serializing_if = "Option::is_none")]
    pub exposure_time_calibration: Option<f64>,
    #[serde(rename = "exposureUserProfile", skip_serializing_if = "Option::is_none")]
    pub exposure_user_profile: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRefs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "thumbnailBig", skip_serializing_if = "Option::is_none")]
    pub thumbnail_big: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filepos: Option<u64>,
    #[serde(rename = "printTime", skip_serializing_if = "Option::is_none")]
    pub print_time: Option<u32>,
    #[serde(rename = "printTimeLeft", skip_serializing_if = "Option::is_none")]
    pub print_time_left: Option<u32>,
    #[serde(flatten)]
    pub extra: serde_json::Value, // For printer-specific fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMK3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_z_mm: Option<f64>,
    #[serde(rename = "printSpeed", skip_serializing_if = "Option::is_none")]
    pub print_speed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_factor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filepos: Option<u64>,
    #[serde(rename = "printTime", skip_serializing_if = "Option::is_none")]
    pub print_time: Option<u32>,
    #[serde(rename = "printTimeLeft", skip_serializing_if = "Option::is_none")]
    pub print_time_left: Option<u32>,
    #[serde(rename = "printTimeLeftOrigin", skip_serializing_if = "Option::is_none")]
    pub print_time_left_origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMini {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_z_mm: Option<f64>,
    #[serde(rename = "printSpeed", skip_serializing_if = "Option::is_none")]
    pub print_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressSL1 {
    #[serde(rename = "currentLayer", skip_serializing_if = "Option::is_none")]
    pub current_layer: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<f64>,
    #[serde(rename = "printTime", skip_serializing_if = "Option::is_none")]
    pub print_time: Option<u32>,
    #[serde(rename = "printTimeLeft", skip_serializing_if = "Option::is_none")]
    pub print_time_left: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<JobCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<JobAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobCommand {
    Start,
    Restart,
    Pause,
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobAction {
    Pause,
    Resume,
}

// ============================================================================
// SYSTEM COMMANDS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<SystemCommand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

// ============================================================================
// CUSTOM SYSTEM COMMANDS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeExposureRequest {
    #[serde(rename = "exposureTime", skip_serializing_if = "Option::is_none")]
    pub exposure_time: Option<i32>,
    #[serde(rename = "exposureTimeFirst", skip_serializing_if = "Option::is_none")]
    pub exposure_time_first: Option<f64>,
    #[serde(rename = "exposureTimeCalibration", skip_serializing_if = "Option::is_none")]
    pub exposure_time_calibration: Option<f64>,
    #[serde(rename = "exposureUserProfile", skip_serializing_if = "Option::is_none")]
    pub exposure_user_profile: Option<f64>,
}

// ============================================================================
// ACCESS / USERS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apikey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<bool>,
}

// ============================================================================
// SETTINGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsResponse {
    #[serde(rename = "api-key", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printer: Option<PrinterSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub farm_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_error_chime: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printer: Option<PrinterSettingsUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserSettingsUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub farm_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_error_chime: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterSettingsUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_repassword: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<SettingsErrors>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsErrors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printer: Option<PrinterErrors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserErrors>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterErrors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_credentials: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserErrors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repassword: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_digest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_digest: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SNRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
}

// ============================================================================
// FILE OPERATIONS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(rename = "typePath", skip_serializing_if = "Option::is_none")]
    pub type_path: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<FileRefs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<u64>,
    #[serde(rename = "gcodeAnalysis", skip_serializing_if = "Option::is_none")]
    pub gcode_analysis: Option<GcodeAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcodeAnalysis {
    #[serde(rename = "estimatedPrintTime", skip_serializing_if = "Option::is_none")]
    pub estimated_print_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(rename = "layerHeight", skip_serializing_if = "Option::is_none")]
    pub layer_height: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(rename = "typePath", skip_serializing_if = "Option::is_none")]
    pub type_path: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<FileRefsDetailed>,
    #[serde(rename = "gcodeAnalysis", skip_serializing_if = "Option::is_none")]
    pub gcode_analysis: Option<GcodeAnalysisDetailed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRefsDetailed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<String>,
    #[serde(rename = "thumbnailSmall", skip_serializing_if = "Option::is_none")]
    pub thumbnail_small: Option<String>,
    #[serde(rename = "thumbnailBig", skip_serializing_if = "Option::is_none")]
    pub thumbnail_big: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcodeAnalysisDetailed {
    #[serde(rename = "estimatedPrintTime", skip_serializing_if = "Option::is_none")]
    pub estimated_print_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(rename = "layerHeight", skip_serializing_if = "Option::is_none")]
    pub layer_height: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub folder_type: Option<String>,
    #[serde(rename = "typePath", skip_serializing_if = "Option::is_none")]
    pub type_path: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<FolderRefs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderRefs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllFilesInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSelectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCommandRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<FileCommandType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileCommandType {
    Select,
    Copy,
    Move,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRequest {
    pub source: String,
    pub destination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    pub path: String,
    pub foldername: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub destination: String,
    #[serde(default)]
    pub to_select: bool,
    #[serde(default)]
    pub to_print: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadInfo {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_select: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_print: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploaded {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<HashMap<String, FileUploadedInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadedInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<FileRefsDetailed>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderCreated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<FolderCreatedInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderCreatedInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<FolderRefs>,
}

// ============================================================================
// LOGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<LogFile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<LogRefs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRefs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<String>,
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub title: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// use serde::{Deserialize, Serialize};
//
// // Version endpoint response
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Version {
//     pub api: String,
//     pub server: String,
//     pub original: String,
//     pub text: String,
//     pub firmware: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub sdk: Option<String>,
//     #[serde(default)]
//     pub capabilities: Capabilities,
//     hostname: String,
// }
//
// #[derive(Debug, Clone, Default, Serialize, Deserialize)]
// pub struct Capabilities {
//     #[serde(rename = "upload-by-put", default)]
//     pub upload_by_put: bool,
// }
//
// // Info endpoint response
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Info {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub mmu: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub location: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub farm_mode: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub nozzle_diameter: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub min_extrusion_temp: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub serial: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub sd_ready: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub active_camera: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub hostname: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub port: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub network_error_chime: Option<bool>,
// }
//
// // Status endpoint response
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Status {
//     pub printer: StatusPrinter,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub job: Option<StatusJob>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub transfer: Option<StatusTransfer>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub storage: Option<StatusStorage>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub camera: Option<StatusCamera>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct StatusJob {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub id: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub progress: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub time_remaining: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub time_printing: Option<i32>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct StatusPrinter {
//     pub state: PrinterState,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub temp_nozzle: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub target_nozzle: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub temp_bed: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub target_bed: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub axis_x: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub axis_y: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub axis_z: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub flow: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub speed: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub fan_hotend: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub fan_print: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub printer: Option<PrinterStatus>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub status_connect: Option<ConnectStatus>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum PrinterState {
//     Idle,
//     Busy,
//     Printing,
//     Paused,
//     Finished,
//     Stopped,
//     Error,
//     Attention,
//     Ready,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PrinterStatus {
//     pub ok: bool,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub message: Option<String>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ConnectStatus {
//     pub ok: bool,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub message: Option<String>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct StatusTransfer {
//     pub id: i64,
//     pub time_transferring: i32,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub progress: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub data_transferred: Option<i64>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct StatusStorage {
//     pub name: String,
//     pub path: String,
//     pub read_only: bool,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub free_space: Option<i64>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct StatusCamera {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub id: Option<String>,
// }
//
// // Storage endpoint response
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct StorageList {
//     pub storage_list: Vec<Storage>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Storage {
//     pub available: bool,
//     pub path: String,
//     #[serde(rename = "type")]
//     pub storage_type: StorageType,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub print_files: Option<i64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub system_files: Option<i64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub free_space: Option<i64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub total_space: Option<i64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub read_only: Option<bool>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum StorageType {
//     Local,
//     Sdcard,
//     Usb,
// }
//
// // Job endpoint response
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Job {
//     pub id: i32,
//     pub state: JobState,
//     pub progress: f64,
//     pub time_printing: i32,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub time_remaining: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub inaccurate_estimates: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub serial_print: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub file: Option<JobFile>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum JobState {
//     Printing,
//     Paused,
//     Finished,
//     Stopped,
//     Error,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct JobFile {
//     pub name: String,
//     pub path: String,
//     pub m_timestamp: i64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub display_name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub display_path: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub size: Option<i64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub meta: Option<PrintFileMetadata>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub refs: Option<PrintFileRefs>,
// }
//
// // Transfer endpoint response
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Transfer {
//     #[serde(rename = "type")]
//     pub transfer_type: TransferType,
//     pub display_name: String,
//     pub path: String,
//     pub progress: f64,
//     pub transferred: i64,
//     pub time_transferring: i32,
//     pub to_print: bool,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub url: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub size: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub time_remaining: Option<i32>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum TransferType {
//     NoTransfer,
//     FromWeb,
//     FromConnect,
//     FromPrinter,
//     FromSlicer,
//     FromClient,
//     ToConnect,
//     ToClient,
// }
//
// // File info structures
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum FileInfo {
//     PrintFile(PrintFileInfo),
//     Firmware(FirmwareFileInfo),
//     File(GenericFileInfo),
//     Folder(FolderInfo),
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct GenericFileInfo {
//     pub name: String,
//     pub read_only: bool,
//     pub m_timestamp: i64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub size: Option<i64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub display_name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub refs: Option<FileRefs>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct FileRefs {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub download: Option<String>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PrintFileInfo {
//     pub name: String,
//     pub read_only: bool,
//     pub m_timestamp: i64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub size: Option<i64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub display_name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub refs: Option<PrintFileRefs>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub meta: Option<PrintFileMetadata>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PrintFileRefs {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub download: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub icon: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub thumbnail: Option<String>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PrintFileMetadata {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub bed_temperature: Option<i32>,
//     #[serde(
//         rename = "bed_temperature per tool",
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub bed_temperature_per_tool: Option<Vec<i32>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub temperature: Option<i32>,
//     #[serde(
//         rename = "temperature per tool",
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub temperature_per_tool: Option<Vec<i32>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub brim_width: Option<i32>,
//     #[serde(
//         rename = "estimated printing time (normal mode)",
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub estimated_printing_time_normal: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub estimated_print_time: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub faded_layers: Option<i32>,
//     #[serde(rename = "filament cost", skip_serializing_if = "Option::is_none")]
//     pub filament_cost: Option<f64>,
//     #[serde(
//         rename = "filament cost per tool",
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub filament_cost_per_tool: Option<Vec<f64>>,
//     #[serde(
//         rename = "filament used [cm3]",
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub filament_used_cm3: Option<f64>,
//     #[serde(
//         rename = "filament used [cm3] per tool",
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub filament_used_cm3_per_tool: Option<Vec<f64>>,
//     #[serde(rename = "filament used [g]", skip_serializing_if = "Option::is_none")]
//     pub filament_used_g: Option<f64>,
//     #[serde(
//         rename = "filament used [g] per tool",
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub filament_used_g_per_tool: Option<Vec<f64>>,
//     #[serde(rename = "filament used [mm]", skip_serializing_if = "Option::is_none")]
//     pub filament_used_mm: Option<f64>,
//     #[serde(
//         rename = "filament used [mm] per tool",
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub filament_used_mm_per_tool: Option<Vec<f64>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub filament_type: Option<String>,
//     #[serde(
//         rename = "filament_type per tool",
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub filament_type_per_tool: Option<Vec<String>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub fill_density: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub initial_exposure_time: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub layer_height: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub material_name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub exposure_time: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub max_exposure_time: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub max_initial_exposure_time: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub min_exposure_time: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub min_initial_exposure_time: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub nozzle_diameter: Option<f64>,
//     #[serde(
//         rename = "nozzle_diameter per tool",
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub nozzle_diameter_per_tool: Option<Vec<f64>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub normal_percent_present: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub normal_left_present: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub quiet_percent_present: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub quiet_left_present: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub layer_info_present: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub max_layer_z: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub print_time: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub printer_model: Option<PrinterModel>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub support_material: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub ironing: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub required_resin_ml: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub profile: Option<String>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum PrinterModel {
//     Mk3,
//     Mk3s,
//     Mini,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct FirmwareFileInfo {
//     pub name: String,
//     pub read_only: bool,
//     pub m_timestamp: i64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub size: Option<i64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub display_name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub refs: Option<FileRefs>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub meta: Option<FirmwareMetadata>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct FirmwareMetadata {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub version: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub printer_type: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub printer_version: Option<i32>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct FolderInfo {
//     pub name: String,
//     pub read_only: bool,
//     pub m_timestamp: i64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub display_name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub children: Option<Vec<FileInfo>>,
// }
//
// // Camera structures
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Camera {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub camera_id: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub config: Option<CameraBasicConfig>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub connected: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub detected: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub stored: Option<bool>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub linked: Option<bool>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CameraBasicConfig {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub path: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub driver: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub resolution: Option<String>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CameraConfig {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub trigger_scheme: Option<TriggerScheme>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub available_resolutions: Option<Vec<Resolution>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub resolution: Option<Resolution>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub focus: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub capabilities: Option<Vec<CameraCapability>>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum TriggerScheme {
//     TenSec,
//     ThirtySec,
//     SixtySec,
//     EachLayer,
//     FifthLayer,
//     Manual,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum CameraCapability {
//     TriggerScheme,
//     Imaging,
//     Resolution,
//     Rotation,
//     Exposure,
//     Focus,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Resolution {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub width: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub height: Option<i32>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CameraConfigSet {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub name: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub trigger_scheme: Option<TriggerScheme>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub resolution: Option<Resolution>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub rotation: Option<i32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub focus: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub exposure: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub send_to_connect: Option<bool>,
// }
//
// // Update structures
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PrusaLinkPackage {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub new_version: Option<String>,
// }
//
// // Error structures
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Error {
//     pub title: String,
//     pub text: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub code: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub url: Option<String>,
// }
