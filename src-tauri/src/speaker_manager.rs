use shared::speaker::{SpeakerMetadata, SpeakerOption};
use std::collections::HashMap;
use std::error::Error;
use tonic::transport::Channel;
use tonic::Streaming;

use crate::pb::v1::control_request::Command;
use crate::pb::v1::speaker_service_client::SpeakerServiceClient;
use crate::pb::v1::{ControlRequest, GetPluginMetadataRequest, SpeakRequest, SpeakResponse};

pub struct Speaker {
    client: SpeakerServiceClient<Channel>,
    metadata: SpeakerMetadata,
}

impl Speaker {
    async fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
        let mut client = SpeakerServiceClient::connect(addr.to_string()).await?;
        let req = GetPluginMetadataRequest {};
        let res = client.get_plugin_metadata(req).await?.into_inner();
        Ok(Self {
            client: client,
            metadata: SpeakerMetadata {
                name: res.name,
                version: res.version,
                options: res
                    .options
                    .iter()
                    .map(|opt| SpeakerOption {
                        id: opt.id.clone(),
                        label: opt.label.clone(),
                        description: opt.description.clone(),
                        default_value: opt.default_value.clone(),
                    })
                    .collect(),
            },
        })
    }

    pub async fn speak(
        &self,
        lines: Vec<String>,
        options: HashMap<String, String>,
    ) -> Result<Streaming<SpeakResponse>, Box<dyn Error>> {
        let req = SpeakRequest { lines, options };
        let res = self.client.clone().speak(req).await?;
        Ok(res.into_inner())
    }

    pub async fn pause(&self) -> Result<(), Box<dyn Error>> {
        let req = ControlRequest {
            command: Command::Pause as i32,
        };
        let _ = self.client.clone().control(req).await?;
        Ok(())
    }

    pub async fn resume(&self) -> Result<(), Box<dyn Error>> {
        let req = ControlRequest {
            command: Command::Resume as i32,
        };
        let _ = self.client.clone().control(req).await?;
        Ok(())
    }
    pub async fn stop(&self) -> Result<(), Box<dyn Error>> {
        let req = ControlRequest {
            command: Command::Stop as i32,
        };
        let _ = self.client.clone().control(req).await?;
        Ok(())
    }
}

pub struct SpeakerManager {
    speakers: HashMap<String, Speaker>,
}

impl SpeakerManager {
    pub fn new() -> Self {
        Self {
            speakers: HashMap::new(),
        }
    }

    pub async fn add_client(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        let speaker = Speaker::new(addr).await?;
        self.speakers.insert(speaker.metadata.name.clone(), speaker);
        Ok(())
    }

    pub fn get_metadata_info(&self) -> Vec<SpeakerMetadata> {
        self.speakers
            .iter()
            .map(|(_, s)| s.metadata.clone())
            .collect()
    }

    pub fn get_client(&self, name: String) -> Option<&Speaker> {
        self.speakers.get(&name)
    }
}
