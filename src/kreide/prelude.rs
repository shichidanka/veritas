use super::types::*;

impl From<RPG_GameCore_CharacterDataComponent> for RPG_GameCore_BattleEventDataComponent {
    fn from(c: RPG_GameCore_CharacterDataComponent) -> Self {
        RPG_GameCore_BattleEventDataComponent(c.0)
    }
}

impl From<RPG_GameCore_AvatarDataComponent> for RPG_GameCore_CharacterDataComponent {
    fn from(c: RPG_GameCore_AvatarDataComponent) -> Self {
        RPG_GameCore_CharacterDataComponent(c.0)
    }
}
