// src/lib.rs

use anyhow::anyhow;
// region: --- imports
use anyhow::Result;
use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
// endregion: --- imports

use tokio::sync::mpsc::Sender;

pub struct AppState {
    pub playback_tx: Sender<PlaybackCommand>,
    pub record_tx: Sender<RecordingCommand>,
}

// region: --- Recording Manager

pub enum RecordingCommand {
    Start(PathBuf),
    Stop,
}

pub enum RecordingControl {
    Start,
    Stop,
}

pub struct AudioRecordingManager {
    is_recording: Arc<AtomicBool>,
}

impl AudioRecordingManager {
    pub fn new() -> Self {
        Self {
            is_recording: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn start_recording(&self) -> Result<()> {
        if !self.is_recording.load(Ordering::SeqCst) {
            println!("Recording started.");
            self.is_recording.store(true, Ordering::SeqCst);
            Ok(()) // Explicitly return Ok to match the expected Result type
        } else {
            println!("Recording is already in progress.");
            Err(anyhow!("Recording is already in progress")) // Return an error if recording is already started
        }
    }

    pub async fn stop_recording(&self) -> Result<()> {
        if self.is_recording.load(Ordering::SeqCst) {
            println!("Recording stopped.");
            self.is_recording.store(false, Ordering::SeqCst);
            Ok(()) // Explicitly return Ok to match the expected Result type
        } else {
            println!("Recording is not currently active.");
            Err(anyhow!("Recording is not currently active")) // Return an error if there's no active recording to stop
        }
    }
}
// endregion: --- Recording Manager

// region: --- Playback Manager

pub enum PlaybackCommand {
    Play(Vec<u8>),
    Pause,
    Stop,
    Resume,
}

type SinkId = usize;

pub struct AudioPlaybackManager {
    pub next_id: SinkId,
    pub sinks: HashMap<SinkId, Sink>,
    pub streams: HashMap<SinkId, OutputStream>,
    pub command_queue: VecDeque<PlaybackCommand>,
    pub is_idle: AtomicBool,
    pub current_sink: Option<SinkId>, // New field to track the current playing audio
}

impl AudioPlaybackManager {
    pub fn new() -> Self {
        AudioPlaybackManager {
            next_id: 0,
            sinks: HashMap::new(),
            streams: HashMap::new(),
            command_queue: VecDeque::new(),
            is_idle: AtomicBool::new(true),
            current_sink: None,
        }
    }

    pub async fn start_processing_commands(&mut self) {
        while let Some(command) = self.command_queue.pop_front() {
            self.handle_command(command)
                .await
                .expect("Failed to handle command");
        }
    }

    pub async fn handle_command(&mut self, command: PlaybackCommand) -> Result<(), Box<dyn Error>> {
        match command {
            PlaybackCommand::Play(audio_data) => {
                self.play_audio(audio_data).await?;
            }
            PlaybackCommand::Pause => {
                if let Some(id) = self.current_sink {
                    if let Some(sink) = self.sinks.get(&id) {
                        sink.pause();
                    }
                }
            }
            PlaybackCommand::Stop => {
                if let Some(id) = self.current_sink.take() {
                    // Remove the current sink from tracking
                    if let Some(sink) = self.sinks.get(&id) {
                        sink.stop(); // Stop the current sink
                    }
                }
            }
            PlaybackCommand::Resume => {
                if let Some(id) = self.current_sink {
                    if let Some(sink) = self.sinks.get(&id) {
                        sink.play(); // Resume the current sink
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn play_audio(&mut self, audio_data: Vec<u8>) -> Result<SinkId, Box<dyn Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let source = Decoder::new(Cursor::new(audio_data))?;

        sink.append(source);

        // Assume playback starts immediately without blocking
        let id = self.next_id;
        self.sinks.insert(id, sink);
        self.streams.insert(id, stream);
        self.current_sink = Some(id); // Set current sink ID here
        self.next_id += 1;
        Ok(id)
    }
}

// endregion: --- Playback Manager
