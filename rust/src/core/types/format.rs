#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    /// Plain JSON
    Json,
    /// Json Lines
    /// https://jsonlines.org/
    #[default]
    JsonStream,
    /// Arrow IPC format
    Arrow,
    /// Arrow IPC Stream format
    ArrowStream,
}
