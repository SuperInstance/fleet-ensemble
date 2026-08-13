//! Fleet Ensemble — entry point.
//!
//! Starts the Director and N Instrument Agents, wires them together
//! via an in-process CNS bus, and runs the main performance loop.

use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::broadcast;
use tracing::{info, warn};

use fleet_ensemble::director::Director;
use fleet_ensemble::instrument::{InstrumentAgent, VoiceClass};
use fleet_ensemble::protocol::{CnsPacket, EMBEDDING_DIM};

const PULSE_INTERVAL_MS: u64 = 125; // ~16th note at 120 BPM

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("Fleet Ensemble starting up");

    // CNS bus — broadcast channel for all packets
    let (cns_tx, _) = broadcast::channel::<CnsPacket>(1024);

    // Shared registry of instrument embeddings (latest per instrument)
    let embeddings: Arc<DashMap<u16, [f32; EMBEDDING_DIM]>> = Arc::new(DashMap::new());

    // Spawn the Director
    let director = Director::new(cns_tx.clone(), embeddings.clone());
    let director_handle = tokio::spawn(async move {
        director.run().await;
    });

    // Spawn Instrument Agents: Piano, Bass, Drums
    let instruments = vec![
        InstrumentAgent::new(1, VoiceClass::Piano, cns_tx.clone()),
        InstrumentAgent::new(2, VoiceClass::Bass, cns_tx.clone()),
        InstrumentAgent::new(3, VoiceClass::Drums, cns_tx.clone()),
    ];

    let mut instrument_handles = Vec::new();
    for mut agent in instruments {
        let rx = cns_tx.subscribe();
        instrument_handles.push(tokio::spawn(async move {
            agent.run(rx).await;
        }));
    }

    info!("Ensemble running: 1 director + 3 instruments");

    // Keep running until interrupted
    tokio::select! {
        _ = director_handle => warn!("Director task ended"),
        _ = futures::future::join_all(instrument_handles) => warn!("All instruments ended"),
        _ = tokio::signal::ctrl_c() => info!("Received Ctrl-C, shutting down"),
    }

    info!("Fleet Ensemble shutting down");
}
