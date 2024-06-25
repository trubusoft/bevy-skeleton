use bevy::prelude::{App, Update};

use crate::people::components::{Employment, Person};
use crate::people::systems::{spawn_person, spawn_person_with_job};

#[test]
fn did_spawn_5_people_without_job() {
    let mut app = App::new();
    app.add_systems(Update, spawn_person);
    app.update();

    // 5 people should be spawned
    assert_eq!(app.world.query::<&Person>().iter(&app.world).len(), 5);
}

#[test]
fn did_spawn_5_people_with_job() {
    let mut app = App::new();
    app.add_systems(Update, spawn_person_with_job);
    app.update();

    // 5 people with job should be spawned
    assert_eq!(
        app.world
            .query::<(&Person, &Employment)>()
            .iter(&app.world)
            .len(),
        5
    );
}
