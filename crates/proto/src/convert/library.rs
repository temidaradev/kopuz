use super::macros::struct_conversion;
use super::*;
use crate::*;

struct_conversion!(
    track_filter_to_proto,
    track_filter_from_proto,
    api::TrackFilter,
    TrackFilter,
    copy { favorite },
    clone {
        search,
        artist,
        album,
        genre,
        sort
    }
);

pub fn page_to_proto(value: api::Page) -> Page {
    Page {
        offset: value.offset,
        limit: value.limit,
    }
}

pub fn page_from_proto(value: Option<&Page>) -> api::Page {
    let value = value.cloned().unwrap_or_default();
    api::Page {
        offset: value.offset,
        limit: if value.limit == 0 {
            api::DEFAULT_PAGE_LIMIT
        } else {
            value.limit
        },
    }
}

pub fn track_info_to_proto(value: &api::TrackInfo) -> TrackInfo {
    TrackInfo {
        key: value.key.clone(),
        uid: value.uid.clone(),
        title: value.title.clone(),
        artist: value.artist.clone(),
        album: value.album.clone(),
        album_id: value.album_id.clone(),
        duration_ms: value.duration_ms,
        khz: value.khz,
        bitrate: u32::from(value.bitrate),
        track_number: value.track_number,
        disc_number: value.disc_number,
        kind: track_kind_to_proto(value.kind) as i32,
        seekable: value.seekable,
        offline: value.offline,
        service: value
            .service
            .map(|service| music_service_to_proto(service) as i32),
        artists: value.artists.clone(),
        musicbrainz_release_id: value.musicbrainz_release_id.clone(),
        musicbrainz_recording_id: value.musicbrainz_recording_id.clone(),
        musicbrainz_track_id: value.musicbrainz_track_id.clone(),
        playlist_item_id: value.playlist_item_id.clone(),
        source: value.source.clone(),
    }
}

pub fn track_info_from_proto(value: &TrackInfo) -> api::TrackInfo {
    api::TrackInfo {
        key: value.key.clone(),
        uid: value.uid.clone(),
        title: value.title.clone(),
        artist: value.artist.clone(),
        album: value.album.clone(),
        album_id: value.album_id.clone(),
        duration_ms: value.duration_ms,
        khz: value.khz,
        bitrate: value.bitrate.min(u32::from(u16::MAX)) as u16,
        track_number: value.track_number,
        disc_number: value.disc_number,
        kind: track_kind_from_proto(value.kind),
        seekable: value.seekable,
        offline: value.offline,
        service: value.service.map(music_service_from_proto),
        artists: value.artists.clone(),
        musicbrainz_release_id: value.musicbrainz_release_id.clone(),
        musicbrainz_recording_id: value.musicbrainz_recording_id.clone(),
        musicbrainz_track_id: value.musicbrainz_track_id.clone(),
        playlist_item_id: value.playlist_item_id.clone(),
        source: value.source.clone(),
    }
}

pub fn track_page_to_proto(value: &api::TrackPage) -> TrackPage {
    TrackPage {
        total: value.total,
        offset: value.offset,
        items: value.items.iter().map(track_info_to_proto).collect(),
    }
}

pub fn track_page_from_proto(value: &TrackPage) -> api::TrackPage {
    api::TrackPage {
        total: value.total,
        offset: value.offset,
        items: value.items.iter().map(track_info_from_proto).collect(),
    }
}

pub fn lyrics_to_proto(value: &api::LyricsView) -> Lyrics {
    Lyrics {
        plain: value.plain.clone(),
        synced: value
            .synced
            .iter()
            .map(|line| LyricLine {
                start_ms: line.start_ms,
                end_ms: line.end_ms,
                text: line.text.clone(),
                chunks: line
                    .chunks
                    .iter()
                    .map(|chunk| LyricChunk {
                        start_ms: chunk.start_ms,
                        text: chunk.text.clone(),
                    })
                    .collect(),
                parent_line_index: line.parent_line_index,
                background: line.background,
                opposite_turn: line.opposite_turn,
            })
            .collect(),
    }
}

pub fn lyrics_from_proto(value: &Lyrics) -> api::LyricsView {
    api::LyricsView {
        plain: value.plain.clone(),
        synced: value
            .synced
            .iter()
            .map(|line| api::LyricLineView {
                start_ms: line.start_ms,
                end_ms: line.end_ms,
                text: line.text.clone(),
                chunks: line
                    .chunks
                    .iter()
                    .map(|chunk| api::LyricChunkView {
                        start_ms: chunk.start_ms,
                        text: chunk.text.clone(),
                    })
                    .collect(),
                parent_line_index: line.parent_line_index,
                background: line.background,
                opposite_turn: line.opposite_turn,
            })
            .collect(),
    }
}

struct_conversion!(
    stats_to_proto,
    stats_from_proto,
    api::StatsView,
    Stats,
    copy {},
    clone { listen_counts }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_rows_round_trip_with_optional_fields() {
        let track = api::TrackInfo {
            key: "key".into(),
            uid: "uid".into(),
            title: "title".into(),
            artist: "artist".into(),
            album: "album".into(),
            album_id: "album-id".into(),
            duration_ms: None,
            khz: 48,
            bitrate: 320,
            track_number: Some(2),
            disc_number: None,
            kind: api::TrackKind::Radio,
            seekable: false,
            offline: true,
            service: Some(api::MusicService::YtMusic),
            artists: vec!["artist".into(), "guest".into()],
            musicbrainz_release_id: Some("release".into()),
            musicbrainz_recording_id: None,
            musicbrainz_track_id: Some("track-id".into()),
            playlist_item_id: Some("entry".into()),
            source: "server-1".into(),
        };
        assert_eq!(track, track_info_from_proto(&track_info_to_proto(&track)));

        let page = api::TrackPage {
            total: 1,
            offset: 7,
            items: vec![track],
        };
        assert_eq!(page, track_page_from_proto(&track_page_to_proto(&page)));
    }

    #[test]
    fn lyrics_and_stats_round_trip() {
        let lyrics = api::LyricsView {
            plain: None,
            synced: vec![api::LyricLineView {
                start_ms: 1,
                end_ms: None,
                text: "line".into(),
                chunks: vec![api::LyricChunkView {
                    start_ms: 2,
                    text: "word".into(),
                }],
                parent_line_index: Some(0),
                background: true,
                opposite_turn: true,
            }],
        };
        assert_eq!(lyrics, lyrics_from_proto(&lyrics_to_proto(&lyrics)));

        let stats = api::StatsView {
            listen_counts: std::collections::HashMap::from([("uid".into(), 3)]),
        };
        assert_eq!(stats, stats_from_proto(&stats_to_proto(&stats)));
    }
}
