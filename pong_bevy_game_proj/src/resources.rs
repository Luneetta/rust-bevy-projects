use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerOneScore{
    pub value: u32,
}

#[derive(Resource)]
pub struct PlayerTwoScore{
    pub value: u32,
}

impl Default for PlayerOneScore {
    fn default() -> PlayerOneScore {
        PlayerOneScore {
            value: 0
        }
    }
}

impl Default for PlayerTwoScore {
    fn default() -> PlayerTwoScore {
        PlayerTwoScore {value: 0}
    }
}