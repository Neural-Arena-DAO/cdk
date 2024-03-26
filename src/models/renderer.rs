use three_rs::math::{
    Vector3, 
    Quaternion
};
#[cfg(not(target_arch = "wasm32"))]
use three_rs::core::RGB;
use super::assets::{Asset, AssetRef};

pub type ModelRef = usize;

pub trait Renderer<ES> {
    fn reset(
        &mut self
    );

    fn load_asset(
        &mut self,
        asset_ref: AssetRef,
        asset: &Asset
    ) -> Result<(), String>;
    
    fn add_model(
        &mut self,
        asset_ref: AssetRef
    ) -> Result<ModelRef, String>;

    #[cfg(not(target_arch = "wasm32"))]
    fn add_from_linestrips(
        &mut self,
        color: RGB,
        vertices: Vec<Vector3>
    ) -> Result<ModelRef, String>;

    #[cfg(not(target_arch = "wasm32"))]
    fn add_from_lines(
        &mut self,
        color: RGB,
        vertices: Vec<Vector3>
    ) -> Result<ModelRef, String>;

    fn show_model(
        &mut self,
        model_ref: ModelRef
    );

    fn hide_model(
        &mut self,
        model_ref: ModelRef
    );

    #[cfg(not(target_arch = "wasm32"))]
    fn set_model_vertices(
        &mut self,
        model_ref: ModelRef,
        vertices: Vec<Vector3>
    );
    
    fn set_model_position(
        &mut self,
        model_ref: ModelRef,
        pos: &Vector3
    );

    fn set_model_rotation(
        &mut self,
        model_ref: ModelRef,
        q: &Quaternion
    );

    fn set_model_scale(
        &mut self,
        model_ref: ModelRef,
        scale: &Vector3
    );

    fn change_model_state(
        &mut self,
        model_ref: ModelRef,
        from: ES,
        to: ES
    );

    fn update(
        &mut self,
        delta_time: f32
    );

    fn render(
        &mut self
    );
}

