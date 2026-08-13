//! Real-time MIDI I/O — live hardware MIDI device access.
//!
//! This module provides a thin abstraction over [`midir`] for real-time MIDI
//! input and output to physical or virtual MIDI devices. It is gated behind
//! the `realtime-midi` Cargo feature flag.
//!
//! ## Feature Flag
//!
//! Add to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! fleet-ensemble = { features = ["realtime-midi"] }
//! ```
//!
//! Without the feature, this module compiles to stubs that return errors,
//! allowing the project to build on systems without MIDI hardware support.
//!
//! ## Architecture
//!
//! - [`RealtimeMidiInput`] — wraps `midir::MidiInput`, spawns a callback thread
//! - [`RealtimeMidiOutput`] — wraps `midir::MidiOutputConnection`, sends raw bytes
//! - [`MidiDeviceList`] — enumerates available ports
//!
//! The rest of the codebase interacts with MIDI through [`super::stream::MidiStream`],
//! which is device-agnostic. This module bridges `midir` raw bytes ↔ `MidiEvent`.

use super::stream::MidiEvent;

/// Error type for real-time MIDI operations.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MidiError {
    #[error("real-time MIDI is not enabled — build with --features realtime-midi")]
    FeatureNotEnabled,
    #[error("no MIDI input devices found")]
    NoInputDevices,
    #[error("no MIDI output devices found")]
    NoOutputDevices,
    #[error("invalid port number: {0}")]
    InvalidPort(usize),
    #[error("MIDI I/O error: {0}")]
    Io(String),
}

/// List of available MIDI input/output ports.
#[derive(Debug, Clone, Default)]
pub struct MidiDeviceList {
    pub input_ports: Vec<String>,
    pub output_ports: Vec<String>,
}

impl MidiDeviceList {
    /// Enumerate all available MIDI ports.
    ///
    /// Returns a stub (empty) list when `realtime-midi` is not enabled.
    pub fn enumerate() -> Result<Self, MidiError> {
        #[cfg(feature = "realtime-midi")]
        {
            enumerate_devices()
        }
        #[cfg(not(feature = "realtime-midi"))]
        {
            Ok(Self::default())
        }
    }
}

/// Real-time MIDI input from a hardware/virtual device.
///
/// Wraps `midir::MidiInput` and delivers events via a callback.
pub struct RealtimeMidiInput {
    #[cfg(feature = "realtime-midi")]
    inner: Option<midir::MidiInputConnection<()>>,
    #[cfg(not(feature = "realtime-midi"))]
    _phantom: (),
}

impl RealtimeMidiInput {
    /// Create a new MIDI input wrapper.
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "realtime-midi")]
            inner: None,
            #[cfg(not(feature = "realtime-midi"))]
            _phantom: (),
        }
    }

    /// Open a connection to the specified input port.
    ///
    /// The callback `on_event` is called from the MIDI thread for each
    /// incoming MIDI message. Keep it fast (<1ms).
    ///
    /// Returns `Err(MidiError::FeatureNotEnabled)` if built without
    /// the `realtime-midi` feature.
    pub fn open<F>(
        &mut self,
        port: usize,
        port_name: &str,
        on_event: F,
    ) -> Result<(), MidiError>
    where
        F: FnMut(MidiEvent) + Send + 'static,
    {
        #[cfg(feature = "realtime-midi")]
        {
            let midi_in = midir::MidiInput::new("fleet-ensemble")
                .map_err(|e| MidiError::Io(e.to_string()))?;

            let in_ports = midi_in.ports();
            let device = in_ports
                .get(port)
                .ok_or(MidiError::InvalidPort(port))?;

            let _name = port_name.to_string();

            let conn = midi_in
                .connect(
                    device,
                    "fleet-ensemble-in",
                    move |stamp: u64, data: &[u8], _| {
                        if data.len() >= 3 {
                            let arr: [u8; 3] = [data[0], data[1], data[2]];
                            if let Some(event) =
                                super::stream::MidiStream::parse_midi_bytes(&arr, stamp)
                            {
                                on_event(event);
                            }
                        }
                    },
                    (),
                )
                .map_err(|e| MidiError::Io(e.to_string()))?;

            self.inner = Some(conn);
            Ok(())
        }

        #[cfg(not(feature = "realtime-midi"))]
        {
            let _ = (port, port_name, on_event);
            Err(MidiError::FeatureNotEnabled)
        }
    }

    /// Close the MIDI input connection.
    pub fn close(&mut self) {
        #[cfg(feature = "realtime-midi")]
        {
            self.inner = None;
        }
    }
}

impl Default for RealtimeMidiInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RealtimeMidiInput {
    fn drop(&mut self) {
        self.close();
    }
}

/// Real-time MIDI output to a hardware/virtual device.
///
/// Wraps `midir::MidiOutputConnection` and sends raw MIDI bytes.
pub struct RealtimeMidiOutput {
    #[cfg(feature = "realtime-midi")]
    inner: Option<midir::MidiOutputConnection>,
    #[cfg(not(feature = "realtime-midi"))]
    _phantom: (),
}

