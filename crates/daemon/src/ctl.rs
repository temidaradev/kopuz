//! The control CLI: `kopuz pause`, `kopuzd next`, `kopuz status`, and so on.
//! Both binaries dispatch here before their normal startup, so one command
//! set drives whichever process is serving the API (the GUI or kopuzd),
//! found through the discovery file. Playback commands use the same typed
//! unary RPCs as any other frontend.
//!
//! Stdout/stderr ARE this module's interface: the process is acting as a
//! terminal client, tracing never gets initialized on this path.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use proto::kopuz_client::KopuzClient;
use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;

const USAGE: &str = "playback control:
  play | pause | toggle | stop     transport
  next | prev                      queue movement
  seek <secs | mm:ss>              absolute seek
  volume <0-100>                   set volume
  shuffle <on|off>                 set shuffle
  loop <none|queue|track>          set loop mode
  status [--json]                  what is playing";

/// Runs `args` as a control command if it is one. `None` means the args are
/// not a control invocation and the caller should start up normally.
pub fn run(args: &[String]) -> Option<i32> {
    let command = args.first()?.to_lowercase();
    let known = matches!(
        command.as_str(),
        "play"
            | "pause"
            | "toggle"
            | "stop"
            | "next"
            | "prev"
            | "previous"
            | "seek"
            | "volume"
            | "shuffle"
            | "loop"
            | "status"
            | "help"
    );
    if !known {
        return None;
    }
    Some(execute(&command, &args[1..]))
}

pub fn print_daemon_usage() {
    println!("usage: kopuzd [--bind 127.0.0.1:0] [--token <hex>] [--db-path <file>]");
}

