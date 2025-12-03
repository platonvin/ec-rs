#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(decl_macro)]
use ecs::*;

#[derive(Default, Debug, Clone, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Default, Debug, Clone, PartialEq)]
struct Velocity {
    x: f32,
    y: f32,
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Health {}

declare_ecs! {
    world: MyWorld,
    archetypes: {
        movers: (Position, Velocity, Health),
        statics: (Position, Health),
        player: (Position, Velocity, Health)
    }
}

#[test]
fn test_spawn_and_query() {
    let mut world = MyWorld::new();

    let mover_0 = world.movers.spawn(
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 1.0, y: 0.0 },
        Health {},
    );
    world.statics.spawn(Position { x: 10.0, y: 10.0 }, Health {});
    world.player.spawn(
        Position { x: 5.0, y: 5.0 },
        Velocity { x: 0.0, y: 1.0 },
        Health {},
    );

    query!(world, |pos: &mut Position| {
        pos.x += 1.0;
    });

    assert_eq!(world.movers.Position()[0].x, 1.0);
    assert_eq!(world.statics.Position()[0].x, 11.0);
    assert_eq!(world.player.Position()[0].x, 6.0);

    query!(world, |vel: &mut Velocity,
                   pos: &mut Position,
                   h: &mut Health| {
        vel.x += 10.0;
        pos.x += vel.x;

        let mut mover1 = unsafe { world.get_entity_mut(mover_0).unwrap() };
        let (pos, vel) = extract_components_from_refs!(mover1, [Position, Velocity]).unwrap();

        // let mut mover2 = unsafe { world.movers.get_entity_mut(mover_0) };
        // let (pos, vel) = extract_components_from_refs!(mover2, [Position, Velocity]).unwrap();
    });

    assert_eq!(world.movers.Velocity()[0].x, 11.0);
    assert_eq!(world.player.Velocity()[0].x, 10.0);
}
