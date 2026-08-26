use super::macros::struct_conversion;
use super::*;
use crate::*;

struct_conversion!(
    favorites_to_proto,
    favorites_from_proto,
    api::FavoritesView,
    Favorites,
    copy { generation },
    clone { refs }
);

pub fn job_status_to_proto(value: &api::JobStatus) -> JobStatus {
    JobStatus {
        id: value.id.clone(),
        kind: job_kind_to_proto(value.kind) as i32,
        state: job_state_to_proto(value.state) as i32,
        phase: value.phase.clone(),
        current: value.current,
        total: value.total,
        message: value.message.clone(),
        error: value.error.as_ref().map(error_body_to_proto),
        request: value.request.clone(),
        title: value.title.clone(),
        format: value.format.clone(),
        speed: value.speed.clone(),
        eta: value.eta.clone(),
    }
}

pub fn job_status_from_proto(value: &JobStatus) -> api::JobStatus {
    api::JobStatus {
        id: value.id.clone(),
        kind: job_kind_from_proto(value.kind),
        state: job_state_from_proto(value.state),
        phase: value.phase.clone(),
        current: value.current,
        total: value.total,
        message: value.message.clone(),
        error: value.error.as_ref().map(error_body_from_proto),
        request: value.request.clone(),
        title: value.title.clone(),
        format: value.format.clone(),
        speed: value.speed.clone(),
        eta: value.eta.clone(),
    }
}

pub fn config_view_to_proto(value: &api::ConfigView) -> ConfigView {
    ConfigView {
        config_json: value.config.to_string(),
        locked_keys: value.locked_keys.clone(),
    }
}

pub fn config_view_from_proto(value: &ConfigView) -> api::ConfigView {
    api::ConfigView {
        config: serde_json::from_str(&value.config_json).unwrap_or(serde_json::Value::Null),
        locked_keys: value.locked_keys.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn favorites_jobs_and_config_round_trip() {
        let favorites = api::FavoritesView {
            refs: vec!["key".into()],
            generation: 8,
        };
        assert_eq!(
            favorites,
            favorites_from_proto(&favorites_to_proto(&favorites))
        );

        let job = api::JobStatus {
            id: "job".into(),
            kind: api::JobKind::LibrarySync,
            state: api::JobState::Failed,
            phase: "done".into(),
            current: None,
            total: Some(5),
            message: None,
            error: Some(api::ErrorBody {
                code: api::ErrorCode::Internal,
                message: "failed".into(),
            }),
            request: Some("https://example.com/watch".into()),
            title: Some("Track".into()),
            format: Some("m4a".into()),
            speed: Some("1.2MiB/s".into()),
            eta: Some("00:12".into()),
        };
        assert_eq!(job, job_status_from_proto(&job_status_to_proto(&job)));

        let config = api::ConfigView {
            config: serde_json::json!({"volume": 0.5}),
            locked_keys: vec!["theme".into()],
        };
        assert_eq!(
            config,
            config_view_from_proto(&config_view_to_proto(&config))
        );
    }
}