fn execute(command: &str, rest: &[String]) -> i32 {
    if command == "help" {
        println!("{USAGE}");
        return 0;
    }
    let player_command = match command {
        "play" => Some(api::PlayerCommand::Play),
        "pause" => Some(api::PlayerCommand::Pause),
        "toggle" => Some(api::PlayerCommand::Toggle),
        "stop" => Some(api::PlayerCommand::Stop),
        "next" => Some(api::PlayerCommand::Next),
        "prev" | "previous" => Some(api::PlayerCommand::Previous),
        "seek" => match rest.first().map(|value| parse_seconds(value)) {
            Some(Some(secs)) => Some(api::PlayerCommand::Seek {
                position_ms: secs * 1000,
            }),
            _ => return usage_error("seek needs a position, e.g. `seek 90` or `seek 1:30`"),
        },
        "volume" => match rest.first().and_then(|value| value.parse::<u32>().ok()) {
            Some(percent) if percent <= 100 => Some(api::PlayerCommand::SetVolume {
                volume: percent as f32 / 100.0,
            }),
            _ => return usage_error("volume needs a percentage, e.g. `volume 80`"),
        },
        "shuffle" => match rest.first().map(String::as_str) {
            Some("on") => Some(api::PlayerCommand::SetMode {
                shuffle: Some(true),
                loop_mode: None,
            }),
            Some("off") => Some(api::PlayerCommand::SetMode {
                shuffle: Some(false),
                loop_mode: None,
            }),
            _ => return usage_error("shuffle needs `on` or `off`"),
        },
        "loop" => {
            let mode = match rest.first().map(String::as_str) {
                Some("none") => api::LoopMode::None,
                Some("queue") => api::LoopMode::Queue,
                Some("track") => api::LoopMode::Track,
                _ => return usage_error("loop needs `none`, `queue`, or `track`"),
            };
            Some(api::PlayerCommand::SetMode {
                shuffle: None,
                loop_mode: Some(mode),
            })
        }
        "status" => None,
        _ => unreachable!(),
    };

    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("could not start a runtime: {error}");
            return 1;
        }
    };
    let json = rest.first().map(String::as_str) == Some("--json");
    let outcome = runtime.block_on(async {
        let mut client = connect().await?;
        match player_command {
            Some(command) => send_command(&mut client, command).await.map(|_| None),
            None => {
                let state = client
                    .get_player_state(Request::new(proto::GetPlayerStateRequest {}))
                    .await
                    .map_err(|status| proto::status::from_status(&status).to_string())?;
                Ok(Some(proto::convert::player_state_from_proto(
                    state.get_ref(),
                )))
            }
        }
    });
    match outcome {
        Ok(None) => 0,
        Ok(Some(state)) => {
            if json {
                println!("{}", status_json(&state));
            } else {
                println!("{}", summarize(&state));
            }
            0
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

type AuthedClient =
    KopuzClient<tonic::service::interceptor::InterceptedService<Channel, AuthInterceptor>>;

#[derive(Clone)]
struct AuthInterceptor {
    header: MetadataValue<tonic::metadata::Ascii>,
}

impl tonic::service::Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, tonic::Status> {
        request
            .metadata_mut()
            .insert("authorization", self.header.clone());
        Ok(request)
    }
}

async fn connect() -> Result<AuthedClient, String> {
    let not_running =
        "no running kopuz found: start the app, or kopuzd for headless use".to_string();
    let path = crate::discovery::path().ok_or_else(|| not_running.clone())?;
    let body = std::fs::read_to_string(&path).map_err(|_| not_running.clone())?;
    let value: serde_json::Value = serde_json::from_str(&body).map_err(|_| not_running.clone())?;
    let port = value
        .get("port")
        .and_then(serde_json::Value::as_u64)
        .and_then(|port| u16::try_from(port).ok())
        .ok_or_else(|| not_running.clone())?;
    let token = value
        .get("token")
        .and_then(serde_json::Value::as_str)
        .ok_or(not_running)?;
    let header: MetadataValue<tonic::metadata::Ascii> = format!("Bearer {token}")
        .parse()
        .map_err(|_| "the discovery file holds an unusable token".to_string())?;
    let stale = "the kopuz that wrote the discovery file is not answering; is it still running?";
    let channel = tonic::transport::Endpoint::from_shared(format!("http://127.0.0.1:{port}"))
        .map_err(|_| stale.to_string())?
        .connect_timeout(std::time::Duration::from_secs(3))
        .connect()
        .await
        .map_err(|_| stale.to_string())?;
    Ok(KopuzClient::with_interceptor(
        channel,
        AuthInterceptor { header },
    ))
}

/// Dispatch one typed unary playback RPC.
async fn send_command(
    client: &mut AuthedClient,
    command: api::PlayerCommand,
) -> Result<(), String> {
    let result = match command {
        api::PlayerCommand::Play => client.play(Request::new(proto::PlayRequest {})).await,
        api::PlayerCommand::Pause => client.pause(Request::new(proto::PauseRequest {})).await,
        api::PlayerCommand::Toggle => client.toggle(Request::new(proto::ToggleRequest {})).await,
        api::PlayerCommand::Next => client.next(Request::new(proto::NextRequest {})).await,
        api::PlayerCommand::Previous => {
            client
                .previous(Request::new(proto::PreviousRequest {}))
                .await
        }
        api::PlayerCommand::Stop => client.stop(Request::new(proto::StopRequest {})).await,
        api::PlayerCommand::Seek { position_ms } => {
            client.seek(Request::new(proto::Seek { position_ms })).await
        }
        api::PlayerCommand::SetVolume { volume } => {
            client
                .set_volume(Request::new(proto::SetVolume { volume }))
                .await
        }
        api::PlayerCommand::SetMode { shuffle, loop_mode } => {
            client
                .set_mode(Request::new(proto::SetMode {
                    shuffle,
                    r#loop: loop_mode.map(|mode| proto::convert::loop_to_proto(mode) as i32),
                }))
                .await
        }
    };
    result
        .map(|_| ())
        .map_err(|status| proto::status::from_status(&status).to_string())
}

fn usage_error(message: &str) -> i32 {
    eprintln!("{message}");
    2
}

fn parse_seconds(value: &str) -> Option<u64> {
    if let Some((minutes, seconds)) = value.split_once(':') {
        let minutes: u64 = minutes.parse().ok()?;
        let seconds: u64 = seconds.parse().ok()?;
        (seconds < 60).then_some(minutes * 60 + seconds)
    } else {
        value.parse().ok()
    }
}

/// `status --json`. The CLI owns this shape: the `api` types carry no serde
/// derives, since protobuf is the wire contract and this is a presentation
/// layer, not a second one.
fn status_json(state: &api::PlayerState) -> String {
    let phase = match state.phase {
        api::Phase::Idle => "idle",
        api::Phase::Playing => "playing",
        api::Phase::Paused => "paused",
        api::Phase::Ended => "ended",
    };
    let track = state.track.as_ref().map(|track| {
        serde_json::json!({
            "key": track.key,
            "uid": track.uid,
            "title": track.title,
            "artist": track.artist,
            "album": track.album,
            "duration_ms": track.duration_ms,
        })
    });
    serde_json::json!({
        "phase": phase,
        "position_ms": state.position.map(|anchor| anchor.ms),
        "volume": state.volume,
        "shuffle": state.queue.shuffle,
        "track": track,
    })
    .to_string()
}

fn summarize(state: &api::PlayerState) -> String {
    let phase = match state.phase {
        api::Phase::Idle => "idle",
        api::Phase::Playing => "playing",
        api::Phase::Paused => "paused",
        api::Phase::Ended => "ended",
    };
    let Some(track) = &state.track else {
        return phase.to_string();
    };
    let mut line = format!("{phase}: {}", track.title);
    if !track.artist.is_empty() {
        line.push_str(&format!(" - {}", track.artist));
    }
    let position_ms = state.position.map(|anchor| anchor.ms);
    if let (Some(position), Some(duration)) = (position_ms, track.duration_ms) {
        let clock = |ms: u64| format!("{}:{:02}", ms / 60000, ms % 60000 / 1000);
        line.push_str(&format!(" [{}/{}]", clock(position), clock(duration)));
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seconds_parse_both_forms() {
        assert_eq!(parse_seconds("90"), Some(90));
        assert_eq!(parse_seconds("1:30"), Some(90));
        assert_eq!(parse_seconds("0:05"), Some(5));
        assert_eq!(parse_seconds("1:75"), None);
        assert_eq!(parse_seconds("abc"), None);
    }

    #[test]
    fn only_known_commands_are_claimed() {
        assert!(run(&["definitely-not-a-command".to_string()]).is_none());
        assert!(run(&["--pause".to_string()]).is_none());
        assert!(run(&["--help".to_string()]).is_none());
        assert!(run(&[]).is_none());
        assert_eq!(run(&["help".to_string()]), Some(0));
    }

    #[test]
    fn status_summary_reads_naturally() {
        let state = api::PlayerState {
            phase: api::Phase::Playing,
            track: Some(api::NowPlaying {
                title: "Song".into(),
                artist: "Band".into(),
                duration_ms: Some(223_000),
                ..Default::default()
            }),
            position: Some(api::PositionAnchor {
                ms: 63_000,
                at_ms: 0,
                playing: true,
            }),
            ..Default::default()
        };
        assert_eq!(summarize(&state), "playing: Song - Band [1:03/3:43]");
    }
}
