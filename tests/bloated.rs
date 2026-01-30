// tests/bloated.rs
#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(decl_macro)]

use ec_rs::*;

// ──────────────────────────────────────────────────────────────
//  Basic components
// ──────────────────────────────────────────────────────────────
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub val: i32,
}

macro_rules! status_components {
    ($($name:ident),* $(,)?) => {
        $(#[derive(Default, Debug, Clone, Copy, PartialEq)] pub struct $name;)*
    };
}

status_components! {
    // Basic debuffs
    ScStone, ScFreeze, ScStun, ScSleep, ScPoison, ScCurse,
    ScSilence, ScConfusion, ScBlind, ScBleeding, ScDpoison,
    ScFear, ScBurning, ScCrystalize, ScWhiteImprison, ScDeepSleep,

    // Buffs & special states
    ScBerserk, ScSaturdayNightFever, ScBloodyLust,
    ScSteelBody, ScBladeStop, ScAuraBlade,
    ScExplosionSpirits, ScEnergyCoat, ScOverThrust,
    ScQuicken, ScAngelus, ScSignumCrucis,
    ScAssumptio, ScKaite, ScBunsin,
    ScSoulLink, ScWarm, ScLightBlade,
    ScMoonlit, ScMarionette, ScUndead,
    ScContract, ScSight, ScHide,
    ScCloaking, ScChaseWalk, ScRuwach,
    ScFlying, ScWug, ScWugRider,
    ScMadogear, ScXmas, ScSummer,
    ScDragon1, ScDragon2, ScDragon3, ScDragon4, ScDragon5,
    ScHanbok, ScOktoberfest,

    // Skills & job-specific
    ScCartBoost, ScWindWalk, ScIncreaseAgi, ScDecreaseAgi,
    ScSlowGrace, ScPlatinumAlter, ScMadnessCanceler,
    ScOneHand, ScTarotCard, ScRefresh, ScLuxAnima,
    ScHermode, ScGospel, ScInspiration,
    ScBanishingBuster, ScClearance, ScDispell,
    ScElementalChange, ScTidalWeapon, ScWaterScreen,
    ScCpWeapon, ScCpArmor, ScCpShield, ScCpHelm,
    ScMaximizePower, ScEdp, ScAdrenaline, ScWeaponPerfection,
    ScOverThrustMax, ScPowerThrust, ScMagicPower, ScPreserve,
    ScEnchantBlade, ScDeathBound, ScMillenniumShield,
    ScCrushStrike, ScRebirth, ScSpellBreaker,
    ScEnchantPoison, ScPoisonReact, ScAspersio,
    ScBenedictum, ScKyrie, ScMagnificat,
    ScGloria, ScLexDivina, ScLexAeterna,
    ScAdoramus, ScDupleLight,

    // 3rd job skills
    ScCloakingExceed, ScRollingCutter, ScCrossImpact, ScDarkClaw,
    ScReproduce, ScAutoShadowSpell, ScShadowForm, ScFatalMenace,
    ScReadingSpellBook, ScFreezingSpell, ScSummonBall,
    ScGentleTouch, ScRisingDragon,
    ScWargRider, ScFearBreeze, ScAimedBolt,
    ScSongOfLutie, ScDrumOnTheBattlefield,

    // Food & support
    ScBlessing, ScFoodStr, ScFoodAgi, ScFoodVit, ScFoodInt, ScFoodDex, ScFoodLuk,

    // Defense & survival
    ScEndure, ScAutoguard, ScDefender, ScReflectShield, ScPrestige, ScBanding,

    // Gunslinger / Rebellion
    ScLastStand, ScGatlingFever, ScHeatBarrel, ScAntiMaterialBlast, ScEternalChain,

    // Ninja / Kagerou-Oboro
    ScShadowLeap, ScMirrorImage, ScKunaiExplosion, ScSwirlingPetal, ScCrossSlash,

    // Summoner
    ScCatnipMeteor, ScPickyPeck, ScScarOfTarou,

    // Star Emperor / Soul Reaper
    ScSolarBurst, ScFullMoonKick, ScFallingStar,
    ScSoulReap, ScCurseOfSoul, ScSoulUnity,

    // Genetic / Mechanic
    ScPalletPaint, ScPyrotechnic, ScThornTrap,
    ScBloodSucker, ScSporeExplosion,

    // Others
    ScMasquerade, ScFullThrottle, ScReboundShield,
    ScStripAccessory, ScInvisibility, ScMagicalBullet,

    ScManuAtk,ScManuDef,ScSplAtk,ScSplDef
}

// ──────────────────────────────────────────────────────────────
//  Giant ECS with 30 archetypes
// ──────────────────────────────────────────────────────────────
declare_ecs! {
    world: StressWorld,
    archetypes: {
        Player:            (Position, Velocity, Health, ScBerserk, ScQuicken, ScAngelus, ScAdrenaline, ScInspiration),
        MonsterNormal:     (Position, Velocity, Health, ScPoison, ScBleeding, ScCurse),
        MonsterBoss:       (Position, Velocity, Health, ScStone, ScFreeze, ScStun, ScSleep, ScDeepSleep),
        Homunculus:        (Position, Velocity, ScSoulLink, ScKaite, ScKyrie),
        Mercenary:         (Position, Velocity, Health, ScMagnificat, ScGloria),
        Pet:               (Position, Velocity, ScEndure, ScAutoguard),
        Elemental:         (Position, Velocity, ScElementalChange, ScTidalWeapon, ScWaterScreen),
        WugRider:          (Position, Velocity, ScWugRider, ScCartBoost, ScIncreaseAgi),
        MadogearUser:      (Position, Velocity, ScMadogear, ScHeatBarrel, ScMagicalBullet),
        GeneticCart:       (Position, Velocity, ScCartBoost, ScPyrotechnic, ScThornTrap),

        RuneKnight:        (Position, Velocity, ScEnchantBlade, ScDeathBound, ScMillenniumShield, ScRebirth),
        RoyalGuard:        (Position, Velocity, ScDefender, ScReflectShield, ScPrestige, ScBanding),
        Mechanic:          (Position, Velocity, ScMadogear, ScPowerThrust, ScWeaponPerfection),
        GuillotineCross:   (Position, Velocity, ScCloakingExceed, ScRollingCutter, ScCrossImpact, ScDarkClaw),
        ShadowChaser:      (Position, Velocity, ScReproduce, ScAutoShadowSpell, ScShadowForm, ScFatalMenace),
        Sorcerer:          (Position, Velocity, ScSpellBreaker, ScSoulLink, ScPreserve),
        Warlock:           (Position, Velocity, ScReadingSpellBook, ScFreezingSpell, ScSummonBall),
        Archbishop:        (Position, Velocity, ScAssumptio, ScKyrie, ScMagnificat, ScBenedictum),
        Sura:              (Position, Velocity, ScGentleTouch, ScPowerThrust, ScRisingDragon),
        Ranger:            (Position, Velocity, ScWargRider, ScFearBreeze, ScAimedBolt),

        MinstrelWanderer:  (Position, Velocity, ScSongOfLutie, ScDrumOnTheBattlefield, ScSaturdayNightFever),
        SuperNovice:       (Position, Velocity, ScAngelus, ScBlessing, ScIncreaseAgi, ScFoodStr, ScFoodInt, ScFoodLuk),
        Gunslinger:        (Position, Velocity, ScMadnessCanceler, ScLastStand, ScGatlingFever),
        Ninja:             (Position, Velocity, ScCloaking, ScShadowLeap, ScMirrorImage),
        KagerouOboro:      (Position, Velocity, ScKunaiExplosion, ScSwirlingPetal, ScCrossSlash),
        Rebellion:         (Position, Velocity, ScHeatBarrel, ScAntiMaterialBlast, ScEternalChain),
        Summoner:          (Position, Velocity, ScCatnipMeteor, ScPickyPeck, ScScarOfTarou),
        StarEmperor:       (Position, Velocity, ScSolarBurst, ScFullMoonKick, ScFallingStar),
        SoulReaper:        (Position, Velocity, ScSoulReap, ScCurseOfSoul, ScSoulUnity),
        Dummy:             (Position, Health, ScStone, ScFreeze, ScStun, ScSleep, ScBurning, ScPoison, ScBleeding, ScCurse)
    }
}

#[test]
fn stress_compilation_time() {
    let mut world = StressWorld::new();

    // Spawn one entity per archetype to force full codegen
    world.Player.spawn(
        Position::default(),
        Velocity::default(),
        Health::default(),
        ScBerserk,
        ScQuicken,
        ScAngelus,
        ScAdrenaline,
        ScInspiration,
    );
    world.MonsterBoss.spawn(
        Position::default(),
        Velocity::default(),
        Health::default(),
        ScStone,
        ScFreeze,
        ScStun,
        ScSleep,
        ScDeepSleep,
    );
    world.MadogearUser.spawn(
        Position::default(),
        Velocity::default(),
        ScMadogear,
        ScHeatBarrel,
        ScMagicalBullet,
    );
    world.Gunslinger.spawn(
        Position::default(),
        Velocity::default(),
        ScMadnessCanceler,
        ScLastStand,
        ScGatlingFever,
    );
    world.KagerouOboro.spawn(
        Position::default(),
        Velocity::default(),
        ScKunaiExplosion,
        ScSwirlingPetal,
        ScCrossSlash,
    );

    // 25 extremely heavy queries
    query!(world, |p: *mut Position, v: *mut Velocity| {
        (*p).x += (*v).x;
    });

    query!(world, |p: *mut Position,
                   h: *mut Health,
                   b: *mut ScBerserk| {
        (*p).y += 1.0;
    });

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   b: *mut ScBerserk,
                   q: *mut ScQuicken,
                   a: *mut ScAngelus| {
        (*p).x += (*v).x * 0.1;
    });

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   m: *mut ScMadogear,
                   hb: *mut ScHeatBarrel,
                   mb: *mut ScMagicalBullet| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   s: *mut ScStone,
                   f: *mut ScFreeze,
                   st: *mut ScStun| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   c: *mut ScCloakingExceed,
                   r: *mut ScRollingCutter,
                   ci: *mut ScCrossImpact,
                   dc: *mut ScDarkClaw| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   sp: *mut ScSpellBreaker,
                   sl: *mut ScSoulLink,
                   pr: *mut ScPreserve| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   fs: *mut ScFoodStr,
                   fa: *mut ScFoodAgi,
                   fv: *mut ScFoodVit,
                   fi: *mut ScFoodInt| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   cart: *mut ScCartBoost,
                   wind: *mut ScWindWalk,
                   agi: *mut ScIncreaseAgi| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   end: *mut ScEndure,
                   ag: *mut ScAutoguard,
                   rs: *mut ScReflectShield,
                   pr: *mut ScPrestige| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   strip: *mut ScStripAccessory,
                   inv: *mut ScInvisibility| {});

    query!(
        world,
        |p: *mut Position,
         v: *mut Velocity,
         h: *mut Health,
         song: *mut ScSongOfLutie,
         drum: *mut ScDrumOnTheBattlefield| {}
    );

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   cat: *mut ScCatnipMeteor,

                   peck: *mut ScPickyPeck| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   sb: *mut ScSolarBurst,
                   fmk: *mut ScFullMoonKick| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   reap: *mut ScSoulReap,
                   curse: *mut ScCurseOfSoul| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   cpw: *mut ScCpWeapon,
                   cpa: *mut ScCpArmor,
                   cps: *mut ScCpShield,
                   cph: *mut ScCpHelm| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   paint: *mut ScPalletPaint,
                   pyro: *mut ScPyrotechnic| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   thorn: *mut ScThornTrap,
                   spore: *mut ScSporeExplosion| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   ma: *mut ScManuAtk,
                   md: *mut ScManuDef| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   sa: *mut ScSplAtk,
                   sd: *mut ScSplDef| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   hb: *mut ScHeatBarrel,
                   amb: *mut ScAntiMaterialBlast| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   edp: *mut ScEdp,
                   maxp: *mut ScMaximizePower| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   rb: *mut ScReboundShield,
                   hb2: *mut ScHeatBarrel| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   h: *mut Health,
                   tarot: *mut ScTarotCard,
                   ref_: *mut ScRefresh,
                   lux: *mut ScLuxAnima,
                   her: *mut ScHermode| {});

    query!(world, |p: *mut Position,
                   v: *mut Velocity,
                   stone: *mut ScStone,
                   f: *mut ScFreeze,
                   s: *mut ScStun,
                   sleep: *mut ScSleep,
                   burn: *mut ScBurning,
                   poi: *mut ScPoison| {});
}
