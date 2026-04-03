use std::{
    collections::HashSet,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use uuid::Uuid;

use crate::{
    app::event::SkinUpdate,
    services::{
        cache_service::{SkinCache, SkinRecord},
        mojang_api::MojangApi,
    },
    util::result::Result,
};

pub struct SkinService {
    api: MojangApi,
    cache: SkinCache,
    pending: HashSet<Uuid>,
    sender: Sender<SkinUpdate>,
    receiver: Receiver<SkinUpdate>,
}

impl SkinService {
    pub fn new() -> Result<Self> {
        let (sender, receiver) = mpsc::channel();
        Ok(Self {
            api: MojangApi::new()?,
            cache: SkinCache::default(),
            pending: HashSet::new(),
            sender,
            receiver,
        })
    }

    pub fn request(&mut self, uuid: Uuid) -> Option<SkinUpdate> {
        if let Some(record) = self.cache.get(&uuid) {
            return Some(match record.clone() {
                SkinRecord::Ready {
                    face,
                    name,
                    skin_url,
                } => SkinUpdate {
                    uuid,
                    result: Ok(face),
                    resolved_name: name,
                    skin_url,
                },
                SkinRecord::Unavailable {
                    message,
                    name,
                    skin_url,
                } => SkinUpdate {
                    uuid,
                    result: Err(message),
                    resolved_name: name,
                    skin_url,
                },
            });
        }

        if self.pending.contains(&uuid) {
            return None;
        }

        self.pending.insert(uuid);
        let sender = self.sender.clone();
        let api = self.api.clone();
        thread::spawn(move || {
            let update = match api.fetch_profile(uuid) {
                Ok(profile) => {
                    let result = match profile.skin_url.as_deref() {
                        Some(skin_url) => {
                            api.fetch_face(skin_url).map_err(|error| error.to_string())
                        }
                        None => Err("missing Mojang skin URL".to_owned()),
                    };

                    SkinUpdate {
                        uuid,
                        result,
                        resolved_name: Some(profile.name),
                        skin_url: profile.skin_url,
                    }
                }
                Err(error) => SkinUpdate {
                    uuid,
                    result: Err(error.to_string()),
                    resolved_name: None,
                    skin_url: None,
                },
            };
            let _ = sender.send(update);
        });

        None
    }

    pub fn drain_updates(&mut self) -> Vec<SkinUpdate> {
        let mut updates = Vec::new();
        while let Ok(update) = self.receiver.try_recv() {
            self.pending.remove(&update.uuid);
            match &update.result {
                Ok(face) => self.cache.insert(
                    update.uuid,
                    SkinRecord::Ready {
                        face: face.clone(),
                        name: update.resolved_name.clone(),
                        skin_url: update.skin_url.clone(),
                    },
                ),
                Err(message) => self.cache.insert(
                    update.uuid,
                    SkinRecord::Unavailable {
                        message: message.clone(),
                        name: update.resolved_name.clone(),
                        skin_url: update.skin_url.clone(),
                    },
                ),
            }
            updates.push(update);
        }
        updates
    }
}
