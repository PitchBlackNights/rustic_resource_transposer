#![allow(dead_code)]

use std::path::{Path, PathBuf};

/// Struct to describe a Resource Pack
#[derive(Debug)]
pub struct Pack {
    /// Pack name (e.g. `String::from("MyCoolPack")`)
    name: String,
    /// Pack location (e.g. `PathBuf::from(r"C:\packs\MyCoolPack")`)
    location: PathBuf,
    /// Pack version (e.g. `String::from("1.0.1")`)
    version: String,
    /// Minecraft edition (e.g. `MCEdition.Bedrock`)
    mc_edition: MCEdition,
    /// Minecraft version (e.g. `String::from("1.21.2")`)
    mc_version: String,
}

/// Struct to describe a Resource
#[derive(Debug)]
pub struct Resource {
    /// Resouce name (e.g. `String::from("stone")`)
    name: String,
    /// Resource location (e.g. `PathBuf::from(r"C:\packs\MyCoolPack\textures\blocks\stone.png")`)
    location: PathBuf,
    /// Minecraft edition (e.g. `MCEdition.Bedrock`)
    mc_edition: MCEdition,
    /// Minecraft version (e.g. `String::from("1.21.2")`)
    mc_version: String,
    /// Resouce type (e.g. `ResourceType::Texture(TextureType::Block)`)
    resource_type: ResourceType,
}

impl Pack {
    /// Constructor to create a new Pack instance
    fn new(name: String, location: &Path, version: String, mc_edition: MCEdition, mc_version: String) -> Pack {
        Pack {
            name,
            location: location.to_path_buf(),
            version,
            mc_edition,
            mc_version,
        }
    }

    /// Method to get all resources of a pack
    fn get_resources(&self) -> &Vec<Resource> {
        todo!()
    }
}

impl Resource {
    /// Constructor to create a new Resource instance
    fn new(name: String, location: &Path, mc_edition: MCEdition, mc_version: String, resource_type: ResourceType) -> Resource {
        Resource {
            name,
            location: location.to_path_buf(),
            mc_edition,
            mc_version,
            resource_type,
        }
    }
}


/// Enum to describe a MC Edition
#[derive(Debug, PartialEq, Eq)]
pub enum MCEdition {
    Java,
    Bedrock,
}

/// Enum to describe a Resource Type
#[derive(Debug, PartialEq, Eq)]
pub enum ResourceType {
    AnimationController,
    Animation,
    Entity,
    Fog,
    Material,
    Model,
    RenderController,
    Sound,
    Text,
    UI,
    Generic,
    Texture(TextureType),
}

/// Enum to describe a Texture Type
#[derive(Debug, PartialEq, Eq)]
pub enum TextureType {
    Item,
    Block,
    Entity,
    Particle,
    UI,
    GUI,
}
