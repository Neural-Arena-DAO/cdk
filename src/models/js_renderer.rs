use wasm_bindgen::prelude::*;
use super::instance::{InstancePlayerEntity, InstanceNpcEntity};
use super::renderer::ModelRef;
use super::assets::AssetRef;

#[wasm_bindgen]
extern "C" {
    pub type JsRenderer;

    #[wasm_bindgen(method)]
    pub fn add_model(
        this: &JsRenderer,
        asset_ref: AssetRef,
        model_ref: ModelRef
    ) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn show_model(
        this: &JsRenderer,
        model_ref: ModelRef
    );

    #[wasm_bindgen(method)]
    pub fn hide_model(
        this: &JsRenderer,
        model_ref: ModelRef
    );

    #[wasm_bindgen(method)]
    pub fn set_model_position(
        this: &JsRenderer,
        model_ref: ModelRef,
        pos: &[f32]
    );

    #[wasm_bindgen(method)]
    pub fn set_model_rotation(
        this: &JsRenderer,
        model_ref: ModelRef,
        q: &[f32]
    );

    #[wasm_bindgen(method)]
    pub fn set_model_scale(
        this: &JsRenderer,
        model_ref: ModelRef,
        scale: &[f32]
    );

    #[wasm_bindgen(method)]
    pub fn change_model_state(
        this: &JsRenderer,
        model_ref: ModelRef,
        from: String,
        to: String
    );

    #[wasm_bindgen(method)]
    pub fn set_players(
        this: &JsRenderer,
        players: Vec<InstancePlayerEntity>
    );

    #[wasm_bindgen(method)]
    pub fn set_npcs(
        this: &JsRenderer,
        npcs: Vec<InstanceNpcEntity>
    );

    #[wasm_bindgen(method)]
    pub fn render(
        this: &JsRenderer
    );

    #[wasm_bindgen(method)]
    pub fn reset(
        this: &JsRenderer
    );
}
