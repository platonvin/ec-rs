#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(decl_macro)]
use ec_rs::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Velocity {
    x: f32,
    y: f32,
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Health {}

declare_ecs! {
    world: MyWorld,
    archetypes: {
        movers: (Position, Velocity),
        statics: (Position, Health),
        player: (Position, Velocity, Health),
        barricade: (Health)
    }
}

#[test]
fn test_spawn_and_query() {
    let mut world = MyWorld::new();

    let mover_0 = world.movers.spawn(Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 });
    world.movers.spawn(Position { x: 1.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 });
    world.movers.spawn(Position { x: 2.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 });
    world.movers.spawn(Position { x: 3.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 });
    world.statics.spawn(Position { x: 10.0, y: 10.0 }, Health {});
    world.player.spawn(
        Position { x: 5.0, y: 5.0 },
        Velocity { x: 0.0, y: 1.0 },
        Health {},
    );

    query!(world, |pos: *mut Position| {
        (*pos).x += 1.0;
    });

    assert_eq!(world.movers.Position()[0].x, 1.0);
    assert_eq!(world.statics.Position()[0].x, 11.0);
    assert_eq!(world.player.Position()[0].x, 6.0);

    query!(world, [vel: *mut Velocity, pos: *mut Position] {
        (*vel).x += 10.0;
        (*pos).x += (*vel).x;
    });

    query!(world, [vel: *mut Velocity, pos: *mut Position, health: *mut Health] {
    });

    query!(world, [pos: *mut Position] {
        (*pos).x += 0.0;

        let mut mover0 = unsafe { world.movers.get_entity_mut(mover_0) };
        let (pos2, vel) = extract_components_from_refs!(mover0, [Position, Velocity]).unwrap();

        // invalid since then there would be 2 &mut refs and restrictness does not work anymore
        (*pos).x = 1.0;
        (*pos2).x = 1.0;
    });

    assert_eq!(world.movers.Velocity()[0].x, 11.0);
    assert_eq!(world.player.Velocity()[0].x, 10.0);
}
