pub mod modules;
use nail_tui::{DirectionHint, MappableExit};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Exit {
    leads_to: String,
    locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    name: String,
    description: String,
    edges: Vec<String>,
    contents: Vec<Item>,
}

pub struct Listener {
    pub read: crossbeam::channel::Receiver<Option<nail_common::Message>>,
    pub write: crossbeam::channel::Sender<Option<nail_common::Message>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
    score: i32,
    location: String,
    inventory: Vec<Item>,
}

impl nail_core::traits::Exit for Exit {
    fn is_locked(&self) -> bool {
        self.locked
    }

    fn lock(&mut self) {
        self.locked = true;
    }

    fn unlock(&mut self) {
        self.locked = false;
    }

    fn leads_to(&self) -> String {
        self.leads_to.clone()
    }
}

impl nail_tui::MappableExit for Exit {
    fn get_direction_hint() -> DirectionHint {
        todo!();
    }
}

impl nail_core::traits::Item for Item {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl nail_core::traits::Player<Item> for Player {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_inventory(&self) -> Vec<Item> {
        todo!()
    }
}

impl nail_core::traits::Room<Exit, Item> for Room {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_contents(&self) -> Vec<Item> {
        todo!()
    }

    fn get_exits(&self) -> Vec<Exit> {
        todo!()
    }
}

impl nail_core::traits::Listener for Listener {
    fn get_receiver(&self) -> &crossbeam::channel::Receiver<Option<nail_common::Message>> {
        &self.read
    }

    fn get_sender(&self) -> &crossbeam::channel::Sender<Option<nail_common::Message>> {
        &self.write
    }
}
