use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TestPlugin)
        .run();
}


fn say_hi() {
    println!("hi");
}

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        println!("wow this is pretty cool");

        app.add_system(say_hi);
    }
}


