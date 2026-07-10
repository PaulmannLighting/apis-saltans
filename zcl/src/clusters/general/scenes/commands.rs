use zb_core::Cluster;

pub use self::add_scene::AddScene;
pub use self::add_scene_response::AddSceneResponse;
pub use self::copy_scene::CopyScene;
pub use self::copy_scene_response::CopySceneResponse;
pub use self::enhanced_add_scene::EnhancedAddScene;
pub use self::enhanced_add_scene_response::EnhancedAddSceneResponse;
pub use self::enhanced_view_scene::EnhancedViewScene;
pub use self::enhanced_view_scene_response::EnhancedViewSceneResponse;
pub use self::get_scene_membership::GetSceneMembership;
pub use self::get_scene_membership_response::GetSceneMembershipResponse;
pub use self::recall_scene::RecallScene;
pub use self::remove_all_scenes::RemoveAllScenes;
pub use self::remove_all_scenes_response::RemoveAllScenesResponse;
pub use self::remove_scene::RemoveScene;
pub use self::remove_scene_response::RemoveSceneResponse;
pub use self::store_scene::StoreScene;
pub use self::store_scene_response::StoreSceneResponse;
pub use self::view_scene::ViewScene;
pub use self::view_scene_response::ViewSceneResponse;
use crate::macros::zcl_command_enum;

mod add_scene;
mod add_scene_response;
mod copy_scene;
mod copy_scene_response;
mod enhanced_add_scene;
mod enhanced_add_scene_response;
mod enhanced_view_scene;
mod enhanced_view_scene_response;
mod get_scene_membership;
mod get_scene_membership_response;
mod recall_scene;
mod remove_all_scenes;
mod remove_all_scenes_response;
mod remove_scene;
mod remove_scene_response;
mod store_scene;
mod store_scene_response;
mod view_scene;
mod view_scene_response;

// Available Scenes cluster commands.
zcl_command_enum! {
    { Cluster::Scenes } => Scenes;
    AddScene(AddScene),
    ViewScene(ViewScene),
    RemoveScene(RemoveScene),
    RemoveAllScenes(RemoveAllScenes),
    StoreScene(StoreScene),
    RecallScene(RecallScene),
    GetSceneMembership(GetSceneMembership),
    EnhancedAddScene(EnhancedAddScene),
    EnhancedViewScene(EnhancedViewScene),
    CopyScene(CopyScene),
    AddSceneResponse(AddSceneResponse),
    ViewSceneResponse(ViewSceneResponse),
    RemoveSceneResponse(RemoveSceneResponse),
    RemoveAllScenesResponse(RemoveAllScenesResponse),
    StoreSceneResponse(StoreSceneResponse),
    GetSceneMembershipResponse(GetSceneMembershipResponse),
    EnhancedAddSceneResponse(EnhancedAddSceneResponse),
    EnhancedViewSceneResponse(EnhancedViewSceneResponse),
    CopySceneResponse(CopySceneResponse),
}
