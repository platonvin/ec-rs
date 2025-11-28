#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![allow(unused)]
use ecs::*;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Debug, PartialEq, Clone, Copy)]
struct Velocity {
    x: f32,
    y: f32,
}
#[derive(Debug, PartialEq, Clone, Copy)]
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

    world.movers.spawn(Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 });
    world.statics.spawn(Position { x: 10.0, y: 10.0 }, Health { val: 100 });
    world.player.spawn(
        Position { x: 5.0, y: 5.0 },
        Velocity { x: 0.0, y: 1.0 },
        Health { val: 50 },
    );

    query!(world, |pos: &mut Position| {
        pos.x += 1.0;
    });

    assert_eq!(world.movers.Position()[0].x, 1.0);
    assert_eq!(world.statics.Position()[0].x, 11.0);
    assert_eq!(world.player.Position()[0].x, 6.0);

    query!(world, |vel: &mut Velocity| {
        vel.x += 10.0;
    });

    assert_eq!(world.movers.Velocity()[0].x, 11.0);
    assert_eq!(world.player.Velocity()[0].x, 10.0);
}
