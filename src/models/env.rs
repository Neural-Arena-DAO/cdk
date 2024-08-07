use std::collections::HashMap;
#[cfg(not(feature = "js"))]
use rand::rngs::StdRng;
#[cfg(not(feature = "js"))]
use super::{
    renderer::Renderer,
    assets::{Asset, AssetRef},
    instance::InstancePlayer,
    game::GameInfo
};
#[cfg(not(feature = "js"))]
use crate::models::game::GameMapId;
#[cfg(feature = "js")]
use super::js_renderer::JsRenderer;

pub type EnvObs = HashMap<String, Vec<f32>>;
pub type EnvAction = usize;

#[cfg(not(feature = "js"))]
#[derive(Clone)]
pub struct EnvPlayerOptions {
    pub radius: f32,
    pub height: f32,
    pub xp: f32,
}

pub struct EnvPlayer {
    pub active: bool,
    pub skill: String,
    pub reward: f32,
    pub obs: EnvObs,
}

pub struct EnvStep {
    pub terminated: bool,
    pub winner: Option<usize>,
    pub players: Vec<EnvPlayer>,
}

pub trait Env<ES> {
    #[cfg(not(feature = "js"))]
    fn get_assets(
        &self,
        game: Option<&GameInfo>,
        map_id: GameMapId,
        players: &Vec<Option<&InstancePlayer>>
    ) -> HashMap::<AssetRef, Asset>;
    
    #[cfg(not(feature = "js"))]
    fn get_num_actions(
        &self,
        skill: &str
    ) -> usize;
    
    #[cfg(not(feature = "js"))]
    fn get_num_obs(
        &self,
        skill: &str
    ) -> usize;
    
    #[cfg(not(feature = "js"))]
    fn reset(
        &mut self, 
        players: &Vec<EnvPlayerOptions>,
        renderer: &mut dyn Renderer<ES>,
        rng: &mut StdRng
    ) -> Vec<EnvObs>;

    #[cfg(feature = "js")]
    fn reset(
        &mut self, 
        renderer: JsRenderer
    ) -> Vec<EnvObs>;

    #[cfg(not(feature = "js"))]
    fn step(
        &mut self, 
        actions: &Vec<(usize, Option<EnvAction>)>, 
        delta_time: f32,
        renderer: &mut dyn Renderer<ES>,
        rng: &mut StdRng,
    ) -> EnvStep;
    
    #[cfg(not(feature = "js"))]
    fn random_action(
        &mut self,
        skill: &str, 
        rng: &mut StdRng
    ) -> EnvAction;
    
    #[cfg(not(feature = "js"))]
    fn render(
        &mut self,
        renderer: &mut dyn Renderer<ES>,
        delta_time: f32
    );

    #[cfg(feature = "js")]
    fn render(
        &mut self,
        renderer: JsRenderer,
        delta_time: f32
    );

    #[cfg(not(feature = "js"))]
    fn serialize_events(
        &self
    ) -> Result<Vec<u8>, String>;
	
    #[cfg(feature = "js")]
    fn deserialize_events(
        &mut self, 
        buf: &Vec<u8>
    ) -> Result<(), String>;

    #[cfg(not(feature = "js"))]
    fn serialize_entities(
        &self
    ) -> Result<Vec<u8>, String>;
	
    #[cfg(feature = "js")]
    fn deserialize_entities(
        &mut self, 
        buf: &Vec<u8>
    ) -> Result<(), String>;
}