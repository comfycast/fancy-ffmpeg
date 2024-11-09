use std::{
    path::Path,
    process::{Command, ExitStatus},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FFProbe {
    pub streams: Vec<Stream>,
    pub format: Format,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stream {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Format {}

#[derive(Debug, thiserror::Error)]
pub enum FFProbeError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Process(ExitStatus),
}

pub fn ffprobe<P: AsRef<Path>>(path: P) -> Result<FFProbe, FFProbeError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
        ])
        .arg(path.as_ref())
        .output()?;

    if output.status.success() {
        return serde_json::from_slice(&output.stdout).map_err(FFProbeError::Json);
    }

    Err(FFProbeError::Process(output.status))
}
