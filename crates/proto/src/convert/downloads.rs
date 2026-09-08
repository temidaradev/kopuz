//! Offline downloads and yt-dlp jobs.

use super::*;
use crate::*;

pub fn download_status_to_proto(value: &api::DownloadItemStatus) -> DownloadItemStatus {
    DownloadItemStatus {
        key: value.key.clone(),
        state: download_item_state_to_proto(value.state) as i32,
        bytes_done: value.bytes_done,
        total_bytes: value.total_bytes,
        error: value.error.clone(),
    }
}

pub fn download_status_from_proto(value: &DownloadItemStatus) -> api::DownloadItemStatus {
    api::DownloadItemStatus {
        key: value.key.clone(),
        state: download_item_state_from_proto(value.state),
        bytes_done: value.bytes_done,
        total_bytes: value.total_bytes,
        error: value.error.clone(),
    }
}

pub fn ytdlp_request_to_proto(value: &api::YtdlpRequest) -> YtdlpRequest {
    YtdlpRequest {
        url: value.url.clone(),
        output_dir: value.output_dir.clone(),
        format: ytdlp_format_to_proto(value.format) as i32,
        options_json: value.options.to_string(),
    }
}

pub fn ytdlp_request_from_proto(value: &YtdlpRequest) -> Result<api::YtdlpRequest, api::ApiError> {
    Ok(api::YtdlpRequest {
        url: value.url.clone(),
        output_dir: value.output_dir.clone(),
        format: ytdlp_format_from_proto(value.format),
        options: serde_json::from_str(&value.options_json).map_err(|error| {
            api::ApiError::invalid_input(format!("invalid yt-dlp options JSON: {error}"))
        })?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_status_and_ytdlp_request_round_trip() {
        let download = api::DownloadItemStatus {
            key: "track".into(),
            state: api::DownloadItemState::Downloading,
            bytes_done: 512,
            total_bytes: Some(1024),
            error: None,
        };
        assert_eq!(
            download,
            download_status_from_proto(&download_status_to_proto(&download))
        );

        let ytdlp = api::YtdlpRequest {
            url: "https://example.com/watch".into(),
            output_dir: "/tmp/music".into(),
            format: api::YtdlpAudioFormat::M4a,
            options: serde_json::json!({"embed_metadata": true}),
        };
        assert_eq!(
            ytdlp,
            ytdlp_request_from_proto(&ytdlp_request_to_proto(&ytdlp)).expect("yt-dlp request")
        );
    }
}