impl RealtimeMidiOutput {
    /// Create a new MIDI output wrapper.
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "realtime-midi")]
            inner: None,
            #[cfg(not(feature = "realtime-midi"))]
            _phantom: (),
        }
    }

    /// Open a connection to the specified output port.
    pub fn open(&mut self, port: usize, port_name: &str) -> Result<(), MidiError> {
        #[cfg(feature = "realtime-midi")]
        {
            let midi_out = midir::MidiOutput::new("fleet-ensemble")
                .map_err(|e| MidiError::Io(e.to_string()))?;

            let out_ports = midi_out.ports();
            let device = out_ports
                .get(port)
                .ok_or(MidiError::InvalidPort(port))?;

            let _name = port_name.to_string();

            let conn = midi_out
                .connect(device, "fleet-ensemble-out")
                .map_err(|e| MidiError::Io(e.to_string()))?;

            self.inner = Some(conn);
            Ok(())
        }

        #[cfg(not(feature = "realtime-midi"))]
        {
            let _ = (port, port_name);
            Err(MidiError::FeatureNotEnabled)
        }
    }

    /// Send a raw MIDI byte message.
    pub fn send_raw(&mut self, data: &[u8]) -> Result<(), MidiError> {
        #[cfg(feature = "realtime-midi")]
        {
            if let Some(ref mut conn) = self.inner {
                conn.send(data)
                    .map_err(|e| MidiError::Io(e.to_string()))?;
                Ok(())
            } else {
                Err(MidiError::Io("not connected".into()))
            }
        }

        #[cfg(not(feature = "realtime-midi"))]
        {
            let _ = data;
            Err(MidiError::FeatureNotEnabled)
        }
    }

    /// Send a note-on message.
    pub fn send_note_on(
        &mut self,
        channel: u8,
        note: u8,
        velocity: u8,
    ) -> Result<(), MidiError> {
        let bytes = MidiStream::encode_note_on(channel, note, velocity);
        self.send_raw(&bytes)
    }

    /// Send a note-off message.
    pub fn send_note_off(&mut self, channel: u8, note: u8) -> Result<(), MidiError> {
        let bytes = MidiStream::encode_note_off(channel, note);
        self.send_raw(&bytes)
    }

    /// Close the MIDI output connection.
    pub fn close(&mut self) {
        #[cfg(feature = "realtime-midi")]
        {
            self.inner = None;
        }
    }
}

impl Default for RealtimeMidiOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RealtimeMidiOutput {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(feature = "realtime-midi")]
fn enumerate_devices() -> Result<MidiDeviceList, MidiError> {
    let midi_in = midir::MidiInput::new("fleet-ensemble-fingerprint")
        .map_err(|e| MidiError::Io(e.to_string()))?;
    let midi_out = midir::MidiOutput::new("fleet-ensemble-fingerprint")
        .map_err(|e| MidiError::Io(e.to_string()))?;

    let input_ports: Vec<String> = midi_in
        .ports()
        .iter()
        .map(|p| midi_in.port_name(p).unwrap_or_else(|_| "Unknown".into()))
        .collect();

    let output_ports: Vec<String> = midi_out
        .ports()
        .iter()
        .map(|p| midi_out.port_name(p).unwrap_or_else(|_| "Unknown".into()))
        .collect();

    Ok(MidiDeviceList {
        input_ports,
        output_ports,
    })
}

// Re-export MidiStream for the encode/decode helpers used above.
use super::stream::MidiStream;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_without_feature_returns_empty() {
        // Without the realtime-midi feature, enumerate returns an empty list
        // (not an error — the system just has no devices)
        let devices = MidiDeviceList::enumerate().unwrap();
        #[cfg(not(feature = "realtime-midi"))]
        {
            assert!(devices.input_ports.is_empty());
            assert!(devices.output_ports.is_empty());
        }
    }

    #[test]
    fn open_input_without_feature_returns_error() {
        let mut input = RealtimeMidiInput::new();
        let result = input.open(0, "test", |_| {});
        #[cfg(not(feature = "realtime-midi"))]
        assert_eq!(result, Err(MidiError::FeatureNotEnabled));
        #[cfg(feature = "realtime-midi")]
        {
            // With the feature enabled, this might succeed or fail depending
            // on whether there's a device at port 0. Just verify it doesn't
            // return FeatureNotEnabled.
            assert!(result.is_ok() || result != Err(MidiError::FeatureNotEnabled));
        }
    }

    #[test]
    fn open_output_without_feature_returns_error() {
        let mut output = RealtimeMidiOutput::new();
        let result = output.open(0, "test");
        #[cfg(not(feature = "realtime-midi"))]
        assert_eq!(result, Err(MidiError::FeatureNotEnabled));
        #[cfg(feature = "realtime-midi")]
        {
            let _ = result;
        }
    }

    #[test]
    fn send_raw_without_connection_fails() {
        let mut output = RealtimeMidiOutput::new();
        let result = output.send_raw(&[0x90, 60, 100]);
        // Without the feature, returns FeatureNotEnabled.
        // With the feature but no connection, returns Io error.
        assert!(result.is_err());
    }

    #[test]
    fn send_note_on_without_feature_returns_error() {
        let mut output = RealtimeMidiOutput::new();
        let result = output.send_note_on(0, 60, 100);
        #[cfg(not(feature = "realtime-midi"))]
        assert_eq!(result, Err(MidiError::FeatureNotEnabled));
        #[cfg(feature = "realtime-midi")]
        {
            let _ = result;
        }
    }

    #[test]
    fn midi_error_display_is_informative() {
        let err = MidiError::FeatureNotEnabled;
        assert!(err.to_string().contains("realtime-midi"));
    }

    #[test]
    fn device_list_default_is_empty() {
        let list = MidiDeviceList::default();
        assert!(list.input_ports.is_empty());
        assert!(list.output_ports.is_empty());
    }

    #[test]
    fn drop_does_not_panic_when_not_connected() {
        // Ensure Drop impls are safe even when never opened
        let _input = RealtimeMidiInput::new();
        let _output = RealtimeMidiOutput::new();
    }
}
