#![feature(macro_metavar_expr)]
use ecs::*;

#[derive(Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Debug, PartialEq)]
struct Velocity {
    x: f32,
    y: f32,
}
#[derive(Debug, PartialEq)]
struct Health {
    val: i32,
}

declare_ecs! {
    world: MyWorld,
    archetypes: {
        movers: (Position, Velocity),
        statics: (Position, Health),
        player: (Position, Velocity, Health)
    }
}

#[test]
fn test_spawn_and_query() {
    let mut world = MyWorld::new();

    // Spawn via direct archetype access
    world.movers.spawn(Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 });
    world.statics.spawn(Position { x: 10.0, y: 10.0 }, Health { val: 100 });
    world.player.spawn(
        Position { x: 5.0, y: 5.0 },
        Velocity { x: 0.0, y: 1.0 },
        Health { val: 50 },
    );

    // Query: Update all Positions (matches movers, statics, player)
    query!(world, |pos: &mut Position| {
        pos.x += 1.0;
    });

    assert_eq!(world.movers.Position[0].x, 1.0);
    assert_eq!(world.statics.Position[0].x, 11.0);
    assert_eq!(world.player.Position[0].x, 6.0);

    // Query: Update Velocity (matches movers, player)
    query!(world, |vel: &mut Velocity| {
        vel.x += 10.0;
    });

    assert_eq!(world.movers.Velocity[0].x, 11.0);
    assert_eq!(world.player.Velocity[0].x, 10.0);
}
