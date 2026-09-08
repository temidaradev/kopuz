use super::macros::struct_conversion;
use super::*;
use crate::*;

pub fn intent_to_proto(value: &api::Intent) -> Intent {
    let kind = match value {
        api::Intent::Stopped => intent::Kind::Stopped(Unit {}),
        api::Intent::Loading { token, from_token } => intent::Kind::Loading(intent::Loading {
            token: *token,
            from_token: *from_token,
        }),
        api::Intent::Committed { token } => {
            intent::Kind::Committed(intent::Committed { token: *token })
        }
    };
    Intent { kind: Some(kind) }
}

pub fn intent_from_proto(value: Option<&Intent>) -> api::Intent {
    match value.and_then(|intent| intent.kind.as_ref()) {
        Some(intent::Kind::Loading(loading)) => api::Intent::Loading {
            token: loading.token,
            from_token: loading.from_token,
        },
        Some(intent::Kind::Committed(committed)) => api::Intent::Committed {
            token: committed.token,
        },
        Some(intent::Kind::Stopped(_)) | None => api::Intent::Stopped,
    }
}

pub fn now_playing_to_proto(value: &api::NowPlaying) -> NowPlaying {
    NowPlaying {
        key: value.key.clone(),
        uid: value.uid.clone(),
        title: value.title.clone(),
        artist: value.artist.clone(),
        album: value.album.clone(),
        duration_ms: value.duration_ms,
        khz: value.khz,
        bitrate: u32::from(value.bitrate),
        kind: track_kind_to_proto(value.kind) as i32,
        seekable: value.seekable,
    }
}

pub fn now_playing_from_proto(value: &NowPlaying) -> api::NowPlaying {
    api::NowPlaying {
        key: value.key.clone(),
        uid: value.uid.clone(),
        title: value.title.clone(),
        artist: value.artist.clone(),
        album: value.album.clone(),
        duration_ms: value.duration_ms,
        khz: value.khz,
        bitrate: value.bitrate.min(u32::from(u16::MAX)) as u16,
        kind: track_kind_from_proto(value.kind),
        seekable: value.seekable,
    }
}

struct_conversion!(
    anchor_to_proto,
    anchor_from_proto,
    api::PositionAnchor,
    PositionAnchor,
    copy { ms, at_ms, playing },
    clone {}
);

struct_conversion!(
    buffered_to_proto,
    buffered_from_proto,
    api::BufferedRange,
    BufferedRange,
    copy { start, end, total },
    clone {}
);

pub fn queue_summary_to_proto(value: &api::QueueSummary) -> QueueSummary {
    QueueSummary {
        rev: value.rev,
        length: value.length,
        index: value.index,
        shuffle: value.shuffle,
        r#loop: loop_to_proto(value.loop_mode) as i32,
    }
}

