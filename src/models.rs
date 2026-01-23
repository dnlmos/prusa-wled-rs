use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// ENDPOINT CONSTANTS
// ============================================================================

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
    pub api: Option<String>,
    pub server: Option<String>,
    pub original: Option<String>,
    pub text: Option<String>,
    pub hostname: Option<String>,
    pub firmware: Option<String>,
    pub sdk: Option<String>,
    pub python: Option<Vec<PythonPackage>>,
    pub system: Option<SystemInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonPackage {
    pub name: Option<String>,
    pub version: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub python: Option<String>,
    #[serde(rename = "DESCRIPTION")]
    pub description: Option<String>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "OS")]
    pub os: Option<String>,
}

// ============================================================================
// CONNECTION ENDPOINT TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionResponse {
    pub current: Option<ConnCurrent>,
    pub options: Option<ConnOptions>,
    pub connect: Option<ConnConfig>,
    pub states: Option<ConnStates>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnCurrent {
    pub baudrate: Option<u32>,
    pub port: Option<String>,
    #[serde(rename = "printerProfile")]
    pub printer_profile: Option<String>,
    pub state: Option<State>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnOptions {
    pub ports: Option<Vec<String>>,
    pub baudrates: Option<Vec<u32>>,
    #[serde(rename = "printerProfiles")]
    pub printer_profiles: Option<Vec<PrinterProfileOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterProfileOption {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnConfig {
    pub hostname: Option<String>,
    pub port: Option<u32>,
    pub tls: Option<bool>,
    pub registration: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnStates {
    pub printer: Option<ConnectionState>,
    pub connect: Option<ConnectionState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    pub ok: Option<bool>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRequest {
    pub command: Option<String>,
    pub baudrate: Option<u32>,
    pub port: Option<String>,
    #[serde(rename = "printerProfile")]
    pub printer_profile: Option<String>,
    pub connect: Option<ConnConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionResponse200 {
    pub url: Option<String>,
}

// ============================================================================
// PRINTER PROFILES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterProfilesResponse {
    pub profiles: Option<HashMap<String, PrinterProfile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterProfile {
    pub id: Option<String>,
    pub name: Option<String>,
    pub model: Option<String>,
    pub color: Option<String>,
    pub current: Option<bool>,
    pub default: Option<bool>,
    #[serde(rename = "heatedBed")]
    pub heated_bed: Option<bool>,
    #[serde(rename = "heatedChamber")]
    pub heated_chamber: Option<bool>,
    pub extruder: Option<ExtruderInfo>,
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtruderInfo {
    pub count: Option<u32>,
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
    pub temperature: Option<TemperatureState>,
    pub sd: Option<SDState>,
    pub state: Option<PrinterState>,
    pub telemetry: Option<serde_json::Value>, // Can be FDM, Mini, or SL1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureState {
    pub tool0: Option<TemperatureData>,
    pub bed: Option<TemperatureData>,
    pub chamber: Option<TemperatureData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureData {
    pub actual: Option<f64>,
    pub target: Option<f64>,
    pub offset: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDState {
    pub ready: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterState {
    pub text: Option<State>,
    pub flags: Option<PrinterFlags>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterFlags {
    pub operational: Option<bool>,
    pub paused: Option<bool>,
    pub printing: Option<bool>,
    pub cancelling: Option<bool>,
    pub pausing: Option<bool>,
    #[serde(rename = "sdReady")]
    pub sd_ready: Option<bool>,
    pub error: Option<bool>,
    pub ready: Option<bool>,
    #[serde(rename = "closedOrError")]
    pub closed_or_error: Option<bool>,
    pub finished: Option<bool>,
    pub prepared: Option<bool>,
    pub busy: Option<bool>,
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
    pub temp_bed: Option<f64>,
    pub temp_nozzle: Option<f64>,
    pub material: Option<String>,
    #[serde(rename = "z-height")]
    pub z_height: Option<f64>,
    #[serde(rename = "print-speed")]
    pub print_speed: Option<u32>,
    pub axis_x: Option<f64>,
    pub axis_y: Option<f64>,
    pub axis_z: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryMini {
    pub temp_bed: Option<f64>,
    pub temp_nozzle: Option<f64>,
    pub material: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySL1 {
    #[serde(rename = "fanUvLed")]
    pub fan_uv_led: Option<u32>,
    #[serde(rename = "fanBlower")]
    pub fan_blower: Option<u32>,
    #[serde(rename = "fanRear")]
    pub fan_rear: Option<u32>,
    #[serde(rename = "coverClosed")]
    pub cover_closed: Option<bool>,
    #[serde(rename = "tempAmbient")]
    pub temp_ambient: Option<f64>,
    #[serde(rename = "tempCpu")]
    pub temp_cpu: Option<f64>,
    #[serde(rename = "tempUvLed")]
    pub temp_uv_led: Option<f64>,
}

// ============================================================================
// PRINTHEAD CONTROL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintheadRequest {
    pub jog: Option<JogCommand>,
    pub home: Option<HomeCommand>,
    pub speed: Option<SpeedCommand>,
    pub feedrate: Option<FeedrateCommand>,
    pub disable_steppers: Option<DisableSteppersCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JogCommand {
    pub command: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeCommand {
    pub command: Option<String>,
    pub axes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedCommand {
    pub command: Option<String>,
    pub factor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedrateCommand {
    pub command: Option<String>,
    pub factor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableSteppersCommand {
    pub command: Option<String>,
}

// ============================================================================
// TOOL CONTROL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    pub target: Option<TargetCommand>,
    pub offset: Option<OffsetCommand>,
    pub select: Option<SelectCommand>,
    pub extrude: Option<ExtrudeCommand>,
    pub retract: Option<RetractCommand>,
    pub flowrate: Option<FlowrateCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetCommand {
    pub command: Option<String>,
    pub targets: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffsetCommand {
    pub command: Option<String>,
    pub offsets: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectCommand {
    pub command: Option<String>,
    pub tool: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtrudeCommand {
    pub command: Option<String>,
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetractCommand {
    pub command: Option<String>,
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowrateCommand {
    pub command: Option<String>,
    pub amount: Option<f64>,
}

// ============================================================================
// BED CONTROL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedRequest {
    pub command: Option<String>,
    pub target: Option<f64>,
}

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: Option<String>,
    pub title: String,
    pub text: String,
    pub url: Option<String>,
}

// ============================================================================
// JOB TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResponse {
    pub job: Option<Job>,
    pub progress: Option<Progress>,
    pub state: Option<State>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "estimatedPrintTime")]
    pub estimated_print_time: Option<u32>,
    #[serde(rename = "averagePrintTime")]
    pub average_print_time: Option<u32>,
    #[serde(rename = "lastPrintTime")]
    pub last_print_time: Option<u32>,
    pub filament: Option<String>,
    pub file: Option<JobFile>,
    pub user: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value, // For FDM/SLA specific fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobFile {
    pub name: Option<String>,
    pub path: Option<String>,
    pub origin: Option<String>,
    pub date: Option<u64>,
    pub size: Option<u64>,
    pub display: Option<String>,
    pub refs: Option<FileRefs>,
    pub layers: Option<u32>,
    #[serde(rename = "layerHeight")]
    pub layer_height: Option<f64>,
    #[serde(rename = "exposureTime")]
    pub exposure_time: Option<u32>,
    #[serde(rename = "exposureTimeFirst")]
    pub exposure_time_first: Option<f64>,
    #[serde(rename = "exposureTimeCalibration")]
    pub exposure_time_calibration: Option<f64>,
    #[serde(rename = "exposureUserProfile")]
    pub exposure_user_profile: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRefs {
    pub resource: Option<String>,
    #[serde(rename = "thumbnailBig")]
    pub thumbnail_big: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub completion: Option<f64>,
    pub filepos: Option<u64>,
    #[serde(rename = "printTime")]
    pub print_time: Option<u32>,
    #[serde(rename = "printTimeLeft")]
    pub print_time_left: Option<u32>,
    #[serde(flatten)]
    pub extra: serde_json::Value, // For printer-specific fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMK3 {
    pub pos_z_mm: Option<f64>,
    #[serde(rename = "printSpeed")]
    pub print_speed: Option<u32>,
    pub flow_factor: Option<u32>,
    pub completion: Option<f64>,
    pub filepos: Option<u64>,
    #[serde(rename = "printTime")]
    pub print_time: Option<u32>,
    #[serde(rename = "printTimeLeft")]
    pub print_time_left: Option<u32>,
    #[serde(rename = "printTimeLeftOrigin")]
    pub print_time_left_origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMini {
    pub pos_z_mm: Option<f64>,
    #[serde(rename = "printSpeed")]
    pub print_speed: Option<f64>,
    pub flow_factor: Option<f64>,
    pub filament_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressSL1 {
    #[serde(rename = "currentLayer")]
    pub current_layer: Option<u32>,
    pub completion: Option<f64>,
    #[serde(rename = "printTime")]
    pub print_time: Option<u32>,
    #[serde(rename = "printTimeLeft")]
    pub print_time_left: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRequest {
    pub command: Option<JobCommand>,
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
    pub core: Option<Vec<serde_json::Value>>,
    pub custom: Option<Vec<SystemCommand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCommand {
    pub action: Option<String>,
    pub name: Option<String>,
    pub confirm: Option<String>,
    pub source: Option<String>,
    pub resource: Option<String>,
}

// ============================================================================
// CUSTOM SYSTEM COMMANDS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeExposureRequest {
    #[serde(rename = "exposureTime")]
    pub exposure_time: Option<i32>,
    #[serde(rename = "exposureTimeFirst")]
    pub exposure_time_first: Option<f64>,
    #[serde(rename = "exposureTimeCalibration")]
    pub exposure_time_calibration: Option<f64>,
    #[serde(rename = "exposureUserProfile")]
    pub exposure_user_profile: Option<f64>,
}

// ============================================================================
// ACCESS / USERS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersResponse {
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub active: Option<bool>,
    pub admin: Option<bool>,
    pub apikey: Option<String>,
    pub name: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub user: Option<bool>,
}

// ============================================================================
// SETTINGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsResponse {
    #[serde(rename = "api-key")]
    pub api_key: Option<String>,
    pub printer: Option<PrinterSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterSettings {
    pub name: Option<String>,
    pub location: Option<String>,
    pub farm_mode: Option<bool>,
    pub network_error_chime: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsRequest {
    pub printer: Option<PrinterSettingsUpdate>,
    pub user: Option<UserSettingsUpdate>,
    pub farm_mode: Option<bool>,
    pub network_error_chime: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterSettingsUpdate {
    pub name: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsUpdate {
    pub password: Option<String>,
    pub username: Option<String>,
    pub new_password: Option<String>,
    pub new_repassword: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsErrorResponse {
    pub errors: Option<SettingsErrors>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsErrors {
    pub printer: Option<PrinterErrors>,
    pub user: Option<UserErrors>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterErrors {
    pub missing_credentials: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserErrors {
    pub username: Option<bool>,
    pub password: Option<bool>,
    pub repassword: Option<bool>,
    pub old_digest: Option<bool>,
    pub same_digest: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SNRequest {
    pub serial: Option<String>,
}

// ============================================================================
// FILE OPERATIONS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub origin: Option<String>,
    pub name: Option<String>,
    pub path: Option<String>,
    #[serde(rename = "type")]
    pub file_type: Option<String>,
    #[serde(rename = "typePath")]
    pub type_path: Option<Vec<String>>,
    pub refs: Option<FileRefs>,
    pub size: Option<u64>,
    pub date: Option<u64>,
    #[serde(rename = "gcodeAnalysis")]
    pub gcode_analysis: Option<GcodeAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcodeAnalysis {
    #[serde(rename = "estimatedPrintTime")]
    pub estimated_print_time: Option<u32>,
    pub material: Option<String>,
    #[serde(rename = "layerHeight")]
    pub layer_height: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub origin: Option<String>,
    pub path: Option<String>,
    pub display: Option<String>,
    pub name: Option<String>,
    pub size: Option<u64>,
    #[serde(rename = "type")]
    pub file_type: Option<String>,
    #[serde(rename = "typePath")]
    pub type_path: Option<Vec<String>>,
    pub refs: Option<FileRefsDetailed>,
    #[serde(rename = "gcodeAnalysis")]
    pub gcode_analysis: Option<GcodeAnalysisDetailed>,
    pub hash: Option<String>,
    pub date: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRefsDetailed {
    pub resource: Option<String>,
    pub download: Option<String>,
    #[serde(rename = "thumbnailSmall")]
    pub thumbnail_small: Option<String>,
    #[serde(rename = "thumbnailBig")]
    pub thumbnail_big: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcodeAnalysisDetailed {
    #[serde(rename = "estimatedPrintTime")]
    pub estimated_print_time: Option<u32>,
    pub material: Option<String>,
    #[serde(rename = "layerHeight")]
    pub layer_height: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    pub origin: Option<String>,
    pub path: Option<String>,
    pub display: Option<String>,
    pub name: Option<String>,
    pub size: Option<u64>,
    #[serde(rename = "type")]
    pub folder_type: Option<String>,
    #[serde(rename = "typePath")]
    pub type_path: Option<Vec<String>>,
    pub refs: Option<FolderRefs>,
    pub children: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderRefs {
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllFilesInfo {
    pub files: Option<Vec<serde_json::Value>>,
    pub free: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSelectRequest {
    pub command: Option<String>,
    pub print: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCommandRequest {
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
    #[serde(rename = "type")]
    pub transfer_type: Option<String>,
    pub url: Option<String>,
    pub target: Option<String>,
    pub destination: Option<String>,
    pub path: Option<String>,
    pub size: Option<String>,
    pub start_time: Option<String>,
    pub progress: Option<String>,
    pub remaining_time: Option<String>,
    pub to_select: Option<bool>,
    pub to_print: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploaded {
    pub done: Option<bool>,
    pub files: Option<HashMap<String, FileUploadedInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadedInfo {
    pub name: Option<String>,
    pub origin: Option<String>,
    pub path: Option<String>,
    pub refs: Option<FileRefsDetailed>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderCreated {
    pub done: Option<bool>,
    pub folder: Option<FolderCreatedInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderCreatedInfo {
    pub name: Option<String>,
    pub origin: Option<String>,
    pub path: Option<String>,
    pub refs: Option<FolderRefs>,
}

// ============================================================================
// LOGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogsResponse {
    pub files: Option<Vec<LogFile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFile {
    pub name: Option<String>,
    pub date: Option<u64>,
    pub size: Option<u64>,
    pub refs: Option<LogRefs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRefs {
    pub download: Option<String>,
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: Option<String>,
    pub title: String,
    pub text: String,
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
