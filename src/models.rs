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
    pub api: String,
    pub server: String,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub python: Vec<PythonPackage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonPackage {
    pub name: String,
    pub version: String,
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
    pub current: ConnCurrent,
    pub options: ConnOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<ConnConfig>,
    pub states: ConnStates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnCurrent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baudrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "printerProfile")]
    pub printer_profile: String,
    pub state: State,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnOptions {
    pub ports: Vec<String>,
    pub baudrates: Vec<u32>,
    #[serde(rename = "printerProfiles")]
    pub printer_profiles: Vec<PrinterProfileOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterProfileOption {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnConfig {
    pub hostname: String,
    #[serde(default)]
    pub port: u32,
    #[serde(default = "default_true")]
    pub tls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnStates {
    pub printer: ConnectionState,
    pub connect: ConnectionState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    pub ok: bool,
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
    pub url: String,
}

// ============================================================================
// PRINTER PROFILES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterProfilesResponse {
    #[serde(default)]
    pub profiles: HashMap<String, PrinterProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterProfile {
    pub id: String,
    pub name: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default)]
    pub current: bool,
    #[serde(default)]
    pub default: bool,
    #[serde(rename = "heatedBed", default)]
    pub heated_bed: bool,
    #[serde(rename = "heatedChamber", default)]
    pub heated_chamber: bool,
    pub extruder: ExtruderInfo,
    pub resource: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtruderInfo {
    pub count: u32,
    #[serde(default)]
    pub offsets: Vec<Vec<f64>>,
}

// ============================================================================
// PRINTER STATE TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub temperature: TemperatureState,
    pub sd: SDState,
    pub state: PrinterState,
    pub telemetry: TelemetryFDM,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub telemetry: Option<TelemetryFDM>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(untagged)]
pub enum Telemetry {
    Fdm(TelemetryFDM),
    Mini(TelemetryMini),
    Sl1(TelemetrySL1),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureState {
    pub tool0: TemperatureData,
    pub bed: TemperatureData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chamber: Option<TemperatureData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureData {
    pub actual: f64,
    pub target: f64, // default 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDState {
    pub ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterState {
    pub text: State,
    pub flags: PrinterFlags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterFlags {
    pub operational: bool,
    pub paused: bool,
    pub printing: bool,
    pub cancelling: bool,
    pub pausing: bool,
    #[serde(rename = "sdReady")]
    pub sd_ready: bool,
    pub error: bool,
    pub ready: bool,
    #[serde(rename = "closedOrError")]
    pub closed_or_error: bool,
    pub finished: bool,
    pub prepared: bool,
    #[serde(default)]
    pub busy: bool,
    #[serde(default)]
    pub link_state: LinkState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LinkState {
    #[default]
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
    #[serde(rename = "temp-bed")]
    pub temp_bed: f64,
    #[serde(rename = "temp-nozzle")]
    pub temp_nozzle: f64,
    #[serde(skip_serializing_if = "Option::is_none")] // if nothing, then " - ",
    pub material: Option<String>,
    #[serde(rename = "z-height")]
    pub z_height: f64,
    #[serde(rename = "print-speed")]
    pub print_speed: u32,
    pub axis_x: Option<f64>,
    pub axis_y: Option<f64>,
    pub axis_z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryMini {
    pub temp_bed: f64,
    pub temp_nozzle: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySL1 {
    #[serde(rename = "fanUvLed")]
    pub fan_uv_led: u32,
    #[serde(rename = "fanBlower")]
    pub fan_blower: u32,
    #[serde(rename = "fanRear")]
    pub fan_rear: u32,
    #[serde(rename = "coverClosed")]
    pub cover_closed: bool,
    #[serde(rename = "tempAmbient")]
    pub temp_ambient: f64,
    #[serde(rename = "tempCpu")]
    pub temp_cpu: f64,
    #[serde(rename = "tempUvLed")]
    pub temp_uv_led: f64,
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
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeCommand {
    pub command: String,
    pub axes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedCommand {
    pub command: String,
    pub factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedrateCommand {
    pub command: String,
    pub factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableSteppersCommand {
    pub command: String,
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
    pub command: String,
    pub target: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffsetCommand {
    pub command: String,
    pub offset: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectCommand {
    pub command: String,
    pub tool: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtrudeCommand {
    pub command: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetractCommand {
    pub command: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowrateCommand {
    pub command: String,
    pub factor: u32,
}

// ============================================================================
// BED CONTROL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedRequest {
    pub command: String,
    pub target: f64,
}

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u32,
    pub title: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// ============================================================================
// JOB TYPES - POLYMORPHIC WITH ENUM
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResponse {
    pub job: Job,
    pub progress: Progress,
    pub state: String,
}

// --- JOB SECTION ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Job {
    Fdm(JobFdm),
    Sla(JobSla),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobFdm {
    #[serde(flatten)]
    pub common: JobCommon,
    pub filament: Option<f64>, // Changed to f64 just in case
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSla {
    #[serde(flatten)]
    pub common: JobCommon,
    pub resin: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCommon {
    #[serde(rename = "estimatedPrintTime", default)]
    pub estimated_print_time: Option<f64>,
    #[serde(rename = "averagePrintTime", default)]
    pub average_print_time: Option<f64>,
    #[serde(rename = "lastPrintTime", default)]
    pub last_print_time: Option<f64>,
    pub file: JobFile,
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobFile {
    pub name: Option<String>,
    pub path: Option<String>,
    pub origin: Option<String>, // Changed to String to be safe
    pub date: Option<u64>,
    pub size: Option<u64>,
    pub display: Option<String>,
}

// --- PROGRESS SECTION ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Progress {
    Mk3(ProgressMK3),
    Mini(ProgressMini),
    Sl1(ProgressSL1),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressCommon {
    pub completion: Option<f64>,
    #[serde(default)]
    pub filepos: u64,
    #[serde(rename = "printTime")]
    pub print_time: Option<u32>,
    #[serde(rename = "printTimeLeft")]
    pub print_time_left: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMK3 {
    #[serde(flatten)]
    pub common: ProgressCommon,
    pub pos_z_mm: Option<f64>,
    #[serde(rename = "printSpeed")]
    pub print_speed: Option<f64>, // Use f64 for safety
    #[serde(rename = "flow_factor")]
    pub flow_factor: Option<f64>,
    #[serde(rename = "printTimeLeftOrigin")]
    pub print_time_left_origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMini {
    #[serde(flatten)]
    pub common: ProgressCommon,
    pub pos_z_mm: Option<f64>,
    #[serde(rename = "printSpeed")]
    pub print_speed: Option<f64>,
    #[serde(rename = "flow_factor")]
    pub flow_factor: Option<f64>,
    pub filament_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressSL1 {
    #[serde(flatten)]
    pub common: ProgressCommon,
    pub current_layer: Option<u32>,
}

// --- Supporting Types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileOrigin {
    Local,
    Sdcard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRefs {
    pub resource: String,
    pub download: Option<String>,
    pub thumbnail_small: Option<String>,
    pub thumbnail_big: Option<String>,
}

// ============================================================================
// SYSTEM COMMANDS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsResponse {
    #[serde(default)]
    pub core: Vec<serde_json::Value>,
    #[serde(default)]
    pub custom: Vec<SystemCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCommand {
    pub action: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<String>,
    pub source: String,
    pub resource: String,
}

// ============================================================================
// CUSTOM SYSTEM COMMANDS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeExposureRequest {
    pub time_ms: u32,
    pub initial_time_ms: u32,
    pub max_time_ms: u32,
    pub max_initial_time_ms: u32,
    pub min_time_ms: u32,
    pub min_initial_time_ms: u32,
}

// ============================================================================
// ACCESS / USERS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apikey: Option<String>,
    #[serde(default)]
    pub user: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

// ============================================================================
// SETTINGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub printer: PrinterSettings,
    pub user: UserSettings,
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
    pub nozzle_diameter: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_extrusion_temp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_error_chime: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printer: Option<PrinterSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterSettingsUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub farm_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nozzle_diameter: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_extrusion_temp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_error_chime: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsErrorResponse {
    pub errors: SettingsErrors,
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
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub farm_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserErrors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SNRequest {
    pub sn: String,
}

// ============================================================================
// FILE OPERATIONS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_print_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_height: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcodeAnalysis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_print_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_height: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub origin: FileOrigin,
    pub path: String,
    pub display: String,
    pub name: String,
    pub size: u64,
    #[serde(rename = "type")]
    pub file_type: String,
    pub type_path: Vec<String>,
    pub hash: String,
    pub date: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<FileRefs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcode_analysis: Option<GcodeAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    pub origin: FileOrigin,
    pub path: String,
    pub display: String,
    pub name: String,
    pub size: u64,
    #[serde(rename = "type")]
    pub folder_type: String,
    pub type_path: Vec<String>,
    pub refs: FileRefs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<FileOrFolder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileOrFolder {
    File(FileInfo),
    Folder(FolderInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllFilesInfo {
    pub files: Vec<FileOrFolder>,
    pub free: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSelectRequest {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCommandRequest {
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileCommandType {
    Select,
    Copy,
    Move,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRequest {
    pub path: String,
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
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadInfo {
    #[serde(rename = "type")]
    pub download_type: String,
    pub display_name: String,
    pub path: String,
    pub progress: f64,
    pub transferred: u64,
    pub time_transferring: u32,
    pub to_print: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_remaining: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploaded {
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderCreated {
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<serde_json::Value>,
}

// ============================================================================
// LOGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogsResponse {
    pub files: Vec<LogFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFile {
    pub name: String,
    pub date: u64,
    pub size: u64,
    pub refs: LogFileRefs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFileRefs {
    pub download: String,
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: u32,
    pub title: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn default_true() -> bool {
    true
}
