use super::types::*;

impl From<RPG_GameCore_CharacterDataComponent> for RPG_GameCore_BattleEventDataComponent {
    fn from(c: RPG_GameCore_CharacterDataComponent) -> Self {
        RPG_GameCore_BattleEventDataComponent(c.0)
    }
}
