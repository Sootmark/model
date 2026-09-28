/// Shared, cross-artifact fields that hunts, Sigma mappings and pivots rely
/// on. Adapters fill what their artifact knows; everything else stays `None`.
/// The native fields of a record keep the full detail.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Facets {
    /// Host name as recorded by the artifact.
    pub host_name: Option<String>,
    /// Account name.
    pub user_name: Option<String>,
    /// Account SID.
    pub user_sid: Option<String>,
    /// Logon session identifier (e.g. EVTX `TargetLogonId`).
    pub logon_id: Option<String>,
    /// Windows logon type.
    pub logon_type: Option<u8>,
    /// Process image path.
    pub process_path: Option<String>,
    /// Process command line.
    pub process_command_line: Option<String>,
    /// Process id.
    pub process_id: Option<u64>,
    /// Parent process image path.
    pub parent_process_path: Option<String>,
    /// A file path the record is about.
    pub file_path: Option<String>,
    /// A Windows service name.
    pub service_name: Option<String>,
    /// A scheduled task name.
    pub task_name: Option<String>,
    /// Source IP address.
    pub source_ip: Option<String>,
    /// Destination IP address.
    pub destination_ip: Option<String>,
    /// Event code (EVTX event id).
    pub event_code: Option<u32>,
    /// Log channel (EVTX channel).
    pub channel: Option<String>,
    /// Log provider (EVTX provider name).
    pub provider: Option<String>,
}
