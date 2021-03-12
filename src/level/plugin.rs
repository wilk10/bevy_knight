use bevy::prelude::*;

pub struct LevelPlugin;

#[derive(Eq, PartialEq)]
enum LevelEvents {
    SpawnTrigger,
} 

impl Default for LevelPlugin {
    fn default() -> Self {
        LevelPlugin
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<LevelEvents>()
            .add_system(check_remaining_level_size.system())
            .add_system(spawn_trigger.system());
    }
}

fn check_remaining_level_size(
    mut events: ResMut<Events<LevelEvents>>,
) {
    // TODO: check the condition upon which more level needs to be spawned (eg: distance between viewport and end of spawned level)
    // if condition is met:
    events.send(LevelEvents::SpawnTrigger);
}

fn spawn_trigger(
    mut events_reader: Local<EventReader<LevelEvents>>,
    events: Res<Events<LevelEvents>>,
) {
    if events_reader.latest(&events) == Some(&LevelEvents::SpawnTrigger) {
        println!("puppete")
    }
}