pub fn queue_summary_from_proto(value: Option<&QueueSummary>) -> api::QueueSummary {
    let value = value.cloned().unwrap_or_default();
    api::QueueSummary {
        rev: value.rev,
        length: value.length,
        index: value.index,
        shuffle: value.shuffle,
        loop_mode: loop_from_proto(value.r#loop),
    }
}

pub fn player_state_to_proto(value: &api::PlayerState) -> PlayerState {
    PlayerState {
        rev: value.rev,
        now_ms: value.now_ms,
        phase: phase_to_proto(value.phase) as i32,
        intent: Some(intent_to_proto(&value.intent)),
        track: value.track.as_ref().map(now_playing_to_proto),
        position: value.position.as_ref().map(anchor_to_proto),
        queue: Some(queue_summary_to_proto(&value.queue)),
        volume: value.volume,
        buffered: value.buffered.iter().map(buffered_to_proto).collect(),
        fading: value.fading.as_ref().map(|fading| FadingState {
            from_token: fading.from_token,
            track: Some(now_playing_to_proto(&fading.track)),
            position_ms: fading.position_ms,
        }),
        external: value.external.as_ref().map(|external| ExternalPlayback {
            kind: external.kind.clone(),
            device: external.device.clone(),
        }),
        error: value.error.as_ref().map(error_body_to_proto),
        output_latency_ms: value.output_latency_ms,
    }
}

pub fn player_state_from_proto(value: &PlayerState) -> api::PlayerState {
    api::PlayerState {
        rev: value.rev,
        now_ms: value.now_ms,
        phase: phase_from_proto(value.phase),
        intent: intent_from_proto(value.intent.as_ref()),
        track: value.track.as_ref().map(now_playing_from_proto),
        position: value.position.as_ref().map(anchor_from_proto),
        queue: queue_summary_from_proto(value.queue.as_ref()),
        volume: value.volume,
        buffered: value.buffered.iter().map(buffered_from_proto).collect(),
        fading: value.fading.as_ref().map(|fading| api::FadingState {
            from_token: fading.from_token,
            track: fading
                .track
                .as_ref()
                .map(now_playing_from_proto)
                .unwrap_or_default(),
            position_ms: fading.position_ms,
        }),
        external: value
            .external
            .as_ref()
            .map(|external| api::ExternalPlayback {
                kind: external.kind.clone(),
                device: external.device.clone(),
            }),
        error: value.error.as_ref().map(error_body_from_proto),
        output_latency_ms: value.output_latency_ms,
    }
}

pub fn command_to_proto(value: &api::PlayerCommand) -> PlayerCommand {
    let cmd = match value {
        api::PlayerCommand::Play => player_command::Cmd::Play(Unit {}),
        api::PlayerCommand::Pause => player_command::Cmd::Pause(Unit {}),
        api::PlayerCommand::Toggle => player_command::Cmd::Toggle(Unit {}),
        api::PlayerCommand::Next => player_command::Cmd::Next(Unit {}),
        api::PlayerCommand::Previous => player_command::Cmd::Previous(Unit {}),
        api::PlayerCommand::Stop => player_command::Cmd::Stop(Unit {}),
        api::PlayerCommand::Seek { position_ms } => player_command::Cmd::Seek(Seek {
            position_ms: *position_ms,
        }),
        api::PlayerCommand::SetVolume { volume } => {
            player_command::Cmd::Volume(SetVolume { volume: *volume })
        }
        api::PlayerCommand::SetMode { shuffle, loop_mode } => player_command::Cmd::Mode(SetMode {
            shuffle: *shuffle,
            r#loop: loop_mode.map(|mode| loop_to_proto(mode) as i32),
        }),
    };
    PlayerCommand { cmd: Some(cmd) }
}

pub fn command_from_proto(value: &PlayerCommand) -> Option<api::PlayerCommand> {
    Some(match value.cmd.as_ref()? {
        player_command::Cmd::Play(_) => api::PlayerCommand::Play,
        player_command::Cmd::Pause(_) => api::PlayerCommand::Pause,
        player_command::Cmd::Toggle(_) => api::PlayerCommand::Toggle,
        player_command::Cmd::Next(_) => api::PlayerCommand::Next,
        player_command::Cmd::Previous(_) => api::PlayerCommand::Previous,
        player_command::Cmd::Stop(_) => api::PlayerCommand::Stop,
        player_command::Cmd::Seek(seek) => api::PlayerCommand::Seek {
            position_ms: seek.position_ms,
        },
        player_command::Cmd::Volume(volume) => api::PlayerCommand::SetVolume {
            volume: volume.volume,
        },
        player_command::Cmd::Mode(mode) => api::PlayerCommand::SetMode {
            shuffle: mode.shuffle,
            loop_mode: mode.r#loop.map(loop_from_proto),
        },
    })
}

struct_conversion!(
    external_playback_to_proto,
    external_playback_from_proto,
    api::ExternalPlayback,
    ExternalPlayback,
    copy {},
    clone { kind, device }
);

struct_conversion!(
    external_lease_to_proto,
    external_lease_from_proto,
    api::ExternalPlaybackLease,
    ExternalPlaybackLease,
    copy { expires_in_ms },
    clone { lease_id }
);

pub fn external_report_to_proto(value: &api::ExternalPlaybackReport) -> ExternalPlaybackReport {
    ExternalPlaybackReport {
        lease_id: value.lease_id.clone(),
        track: value.track.as_ref().map(track_info_to_proto),
        position_ms: value.position_ms,
        playing: value.playing,
        completed: value.completed,
        device: value.device.clone(),
    }
}

pub fn external_report_from_proto(value: &ExternalPlaybackReport) -> api::ExternalPlaybackReport {
    api::ExternalPlaybackReport {
        lease_id: value.lease_id.clone(),
        track: value.track.as_ref().map(track_info_from_proto),
        position_ms: value.position_ms,
        playing: value.playing,
        completed: value.completed,
        device: value.device.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::super::fixtures::sample_state;
    use super::*;

    #[test]
    fn player_state_round_trips() {
        let state = sample_state();
        let back = player_state_from_proto(&player_state_to_proto(&state));
        assert_eq!(state, back);
    }

    #[test]
    fn external_playback_dtos_round_trip() {
        let playback = api::ExternalPlayback {
            kind: "spotify".into(),
            device: Some("device".into()),
        };
        assert_eq!(
            playback,
            external_playback_from_proto(&external_playback_to_proto(&playback))
        );

        let lease = api::ExternalPlaybackLease {
            lease_id: "lease".into(),
            expires_in_ms: 15_000,
        };
        assert_eq!(
            lease,
            external_lease_from_proto(&external_lease_to_proto(&lease))
        );

        let report = api::ExternalPlaybackReport {
            lease_id: "lease".into(),
            track: Some(api::TrackInfo {
                key: "track".into(),
                service: Some(api::MusicService::Spotify),
                ..Default::default()
            }),
            position_ms: 42,
            playing: true,
            completed: false,
            device: Some("device".into()),
        };
        assert_eq!(
            report,
            external_report_from_proto(&external_report_to_proto(&report))
        );
    }

    #[test]
    fn every_command_round_trips() {
        for command in [
            api::PlayerCommand::Play,
            api::PlayerCommand::Pause,
            api::PlayerCommand::Toggle,
            api::PlayerCommand::Next,
            api::PlayerCommand::Previous,
            api::PlayerCommand::Stop,
            api::PlayerCommand::Seek { position_ms: 1_200 },
            api::PlayerCommand::SetVolume { volume: 0.4 },
            api::PlayerCommand::SetMode {
                shuffle: Some(true),
                loop_mode: Some(api::LoopMode::Track),
            },
        ] {
            let back = command_from_proto(&command_to_proto(&command)).expect("command survives");
            assert_eq!(command, back);
        }
    }
}
