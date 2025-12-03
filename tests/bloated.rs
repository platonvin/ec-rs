// tests/bloated.rs
#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(decl_macro)]

use ecs::*;

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

// ──────────────────────────────────────────────────────────────
// 95+ status components (complete list)
// ──────────────────────────────────────────────────────────────
macro_rules! status_components {
    ($($name:ident),* $(,)?) => {
        $(#[derive(Default, Debug, Clone, Copy, PartialEq)] pub struct $name;)*
    };
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScStone;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFreeze;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScStun;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSleep;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScPoison;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCurse;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSilence;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScConfusion;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBlind;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBleeding;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDpoison;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFear;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBurning;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCrystalize;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScWhiteImprison;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDeepSleep;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBerserk;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSaturdayNightFever;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBloodyLust;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSteelBody;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBladeStop;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScAuraBlade;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScExplosionSpirits;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScEnergyCoat;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScOverThrust;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScQuicken;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScAngelus;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSignumCrucis;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScAssumptio;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScKaite;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBunsin;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSoulLink;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScWarm;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScLightBlade;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMoonlit;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMarionette;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScUndead;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScContract;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSight;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScHide;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCloaking;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScChaseWalk;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScRuwach;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFlying;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScWug;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScWugRider;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMadogear;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScXmas;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSummer;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDragon1;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDragon2;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDragon3;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDragon4;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDragon5;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScHanbok;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScOktoberfest;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCartBoost;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScWindWalk;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScIncreaseAgi;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDecreaseAgi;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSlowGrace;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScPlatinumAlter;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMadnessCanceler;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScOneHand;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScTarotCard;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScRefresh;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScLuxAnima;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScHermode;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScGospel;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScInspiration;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBanishingBuster;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScClearance;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDispell;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScElementalChange;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScTidalWeapon;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScWaterScreen;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCpWeapon;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCpArmor;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCpShield;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCpHelm;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMaximizePower;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScEdp;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScAdrenaline;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScWeaponPerfection;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScOverThrustMax;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScPowerThrust;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMagicPower;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScPreserve;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScEnchantBlade;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDeathBound;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMillenniumShield;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCrushStrike;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScRebirth;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSpellBreaker;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScEnchantPoison;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScPoisonReact;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScAspersio;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBenedictum;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScKyrie;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMagnificat;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScGloria;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScLexDivina;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScLexAeterna;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScAdoramus;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDupleLight;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCloakingExceed;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScRollingCutter;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCrossImpact;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDarkClaw;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScReproduce;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScAutoShadowSpell;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScShadowForm;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFatalMenace;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScReadingSpellBook;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFreezingSpell;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSummonBall;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScGentleTouch;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScRisingDragon;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScWargRider;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFearBreeze;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScAimedBolt;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSongOfLutie;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDrumOnTheBattlefield;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBlessing;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFoodStr;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFoodAgi;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFoodVit;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFoodInt;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFoodDex;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFoodLuk;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScEndure;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScAutoguard;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScDefender;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScReflectShield;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScPrestige;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBanding;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScLastStand;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScGatlingFever;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScHeatBarrel;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScAntiMaterialBlast;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScEternalChain;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScShadowLeap;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMirrorImage;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScKunaiExplosion;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSwirlingPetal;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCrossSlash;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCatnipMeteor;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScPickyPeck;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScScarOfTarou;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSolarBurst;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFullMoonKick;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFallingStar;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSoulReap;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScCurseOfSoul;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSoulUnity;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScPalletPaint;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScPyrotechnic;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScThornTrap;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScBloodSucker;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScSporeExplosion;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMasquerade;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScFullThrottle;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScReboundShield;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScStripAccessory;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScInvisibility;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ScMagicalBullet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArchId {
    #[allow(nonstandard_style)]
    Player,
    #[allow(nonstandard_style)]
    MonsterNormal,
    #[allow(nonstandard_style)]
    MonsterBoss,
    #[allow(nonstandard_style)]
    Homunculus,
    #[allow(nonstandard_style)]
    Mercenary,
    #[allow(nonstandard_style)]
    Pet,
    #[allow(nonstandard_style)]
    Elemental,
    #[allow(nonstandard_style)]
    WugRider,
    #[allow(nonstandard_style)]
    MadogearUser,
    #[allow(nonstandard_style)]
    GeneticCart,
    #[allow(nonstandard_style)]
    RuneKnight,
    #[allow(nonstandard_style)]
    RoyalGuard,
    #[allow(nonstandard_style)]
    Mechanic,
    #[allow(nonstandard_style)]
    GuillotineCross,
    #[allow(nonstandard_style)]
    ShadowChaser,
    #[allow(nonstandard_style)]
    Sorcerer,
    #[allow(nonstandard_style)]
    Warlock,
    #[allow(nonstandard_style)]
    Archbishop,
    #[allow(nonstandard_style)]
    Sura,
    #[allow(nonstandard_style)]
    Ranger,
    #[allow(nonstandard_style)]
    MinstrelWanderer,
    #[allow(nonstandard_style)]
    SuperNovice,
    #[allow(nonstandard_style)]
    Gunslinger,
    #[allow(nonstandard_style)]
    Ninja,
    #[allow(nonstandard_style)]
    KagerouOboro,
    #[allow(nonstandard_style)]
    Rebellion,
    #[allow(nonstandard_style)]
    Summoner,
    #[allow(nonstandard_style)]
    StarEmperor,
    #[allow(nonstandard_style)]
    SoulReaper,
    #[allow(nonstandard_style)]
    Dummy,
}
impl ArchId {
    #[inline]
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
pub const ARCH_COUNT: usize = 30;
#[allow(nonstandard_style)]
pub enum ArchEntityRefs {
    #[allow(nonstandard_style)]
    Player(PlayerEntityRefs),
    #[allow(nonstandard_style)]
    MonsterNormal(MonsterNormalEntityRefs),
    #[allow(nonstandard_style)]
    MonsterBoss(MonsterBossEntityRefs),
    #[allow(nonstandard_style)]
    Homunculus(HomunculusEntityRefs),
    #[allow(nonstandard_style)]
    Mercenary(MercenaryEntityRefs),
    #[allow(nonstandard_style)]
    Pet(PetEntityRefs),
    #[allow(nonstandard_style)]
    Elemental(ElementalEntityRefs),
    #[allow(nonstandard_style)]
    WugRider(WugRiderEntityRefs),
    #[allow(nonstandard_style)]
    MadogearUser(MadogearUserEntityRefs),
    #[allow(nonstandard_style)]
    GeneticCart(GeneticCartEntityRefs),
    #[allow(nonstandard_style)]
    RuneKnight(RuneKnightEntityRefs),
    #[allow(nonstandard_style)]
    RoyalGuard(RoyalGuardEntityRefs),
    #[allow(nonstandard_style)]
    Mechanic(MechanicEntityRefs),
    #[allow(nonstandard_style)]
    GuillotineCross(GuillotineCrossEntityRefs),
    #[allow(nonstandard_style)]
    ShadowChaser(ShadowChaserEntityRefs),
    #[allow(nonstandard_style)]
    Sorcerer(SorcererEntityRefs),
    #[allow(nonstandard_style)]
    Warlock(WarlockEntityRefs),
    #[allow(nonstandard_style)]
    Archbishop(ArchbishopEntityRefs),
    #[allow(nonstandard_style)]
    Sura(SuraEntityRefs),
    #[allow(nonstandard_style)]
    Ranger(RangerEntityRefs),
    #[allow(nonstandard_style)]
    MinstrelWanderer(MinstrelWandererEntityRefs),
    #[allow(nonstandard_style)]
    SuperNovice(SuperNoviceEntityRefs),
    #[allow(nonstandard_style)]
    Gunslinger(GunslingerEntityRefs),
    #[allow(nonstandard_style)]
    Ninja(NinjaEntityRefs),
    #[allow(nonstandard_style)]
    KagerouOboro(KagerouOboroEntityRefs),
    #[allow(nonstandard_style)]
    Rebellion(RebellionEntityRefs),
    #[allow(nonstandard_style)]
    Summoner(SummonerEntityRefs),
    #[allow(nonstandard_style)]
    StarEmperor(StarEmperorEntityRefs),
    #[allow(nonstandard_style)]
    SoulReaper(SoulReaperEntityRefs),
    #[allow(nonstandard_style)]
    Dummy(DummyEntityRefs),
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Player,{}
,{
    Position,Velocity,Health,ScBerserk,ScQuicken,ScAngelus,ScAdrenaline,ScInspiration
});
#[allow(nonstandard_style)]
pub struct PlayerEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub Health: *mut Health,
    pub ScBerserk: *mut ScBerserk,
    pub ScQuicken: *mut ScQuicken,
    pub ScAngelus: *mut ScAngelus,
    pub ScAdrenaline: *mut ScAdrenaline,
    pub ScInspiration: *mut ScInspiration,
}
#[allow(nonstandard_style)]
pub struct Player {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<PlayerComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Player {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(PlayerComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        Health: Health,
        ScBerserk: ScBerserk,
        ScQuicken: ScQuicken,
        ScAngelus: ScAngelus,
        ScAdrenaline: ScAdrenaline,
        ScInspiration: ScInspiration,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            Health,
            ScBerserk,
            ScQuicken,
            ScAngelus,
            ScAdrenaline,
            ScInspiration,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Player.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Player.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Health(&self) -> &[Health] {
        unsafe { (*self.storage.get()).Health() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScBerserk(&self) -> &[ScBerserk] {
        unsafe { (*self.storage.get()).ScBerserk() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScQuicken(&self) -> &[ScQuicken] {
        unsafe { (*self.storage.get()).ScQuicken() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScAngelus(&self) -> &[ScAngelus] {
        unsafe { (*self.storage.get()).ScAngelus() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScAdrenaline(&self) -> &[ScAdrenaline] {
        unsafe { (*self.storage.get()).ScAdrenaline() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScInspiration(&self) -> &[ScInspiration] {
        unsafe { (*self.storage.get()).ScInspiration() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(MonsterNormal,{}
,{
    Position,Velocity,Health,ScPoison,ScBleeding,ScCurse
});
#[allow(nonstandard_style)]
pub struct MonsterNormalEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub Health: *mut Health,
    pub ScPoison: *mut ScPoison,
    pub ScBleeding: *mut ScBleeding,
    pub ScCurse: *mut ScCurse,
}
#[allow(nonstandard_style)]
pub struct MonsterNormal {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<MonsterNormalComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl MonsterNormal {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(MonsterNormalComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        Health: Health,
        ScPoison: ScPoison,
        ScBleeding: ScBleeding,
        ScCurse: ScCurse,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(Position, Velocity, Health, ScPoison, ScBleeding, ScCurse);
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::MonsterNormal.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::MonsterNormal.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Health(&self) -> &[Health] {
        unsafe { (*self.storage.get()).Health() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScPoison(&self) -> &[ScPoison] {
        unsafe { (*self.storage.get()).ScPoison() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScBleeding(&self) -> &[ScBleeding] {
        unsafe { (*self.storage.get()).ScBleeding() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScCurse(&self) -> &[ScCurse] {
        unsafe { (*self.storage.get()).ScCurse() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(MonsterBoss,{}
,{
    Position,Velocity,Health,ScStone,ScFreeze,ScStun,ScSleep,ScDeepSleep
});
#[allow(nonstandard_style)]
pub struct MonsterBossEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub Health: *mut Health,
    pub ScStone: *mut ScStone,
    pub ScFreeze: *mut ScFreeze,
    pub ScStun: *mut ScStun,
    pub ScSleep: *mut ScSleep,
    pub ScDeepSleep: *mut ScDeepSleep,
}
#[allow(nonstandard_style)]
pub struct MonsterBoss {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<MonsterBossComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl MonsterBoss {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(MonsterBossComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        Health: Health,
        ScStone: ScStone,
        ScFreeze: ScFreeze,
        ScStun: ScStun,
        ScSleep: ScSleep,
        ScDeepSleep: ScDeepSleep,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            Health,
            ScStone,
            ScFreeze,
            ScStun,
            ScSleep,
            ScDeepSleep,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::MonsterBoss.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::MonsterBoss.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Health(&self) -> &[Health] {
        unsafe { (*self.storage.get()).Health() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScStone(&self) -> &[ScStone] {
        unsafe { (*self.storage.get()).ScStone() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScFreeze(&self) -> &[ScFreeze] {
        unsafe { (*self.storage.get()).ScFreeze() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScStun(&self) -> &[ScStun] {
        unsafe { (*self.storage.get()).ScStun() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSleep(&self) -> &[ScSleep] {
        unsafe { (*self.storage.get()).ScSleep() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScDeepSleep(&self) -> &[ScDeepSleep] {
        unsafe { (*self.storage.get()).ScDeepSleep() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Homunculus,{}
,{
    Position,Velocity,ScSoulLink,ScKaite,ScKyrie
});
#[allow(nonstandard_style)]
pub struct HomunculusEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScSoulLink: *mut ScSoulLink,
    pub ScKaite: *mut ScKaite,
    pub ScKyrie: *mut ScKyrie,
}
#[allow(nonstandard_style)]
pub struct Homunculus {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<HomunculusComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Homunculus {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(HomunculusComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScSoulLink: ScSoulLink,
        ScKaite: ScKaite,
        ScKyrie: ScKyrie,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(Position, Velocity, ScSoulLink, ScKaite, ScKyrie);
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Homunculus.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Homunculus.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSoulLink(&self) -> &[ScSoulLink] {
        unsafe { (*self.storage.get()).ScSoulLink() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScKaite(&self) -> &[ScKaite] {
        unsafe { (*self.storage.get()).ScKaite() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScKyrie(&self) -> &[ScKyrie] {
        unsafe { (*self.storage.get()).ScKyrie() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Mercenary,{}
,{
    Position,Velocity,Health,ScMagnificat,ScGloria
});
#[allow(nonstandard_style)]
pub struct MercenaryEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub Health: *mut Health,
    pub ScMagnificat: *mut ScMagnificat,
    pub ScGloria: *mut ScGloria,
}
#[allow(nonstandard_style)]
pub struct Mercenary {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<MercenaryComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Mercenary {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(MercenaryComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        Health: Health,
        ScMagnificat: ScMagnificat,
        ScGloria: ScGloria,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(Position, Velocity, Health, ScMagnificat, ScGloria);
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Mercenary.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Mercenary.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Health(&self) -> &[Health] {
        unsafe { (*self.storage.get()).Health() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScMagnificat(&self) -> &[ScMagnificat] {
        unsafe { (*self.storage.get()).ScMagnificat() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScGloria(&self) -> &[ScGloria] {
        unsafe { (*self.storage.get()).ScGloria() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Pet,{}
,{
    Position,Velocity,ScEndure,ScAutoguard
});
#[allow(nonstandard_style)]
pub struct PetEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScEndure: *mut ScEndure,
    pub ScAutoguard: *mut ScAutoguard,
}
#[allow(nonstandard_style)]
pub struct Pet {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<PetComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Pet {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(PetComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScEndure: ScEndure,
        ScAutoguard: ScAutoguard,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(Position, Velocity, ScEndure, ScAutoguard);
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Pet.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Pet.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScEndure(&self) -> &[ScEndure] {
        unsafe { (*self.storage.get()).ScEndure() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScAutoguard(&self) -> &[ScAutoguard] {
        unsafe { (*self.storage.get()).ScAutoguard() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Elemental,{}
,{
    Position,Velocity,ScElementalChange,ScTidalWeapon,ScWaterScreen
});
#[allow(nonstandard_style)]
pub struct ElementalEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScElementalChange: *mut ScElementalChange,
    pub ScTidalWeapon: *mut ScTidalWeapon,
    pub ScWaterScreen: *mut ScWaterScreen,
}
#[allow(nonstandard_style)]
pub struct Elemental {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<ElementalComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Elemental {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(ElementalComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScElementalChange: ScElementalChange,
        ScTidalWeapon: ScTidalWeapon,
        ScWaterScreen: ScWaterScreen,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScElementalChange,
            ScTidalWeapon,
            ScWaterScreen,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Elemental.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Elemental.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScElementalChange(&self) -> &[ScElementalChange] {
        unsafe { (*self.storage.get()).ScElementalChange() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScTidalWeapon(&self) -> &[ScTidalWeapon] {
        unsafe { (*self.storage.get()).ScTidalWeapon() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScWaterScreen(&self) -> &[ScWaterScreen] {
        unsafe { (*self.storage.get()).ScWaterScreen() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(WugRider,{}
,{
    Position,Velocity,ScWugRider,ScCartBoost,ScIncreaseAgi
});
#[allow(nonstandard_style)]
pub struct WugRiderEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScWugRider: *mut ScWugRider,
    pub ScCartBoost: *mut ScCartBoost,
    pub ScIncreaseAgi: *mut ScIncreaseAgi,
}
#[allow(nonstandard_style)]
pub struct WugRider {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<WugRiderComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl WugRider {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(WugRiderComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScWugRider: ScWugRider,
        ScCartBoost: ScCartBoost,
        ScIncreaseAgi: ScIncreaseAgi,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(Position, Velocity, ScWugRider, ScCartBoost, ScIncreaseAgi);
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::WugRider.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::WugRider.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScWugRider(&self) -> &[ScWugRider] {
        unsafe { (*self.storage.get()).ScWugRider() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScCartBoost(&self) -> &[ScCartBoost] {
        unsafe { (*self.storage.get()).ScCartBoost() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScIncreaseAgi(&self) -> &[ScIncreaseAgi] {
        unsafe { (*self.storage.get()).ScIncreaseAgi() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(MadogearUser,{}
,{
    Position,Velocity,ScMadogear,ScHeatBarrel,ScMagicalBullet
});
#[allow(nonstandard_style)]
pub struct MadogearUserEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScMadogear: *mut ScMadogear,
    pub ScHeatBarrel: *mut ScHeatBarrel,
    pub ScMagicalBullet: *mut ScMagicalBullet,
}
#[allow(nonstandard_style)]
pub struct MadogearUser {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<MadogearUserComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl MadogearUser {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(MadogearUserComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScMadogear: ScMadogear,
        ScHeatBarrel: ScHeatBarrel,
        ScMagicalBullet: ScMagicalBullet,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScMadogear,
            ScHeatBarrel,
            ScMagicalBullet,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::MadogearUser.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::MadogearUser.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScMadogear(&self) -> &[ScMadogear] {
        unsafe { (*self.storage.get()).ScMadogear() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScHeatBarrel(&self) -> &[ScHeatBarrel] {
        unsafe { (*self.storage.get()).ScHeatBarrel() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScMagicalBullet(&self) -> &[ScMagicalBullet] {
        unsafe { (*self.storage.get()).ScMagicalBullet() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(GeneticCart,{}
,{
    Position,Velocity,ScCartBoost,ScPyrotechnic,ScThornTrap
});
#[allow(nonstandard_style)]
pub struct GeneticCartEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScCartBoost: *mut ScCartBoost,
    pub ScPyrotechnic: *mut ScPyrotechnic,
    pub ScThornTrap: *mut ScThornTrap,
}
#[allow(nonstandard_style)]
pub struct GeneticCart {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<GeneticCartComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl GeneticCart {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(GeneticCartComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScCartBoost: ScCartBoost,
        ScPyrotechnic: ScPyrotechnic,
        ScThornTrap: ScThornTrap,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(Position, Velocity, ScCartBoost, ScPyrotechnic, ScThornTrap);
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::GeneticCart.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::GeneticCart.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScCartBoost(&self) -> &[ScCartBoost] {
        unsafe { (*self.storage.get()).ScCartBoost() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScPyrotechnic(&self) -> &[ScPyrotechnic] {
        unsafe { (*self.storage.get()).ScPyrotechnic() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScThornTrap(&self) -> &[ScThornTrap] {
        unsafe { (*self.storage.get()).ScThornTrap() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(RuneKnight,{}
,{
    Position,Velocity,ScEnchantBlade,ScDeathBound,ScMillenniumShield,ScRebirth
});
#[allow(nonstandard_style)]
pub struct RuneKnightEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScEnchantBlade: *mut ScEnchantBlade,
    pub ScDeathBound: *mut ScDeathBound,
    pub ScMillenniumShield: *mut ScMillenniumShield,
    pub ScRebirth: *mut ScRebirth,
}
#[allow(nonstandard_style)]
pub struct RuneKnight {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<RuneKnightComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl RuneKnight {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(RuneKnightComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScEnchantBlade: ScEnchantBlade,
        ScDeathBound: ScDeathBound,
        ScMillenniumShield: ScMillenniumShield,
        ScRebirth: ScRebirth,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScEnchantBlade,
            ScDeathBound,
            ScMillenniumShield,
            ScRebirth,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::RuneKnight.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::RuneKnight.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScEnchantBlade(&self) -> &[ScEnchantBlade] {
        unsafe { (*self.storage.get()).ScEnchantBlade() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScDeathBound(&self) -> &[ScDeathBound] {
        unsafe { (*self.storage.get()).ScDeathBound() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScMillenniumShield(&self) -> &[ScMillenniumShield] {
        unsafe { (*self.storage.get()).ScMillenniumShield() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScRebirth(&self) -> &[ScRebirth] {
        unsafe { (*self.storage.get()).ScRebirth() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(RoyalGuard,{}
,{
    Position,Velocity,ScDefender,ScReflectShield,ScPrestige,ScBanding
});
#[allow(nonstandard_style)]
pub struct RoyalGuardEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScDefender: *mut ScDefender,
    pub ScReflectShield: *mut ScReflectShield,
    pub ScPrestige: *mut ScPrestige,
    pub ScBanding: *mut ScBanding,
}
#[allow(nonstandard_style)]
pub struct RoyalGuard {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<RoyalGuardComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl RoyalGuard {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(RoyalGuardComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScDefender: ScDefender,
        ScReflectShield: ScReflectShield,
        ScPrestige: ScPrestige,
        ScBanding: ScBanding,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScDefender,
            ScReflectShield,
            ScPrestige,
            ScBanding,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::RoyalGuard.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::RoyalGuard.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScDefender(&self) -> &[ScDefender] {
        unsafe { (*self.storage.get()).ScDefender() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScReflectShield(&self) -> &[ScReflectShield] {
        unsafe { (*self.storage.get()).ScReflectShield() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScPrestige(&self) -> &[ScPrestige] {
        unsafe { (*self.storage.get()).ScPrestige() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScBanding(&self) -> &[ScBanding] {
        unsafe { (*self.storage.get()).ScBanding() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Mechanic,{}
,{
    Position,Velocity,ScMadogear,ScPowerThrust,ScWeaponPerfection
});
#[allow(nonstandard_style)]
pub struct MechanicEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScMadogear: *mut ScMadogear,
    pub ScPowerThrust: *mut ScPowerThrust,
    pub ScWeaponPerfection: *mut ScWeaponPerfection,
}
#[allow(nonstandard_style)]
pub struct Mechanic {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<MechanicComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Mechanic {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(MechanicComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScMadogear: ScMadogear,
        ScPowerThrust: ScPowerThrust,
        ScWeaponPerfection: ScWeaponPerfection,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScMadogear,
            ScPowerThrust,
            ScWeaponPerfection,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Mechanic.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Mechanic.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScMadogear(&self) -> &[ScMadogear] {
        unsafe { (*self.storage.get()).ScMadogear() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScPowerThrust(&self) -> &[ScPowerThrust] {
        unsafe { (*self.storage.get()).ScPowerThrust() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScWeaponPerfection(&self) -> &[ScWeaponPerfection] {
        unsafe { (*self.storage.get()).ScWeaponPerfection() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(GuillotineCross,{}
,{
    Position,Velocity,ScCloakingExceed,ScRollingCutter,ScCrossImpact,ScDarkClaw
});
#[allow(nonstandard_style)]
pub struct GuillotineCrossEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScCloakingExceed: *mut ScCloakingExceed,
    pub ScRollingCutter: *mut ScRollingCutter,
    pub ScCrossImpact: *mut ScCrossImpact,
    pub ScDarkClaw: *mut ScDarkClaw,
}
#[allow(nonstandard_style)]
pub struct GuillotineCross {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<GuillotineCrossComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl GuillotineCross {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(GuillotineCrossComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScCloakingExceed: ScCloakingExceed,
        ScRollingCutter: ScRollingCutter,
        ScCrossImpact: ScCrossImpact,
        ScDarkClaw: ScDarkClaw,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScCloakingExceed,
            ScRollingCutter,
            ScCrossImpact,
            ScDarkClaw,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::GuillotineCross.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::GuillotineCross.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScCloakingExceed(&self) -> &[ScCloakingExceed] {
        unsafe { (*self.storage.get()).ScCloakingExceed() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScRollingCutter(&self) -> &[ScRollingCutter] {
        unsafe { (*self.storage.get()).ScRollingCutter() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScCrossImpact(&self) -> &[ScCrossImpact] {
        unsafe { (*self.storage.get()).ScCrossImpact() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScDarkClaw(&self) -> &[ScDarkClaw] {
        unsafe { (*self.storage.get()).ScDarkClaw() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(ShadowChaser,{}
,{
    Position,Velocity,ScReproduce,ScAutoShadowSpell,ScShadowForm,ScFatalMenace
});
#[allow(nonstandard_style)]
pub struct ShadowChaserEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScReproduce: *mut ScReproduce,
    pub ScAutoShadowSpell: *mut ScAutoShadowSpell,
    pub ScShadowForm: *mut ScShadowForm,
    pub ScFatalMenace: *mut ScFatalMenace,
}
#[allow(nonstandard_style)]
pub struct ShadowChaser {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<ShadowChaserComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl ShadowChaser {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(ShadowChaserComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScReproduce: ScReproduce,
        ScAutoShadowSpell: ScAutoShadowSpell,
        ScShadowForm: ScShadowForm,
        ScFatalMenace: ScFatalMenace,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScReproduce,
            ScAutoShadowSpell,
            ScShadowForm,
            ScFatalMenace,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::ShadowChaser.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::ShadowChaser.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScReproduce(&self) -> &[ScReproduce] {
        unsafe { (*self.storage.get()).ScReproduce() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScAutoShadowSpell(&self) -> &[ScAutoShadowSpell] {
        unsafe { (*self.storage.get()).ScAutoShadowSpell() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScShadowForm(&self) -> &[ScShadowForm] {
        unsafe { (*self.storage.get()).ScShadowForm() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScFatalMenace(&self) -> &[ScFatalMenace] {
        unsafe { (*self.storage.get()).ScFatalMenace() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Sorcerer,{}
,{
    Position,Velocity,ScSpellBreaker,ScSoulLink,ScPreserve
});
#[allow(nonstandard_style)]
pub struct SorcererEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScSpellBreaker: *mut ScSpellBreaker,
    pub ScSoulLink: *mut ScSoulLink,
    pub ScPreserve: *mut ScPreserve,
}
#[allow(nonstandard_style)]
pub struct Sorcerer {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<SorcererComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Sorcerer {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(SorcererComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScSpellBreaker: ScSpellBreaker,
        ScSoulLink: ScSoulLink,
        ScPreserve: ScPreserve,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(Position, Velocity, ScSpellBreaker, ScSoulLink, ScPreserve);
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Sorcerer.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Sorcerer.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSpellBreaker(&self) -> &[ScSpellBreaker] {
        unsafe { (*self.storage.get()).ScSpellBreaker() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSoulLink(&self) -> &[ScSoulLink] {
        unsafe { (*self.storage.get()).ScSoulLink() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScPreserve(&self) -> &[ScPreserve] {
        unsafe { (*self.storage.get()).ScPreserve() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Warlock,{}
,{
    Position,Velocity,ScReadingSpellBook,ScFreezingSpell,ScSummonBall
});
#[allow(nonstandard_style)]
pub struct WarlockEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScReadingSpellBook: *mut ScReadingSpellBook,
    pub ScFreezingSpell: *mut ScFreezingSpell,
    pub ScSummonBall: *mut ScSummonBall,
}
#[allow(nonstandard_style)]
pub struct Warlock {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<WarlockComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Warlock {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(WarlockComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScReadingSpellBook: ScReadingSpellBook,
        ScFreezingSpell: ScFreezingSpell,
        ScSummonBall: ScSummonBall,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScReadingSpellBook,
            ScFreezingSpell,
            ScSummonBall,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Warlock.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Warlock.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScReadingSpellBook(&self) -> &[ScReadingSpellBook] {
        unsafe { (*self.storage.get()).ScReadingSpellBook() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScFreezingSpell(&self) -> &[ScFreezingSpell] {
        unsafe { (*self.storage.get()).ScFreezingSpell() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSummonBall(&self) -> &[ScSummonBall] {
        unsafe { (*self.storage.get()).ScSummonBall() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Archbishop,{}
,{
    Position,Velocity,ScAssumptio,ScKyrie,ScMagnificat,ScBenedictum
});
#[allow(nonstandard_style)]
pub struct ArchbishopEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScAssumptio: *mut ScAssumptio,
    pub ScKyrie: *mut ScKyrie,
    pub ScMagnificat: *mut ScMagnificat,
    pub ScBenedictum: *mut ScBenedictum,
}
#[allow(nonstandard_style)]
pub struct Archbishop {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<ArchbishopComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Archbishop {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(ArchbishopComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScAssumptio: ScAssumptio,
        ScKyrie: ScKyrie,
        ScMagnificat: ScMagnificat,
        ScBenedictum: ScBenedictum,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScAssumptio,
            ScKyrie,
            ScMagnificat,
            ScBenedictum,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Archbishop.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Archbishop.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScAssumptio(&self) -> &[ScAssumptio] {
        unsafe { (*self.storage.get()).ScAssumptio() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScKyrie(&self) -> &[ScKyrie] {
        unsafe { (*self.storage.get()).ScKyrie() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScMagnificat(&self) -> &[ScMagnificat] {
        unsafe { (*self.storage.get()).ScMagnificat() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScBenedictum(&self) -> &[ScBenedictum] {
        unsafe { (*self.storage.get()).ScBenedictum() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Sura,{}
,{
    Position,Velocity,ScGentleTouch,ScPowerThrust,ScRisingDragon
});
#[allow(nonstandard_style)]
pub struct SuraEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScGentleTouch: *mut ScGentleTouch,
    pub ScPowerThrust: *mut ScPowerThrust,
    pub ScRisingDragon: *mut ScRisingDragon,
}
#[allow(nonstandard_style)]
pub struct Sura {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<SuraComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Sura {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(SuraComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScGentleTouch: ScGentleTouch,
        ScPowerThrust: ScPowerThrust,
        ScRisingDragon: ScRisingDragon,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScGentleTouch,
            ScPowerThrust,
            ScRisingDragon,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Sura.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Sura.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScGentleTouch(&self) -> &[ScGentleTouch] {
        unsafe { (*self.storage.get()).ScGentleTouch() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScPowerThrust(&self) -> &[ScPowerThrust] {
        unsafe { (*self.storage.get()).ScPowerThrust() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScRisingDragon(&self) -> &[ScRisingDragon] {
        unsafe { (*self.storage.get()).ScRisingDragon() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Ranger,{}
,{
    Position,Velocity,ScWargRider,ScFearBreeze,ScAimedBolt
});
#[allow(nonstandard_style)]
pub struct RangerEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScWargRider: *mut ScWargRider,
    pub ScFearBreeze: *mut ScFearBreeze,
    pub ScAimedBolt: *mut ScAimedBolt,
}
#[allow(nonstandard_style)]
pub struct Ranger {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<RangerComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Ranger {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(RangerComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScWargRider: ScWargRider,
        ScFearBreeze: ScFearBreeze,
        ScAimedBolt: ScAimedBolt,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(Position, Velocity, ScWargRider, ScFearBreeze, ScAimedBolt);
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Ranger.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Ranger.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScWargRider(&self) -> &[ScWargRider] {
        unsafe { (*self.storage.get()).ScWargRider() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScFearBreeze(&self) -> &[ScFearBreeze] {
        unsafe { (*self.storage.get()).ScFearBreeze() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScAimedBolt(&self) -> &[ScAimedBolt] {
        unsafe { (*self.storage.get()).ScAimedBolt() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(MinstrelWanderer,{}
,{
    Position,Velocity,ScSongOfLutie,ScDrumOnTheBattlefield,ScSaturdayNightFever
});
#[allow(nonstandard_style)]
pub struct MinstrelWandererEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScSongOfLutie: *mut ScSongOfLutie,
    pub ScDrumOnTheBattlefield: *mut ScDrumOnTheBattlefield,
    pub ScSaturdayNightFever: *mut ScSaturdayNightFever,
}
#[allow(nonstandard_style)]
pub struct MinstrelWanderer {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<MinstrelWandererComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl MinstrelWanderer {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(MinstrelWandererComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScSongOfLutie: ScSongOfLutie,
        ScDrumOnTheBattlefield: ScDrumOnTheBattlefield,
        ScSaturdayNightFever: ScSaturdayNightFever,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScSongOfLutie,
            ScDrumOnTheBattlefield,
            ScSaturdayNightFever,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::MinstrelWanderer.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::MinstrelWanderer.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSongOfLutie(&self) -> &[ScSongOfLutie] {
        unsafe { (*self.storage.get()).ScSongOfLutie() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScDrumOnTheBattlefield(&self) -> &[ScDrumOnTheBattlefield] {
        unsafe { (*self.storage.get()).ScDrumOnTheBattlefield() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSaturdayNightFever(&self) -> &[ScSaturdayNightFever] {
        unsafe { (*self.storage.get()).ScSaturdayNightFever() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(SuperNovice,{}
,{
    Position,Velocity,ScAngelus,ScBlessing,ScIncreaseAgi,ScFoodStr,ScFoodInt,ScFoodLuk
});
#[allow(nonstandard_style)]
pub struct SuperNoviceEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScAngelus: *mut ScAngelus,
    pub ScBlessing: *mut ScBlessing,
    pub ScIncreaseAgi: *mut ScIncreaseAgi,
    pub ScFoodStr: *mut ScFoodStr,
    pub ScFoodInt: *mut ScFoodInt,
    pub ScFoodLuk: *mut ScFoodLuk,
}
#[allow(nonstandard_style)]
pub struct SuperNovice {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<SuperNoviceComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl SuperNovice {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(SuperNoviceComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScAngelus: ScAngelus,
        ScBlessing: ScBlessing,
        ScIncreaseAgi: ScIncreaseAgi,
        ScFoodStr: ScFoodStr,
        ScFoodInt: ScFoodInt,
        ScFoodLuk: ScFoodLuk,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScAngelus,
            ScBlessing,
            ScIncreaseAgi,
            ScFoodStr,
            ScFoodInt,
            ScFoodLuk,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::SuperNovice.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::SuperNovice.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScAngelus(&self) -> &[ScAngelus] {
        unsafe { (*self.storage.get()).ScAngelus() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScBlessing(&self) -> &[ScBlessing] {
        unsafe { (*self.storage.get()).ScBlessing() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScIncreaseAgi(&self) -> &[ScIncreaseAgi] {
        unsafe { (*self.storage.get()).ScIncreaseAgi() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScFoodStr(&self) -> &[ScFoodStr] {
        unsafe { (*self.storage.get()).ScFoodStr() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScFoodInt(&self) -> &[ScFoodInt] {
        unsafe { (*self.storage.get()).ScFoodInt() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScFoodLuk(&self) -> &[ScFoodLuk] {
        unsafe { (*self.storage.get()).ScFoodLuk() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Gunslinger,{}
,{
    Position,Velocity,ScMadnessCanceler,ScLastStand,ScGatlingFever
});
#[allow(nonstandard_style)]
pub struct GunslingerEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScMadnessCanceler: *mut ScMadnessCanceler,
    pub ScLastStand: *mut ScLastStand,
    pub ScGatlingFever: *mut ScGatlingFever,
}
#[allow(nonstandard_style)]
pub struct Gunslinger {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<GunslingerComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Gunslinger {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(GunslingerComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScMadnessCanceler: ScMadnessCanceler,
        ScLastStand: ScLastStand,
        ScGatlingFever: ScGatlingFever,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScMadnessCanceler,
            ScLastStand,
            ScGatlingFever,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Gunslinger.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Gunslinger.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScMadnessCanceler(&self) -> &[ScMadnessCanceler] {
        unsafe { (*self.storage.get()).ScMadnessCanceler() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScLastStand(&self) -> &[ScLastStand] {
        unsafe { (*self.storage.get()).ScLastStand() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScGatlingFever(&self) -> &[ScGatlingFever] {
        unsafe { (*self.storage.get()).ScGatlingFever() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Ninja,{}
,{
    Position,Velocity,ScCloaking,ScShadowLeap,ScMirrorImage
});
#[allow(nonstandard_style)]
pub struct NinjaEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScCloaking: *mut ScCloaking,
    pub ScShadowLeap: *mut ScShadowLeap,
    pub ScMirrorImage: *mut ScMirrorImage,
}
#[allow(nonstandard_style)]
pub struct Ninja {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<NinjaComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Ninja {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(NinjaComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScCloaking: ScCloaking,
        ScShadowLeap: ScShadowLeap,
        ScMirrorImage: ScMirrorImage,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(Position, Velocity, ScCloaking, ScShadowLeap, ScMirrorImage);
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Ninja.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Ninja.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScCloaking(&self) -> &[ScCloaking] {
        unsafe { (*self.storage.get()).ScCloaking() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScShadowLeap(&self) -> &[ScShadowLeap] {
        unsafe { (*self.storage.get()).ScShadowLeap() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScMirrorImage(&self) -> &[ScMirrorImage] {
        unsafe { (*self.storage.get()).ScMirrorImage() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(KagerouOboro,{}
,{
    Position,Velocity,ScKunaiExplosion,ScSwirlingPetal,ScCrossSlash
});
#[allow(nonstandard_style)]
pub struct KagerouOboroEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScKunaiExplosion: *mut ScKunaiExplosion,
    pub ScSwirlingPetal: *mut ScSwirlingPetal,
    pub ScCrossSlash: *mut ScCrossSlash,
}
#[allow(nonstandard_style)]
pub struct KagerouOboro {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<KagerouOboroComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl KagerouOboro {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(KagerouOboroComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScKunaiExplosion: ScKunaiExplosion,
        ScSwirlingPetal: ScSwirlingPetal,
        ScCrossSlash: ScCrossSlash,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScKunaiExplosion,
            ScSwirlingPetal,
            ScCrossSlash,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::KagerouOboro.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::KagerouOboro.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScKunaiExplosion(&self) -> &[ScKunaiExplosion] {
        unsafe { (*self.storage.get()).ScKunaiExplosion() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSwirlingPetal(&self) -> &[ScSwirlingPetal] {
        unsafe { (*self.storage.get()).ScSwirlingPetal() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScCrossSlash(&self) -> &[ScCrossSlash] {
        unsafe { (*self.storage.get()).ScCrossSlash() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Rebellion,{}
,{
    Position,Velocity,ScHeatBarrel,ScAntiMaterialBlast,ScEternalChain
});
#[allow(nonstandard_style)]
pub struct RebellionEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScHeatBarrel: *mut ScHeatBarrel,
    pub ScAntiMaterialBlast: *mut ScAntiMaterialBlast,
    pub ScEternalChain: *mut ScEternalChain,
}
#[allow(nonstandard_style)]
pub struct Rebellion {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<RebellionComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Rebellion {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(RebellionComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScHeatBarrel: ScHeatBarrel,
        ScAntiMaterialBlast: ScAntiMaterialBlast,
        ScEternalChain: ScEternalChain,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScHeatBarrel,
            ScAntiMaterialBlast,
            ScEternalChain,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Rebellion.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Rebellion.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScHeatBarrel(&self) -> &[ScHeatBarrel] {
        unsafe { (*self.storage.get()).ScHeatBarrel() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScAntiMaterialBlast(&self) -> &[ScAntiMaterialBlast] {
        unsafe { (*self.storage.get()).ScAntiMaterialBlast() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScEternalChain(&self) -> &[ScEternalChain] {
        unsafe { (*self.storage.get()).ScEternalChain() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Summoner,{}
,{
    Position,Velocity,ScCatnipMeteor,ScPickyPeck,ScScarOfTarou
});
#[allow(nonstandard_style)]
pub struct SummonerEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScCatnipMeteor: *mut ScCatnipMeteor,
    pub ScPickyPeck: *mut ScPickyPeck,
    pub ScScarOfTarou: *mut ScScarOfTarou,
}
#[allow(nonstandard_style)]
pub struct Summoner {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<SummonerComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Summoner {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(SummonerComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScCatnipMeteor: ScCatnipMeteor,
        ScPickyPeck: ScPickyPeck,
        ScScarOfTarou: ScScarOfTarou,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScCatnipMeteor,
            ScPickyPeck,
            ScScarOfTarou,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Summoner.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Summoner.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScCatnipMeteor(&self) -> &[ScCatnipMeteor] {
        unsafe { (*self.storage.get()).ScCatnipMeteor() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScPickyPeck(&self) -> &[ScPickyPeck] {
        unsafe { (*self.storage.get()).ScPickyPeck() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScScarOfTarou(&self) -> &[ScScarOfTarou] {
        unsafe { (*self.storage.get()).ScScarOfTarou() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(StarEmperor,{}
,{
    Position,Velocity,ScSolarBurst,ScFullMoonKick,ScFallingStar
});
#[allow(nonstandard_style)]
pub struct StarEmperorEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScSolarBurst: *mut ScSolarBurst,
    pub ScFullMoonKick: *mut ScFullMoonKick,
    pub ScFallingStar: *mut ScFallingStar,
}
#[allow(nonstandard_style)]
pub struct StarEmperor {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<StarEmperorComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl StarEmperor {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(StarEmperorComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScSolarBurst: ScSolarBurst,
        ScFullMoonKick: ScFullMoonKick,
        ScFallingStar: ScFallingStar,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position,
            Velocity,
            ScSolarBurst,
            ScFullMoonKick,
            ScFallingStar,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::StarEmperor.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::StarEmperor.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSolarBurst(&self) -> &[ScSolarBurst] {
        unsafe { (*self.storage.get()).ScSolarBurst() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScFullMoonKick(&self) -> &[ScFullMoonKick] {
        unsafe { (*self.storage.get()).ScFullMoonKick() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScFallingStar(&self) -> &[ScFallingStar] {
        unsafe { (*self.storage.get()).ScFallingStar() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(SoulReaper,{}
,{
    Position,Velocity,ScSoulReap,ScCurseOfSoul,ScSoulUnity
});
#[allow(nonstandard_style)]
pub struct SoulReaperEntityRefs {
    pub Position: *mut Position,
    pub Velocity: *mut Velocity,
    pub ScSoulReap: *mut ScSoulReap,
    pub ScCurseOfSoul: *mut ScCurseOfSoul,
    pub ScSoulUnity: *mut ScSoulUnity,
}
#[allow(nonstandard_style)]
pub struct SoulReaper {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<SoulReaperComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl SoulReaper {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(SoulReaperComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Velocity: Velocity,
        ScSoulReap: ScSoulReap,
        ScCurseOfSoul: ScCurseOfSoul,
        ScSoulUnity: ScSoulUnity,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(Position, Velocity, ScSoulReap, ScCurseOfSoul, ScSoulUnity);
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::SoulReaper.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::SoulReaper.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Velocity(&self) -> &[Velocity] {
        unsafe { (*self.storage.get()).Velocity() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSoulReap(&self) -> &[ScSoulReap] {
        unsafe { (*self.storage.get()).ScSoulReap() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScCurseOfSoul(&self) -> &[ScCurseOfSoul] {
        unsafe { (*self.storage.get()).ScCurseOfSoul() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSoulUnity(&self) -> &[ScSoulUnity] {
        unsafe { (*self.storage.get()).ScSoulUnity() }
    }
}
#[allow(nonstandard_style)]
ecs::generate_storage_recursive!(Dummy,{}
,{
    Position,Health,ScStone,ScFreeze,ScStun,ScSleep,ScBurning,ScPoison,ScBleeding,ScCurse
});
#[allow(nonstandard_style)]
pub struct DummyEntityRefs {
    pub Position: *mut Position,
    pub Health: *mut Health,
    pub ScStone: *mut ScStone,
    pub ScFreeze: *mut ScFreeze,
    pub ScStun: *mut ScStun,
    pub ScSleep: *mut ScSleep,
    pub ScBurning: *mut ScBurning,
    pub ScPoison: *mut ScPoison,
    pub ScBleeding: *mut ScBleeding,
    pub ScCurse: *mut ScCurse,
}
#[allow(nonstandard_style)]
pub struct Dummy {
    #[doc = r" Component storage"]
    pub storage: std::cell::UnsafeCell<DummyComponentStorage>,
    #[doc = r" Maps SlotIndex -> DenseIndex"]
    pub slots: Vec<u32>,
    #[doc = r" Maps DenseIndex -> SlotIndex"]
    pub dense_to_slot: Vec<u32>,
    #[doc = r" Recycled slot free list"]
    pub free_slots: std::collections::VecDeque<u32>,
    #[doc = r" Generation counter per slot (u16). Only used in debug builds."]
    #[cfg(debug_assertions)]
    pub slot_generations: Vec<u16>,
}
#[allow(nonstandard_style)]
impl Dummy {
    pub fn new() -> Self {
        Self {
            storage: std::cell::UnsafeCell::new(DummyComponentStorage::new()),
            slots: Vec::new(),
            dense_to_slot: Vec::new(),
            free_slots: std::collections::VecDeque::new(),
            #[cfg(debug_assertions)]
            slot_generations: Vec::new(),
        }
    }
    #[doc = r" Spawn an entity into this archetype."]
    #[doc = r" Returns the handle (slot_index, generation)."]
    #[doc = r" Since you know which arch it comes from, you can access it with zero arch lookup overhead."]
    #[allow(nonstandard_style)]
    pub fn spawn(
        &mut self,
        Position: Position,
        Health: Health,
        ScStone: ScStone,
        ScFreeze: ScFreeze,
        ScStun: ScStun,
        ScSleep: ScSleep,
        ScBurning: ScBurning,
        ScPoison: ScPoison,
        ScBleeding: ScBleeding,
        ScCurse: ScCurse,
    ) -> ecs::Handle {
        let storage = unsafe { &mut *self.storage.get() };
        let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(u32::MAX);
            #[cfg(debug_assertions)]
            self.slot_generations.push(0u16);
            idx
        };
        let dense_index = storage.len as u32;
        let dense_index_usize = storage.len;
        self.slots[slot_index as usize] = dense_index;
        if dense_index_usize < self.dense_to_slot.len() {
            self.dense_to_slot[dense_index_usize] = slot_index;
        } else {
            self.dense_to_slot.push(slot_index);
        }
        storage.push(
            Position, Health, ScStone, ScFreeze, ScStun, ScSleep, ScBurning, ScPoison, ScBleeding,
            ScCurse,
        );
        #[cfg(debug_assertions)]
        let generation = self.slot_generations[slot_index as usize];
        #[cfg(not(debug_assertions))]
        let generation = 0;
        let arch_id = ArchId::Dummy.as_u8();
        ecs::Handle {
            arch_id,
            slot_index,
            generation,
        }
    }
    #[doc = r" Despawn an entity given its handle."]
    #[doc = r" Since you know which arch it comes from, there is no arch lookup overhead."]
    pub fn despawn(&mut self, handle: ecs::Handle) {
        let storage = unsafe { &mut *self.storage.get() };
        debug_assert_eq!(handle.arch_id, ArchId::Dummy.as_u8());
        let slot_index_usize = handle.slot_index as usize;
        debug_assert!(slot_index_usize < self.slots.len());
        let dense_index = self.slots[slot_index_usize];
        let dense_index_usize = dense_index as usize;
        debug_assert_ne!(dense_index, u32::MAX);
        debug_assert!(dense_index_usize < storage.len);
        let last_dense_index_usize = storage.len - 1;
        storage.swap_remove(dense_index_usize);
        let was_last = dense_index_usize == last_dense_index_usize;
        if !was_last {
            let swapped_slot = self.dense_to_slot[last_dense_index_usize];
            self.slots[swapped_slot as usize] = dense_index;
            self.dense_to_slot[dense_index_usize] = swapped_slot;
        }
        self.free_slots.push_back(handle.slot_index);
        #[cfg(debug_assertions)]
        {
            let generation = &mut self.slot_generations[slot_index_usize];
            debug_assert_eq!(handle.generation, *generation);
            *generation = generation.wrapping_add(1);
        }
        self.slots[slot_index_usize] = u32::MAX;
    }
    pub fn dense_len(&self) -> usize {
        unsafe { (*self.storage.get()).len }
    }
    pub fn clear_preserve_capacity(&mut self) {
        let storage = unsafe { &mut *self.storage.get() };
        #[cfg(debug_assertions)]
        for generation in &mut self.slot_generations {
            *generation = generation.wrapping_add(1);
        }
        for slot in &mut self.slots {
            *slot = u32::MAX;
        }
        self.dense_to_slot.clear();
        self.free_slots.clear();
        storage.clear();
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Position(&self) -> &[Position] {
        unsafe { (*self.storage.get()).Position() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn Health(&self) -> &[Health] {
        unsafe { (*self.storage.get()).Health() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScStone(&self) -> &[ScStone] {
        unsafe { (*self.storage.get()).ScStone() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScFreeze(&self) -> &[ScFreeze] {
        unsafe { (*self.storage.get()).ScFreeze() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScStun(&self) -> &[ScStun] {
        unsafe { (*self.storage.get()).ScStun() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScSleep(&self) -> &[ScSleep] {
        unsafe { (*self.storage.get()).ScSleep() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScBurning(&self) -> &[ScBurning] {
        unsafe { (*self.storage.get()).ScBurning() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScPoison(&self) -> &[ScPoison] {
        unsafe { (*self.storage.get()).ScPoison() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScBleeding(&self) -> &[ScBleeding] {
        unsafe { (*self.storage.get()).ScBleeding() }
    }
    #[inline]
    #[allow(nonstandard_style)]
    pub fn ScCurse(&self) -> &[ScCurse] {
        unsafe { (*self.storage.get()).ScCurse() }
    }
}
#[doc = r" The world contains one archetype struct per declared archetype."]
pub struct StressWorld {
    pub Player: Player,
    pub MonsterNormal: MonsterNormal,
    pub MonsterBoss: MonsterBoss,
    pub Homunculus: Homunculus,
    pub Mercenary: Mercenary,
    pub Pet: Pet,
    pub Elemental: Elemental,
    pub WugRider: WugRider,
    pub MadogearUser: MadogearUser,
    pub GeneticCart: GeneticCart,
    pub RuneKnight: RuneKnight,
    pub RoyalGuard: RoyalGuard,
    pub Mechanic: Mechanic,
    pub GuillotineCross: GuillotineCross,
    pub ShadowChaser: ShadowChaser,
    pub Sorcerer: Sorcerer,
    pub Warlock: Warlock,
    pub Archbishop: Archbishop,
    pub Sura: Sura,
    pub Ranger: Ranger,
    pub MinstrelWanderer: MinstrelWanderer,
    pub SuperNovice: SuperNovice,
    pub Gunslinger: Gunslinger,
    pub Ninja: Ninja,
    pub KagerouOboro: KagerouOboro,
    pub Rebellion: Rebellion,
    pub Summoner: Summoner,
    pub StarEmperor: StarEmperor,
    pub SoulReaper: SoulReaper,
    pub Dummy: Dummy,
}
impl StressWorld {
    pub fn new() -> Self {
        Self {
            Player: Player::new(),
            MonsterNormal: MonsterNormal::new(),
            MonsterBoss: MonsterBoss::new(),
            Homunculus: Homunculus::new(),
            Mercenary: Mercenary::new(),
            Pet: Pet::new(),
            Elemental: Elemental::new(),
            WugRider: WugRider::new(),
            MadogearUser: MadogearUser::new(),
            GeneticCart: GeneticCart::new(),
            RuneKnight: RuneKnight::new(),
            RoyalGuard: RoyalGuard::new(),
            Mechanic: Mechanic::new(),
            GuillotineCross: GuillotineCross::new(),
            ShadowChaser: ShadowChaser::new(),
            Sorcerer: Sorcerer::new(),
            Warlock: Warlock::new(),
            Archbishop: Archbishop::new(),
            Sura: Sura::new(),
            Ranger: Ranger::new(),
            MinstrelWanderer: MinstrelWanderer::new(),
            SuperNovice: SuperNovice::new(),
            Gunslinger: Gunslinger::new(),
            Ninja: Ninja::new(),
            KagerouOboro: KagerouOboro::new(),
            Rebellion: Rebellion::new(),
            Summoner: Summoner::new(),
            StarEmperor: StarEmperor::new(),
            SoulReaper: SoulReaper::new(),
            Dummy: Dummy::new(),
        }
    }
    pub fn clear_preserve_capacity(&mut self) {
        self.Player.clear_preserve_capacity();
        self.MonsterNormal.clear_preserve_capacity();
        self.MonsterBoss.clear_preserve_capacity();
        self.Homunculus.clear_preserve_capacity();
        self.Mercenary.clear_preserve_capacity();
        self.Pet.clear_preserve_capacity();
        self.Elemental.clear_preserve_capacity();
        self.WugRider.clear_preserve_capacity();
        self.MadogearUser.clear_preserve_capacity();
        self.GeneticCart.clear_preserve_capacity();
        self.RuneKnight.clear_preserve_capacity();
        self.RoyalGuard.clear_preserve_capacity();
        self.Mechanic.clear_preserve_capacity();
        self.GuillotineCross.clear_preserve_capacity();
        self.ShadowChaser.clear_preserve_capacity();
        self.Sorcerer.clear_preserve_capacity();
        self.Warlock.clear_preserve_capacity();
        self.Archbishop.clear_preserve_capacity();
        self.Sura.clear_preserve_capacity();
        self.Ranger.clear_preserve_capacity();
        self.MinstrelWanderer.clear_preserve_capacity();
        self.SuperNovice.clear_preserve_capacity();
        self.Gunslinger.clear_preserve_capacity();
        self.Ninja.clear_preserve_capacity();
        self.KagerouOboro.clear_preserve_capacity();
        self.Rebellion.clear_preserve_capacity();
        self.Summoner.clear_preserve_capacity();
        self.StarEmperor.clear_preserve_capacity();
        self.SoulReaper.clear_preserve_capacity();
        self.Dummy.clear_preserve_capacity();
    }
    #[doc = r" Returns mutable references to an entity's components."]
    #[doc = r" **Safety**: The returned references may alias arbitrarily; the caller must ensure"]
    #[doc = r" exclusive access."]
    pub unsafe fn get_entity_mut(&self, handle: ecs::Handle) -> Option<ArchEntityRefs> {
        let arch_enum = unsafe { std::mem::transmute::<u8, ArchId>(handle.arch_id) };
        match arch_enum {
            ArchId::Player => {
                let arch_ref = &self.Player;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Player.storage.get() };
                let entity_refs = PlayerEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    Health: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Health)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScBerserk: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScBerserk)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScQuicken: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScQuicken)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScAngelus: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScAngelus)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScAdrenaline: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScAdrenaline)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScInspiration: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScInspiration)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Player(entity_refs))
            }
            ArchId::MonsterNormal => {
                let arch_ref = &self.MonsterNormal;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.MonsterNormal.storage.get() };
                let entity_refs = MonsterNormalEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    Health: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Health)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScPoison: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScPoison)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScBleeding: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScBleeding)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScCurse: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScCurse)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::MonsterNormal(entity_refs))
            }
            ArchId::MonsterBoss => {
                let arch_ref = &self.MonsterBoss;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.MonsterBoss.storage.get() };
                let entity_refs = MonsterBossEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    Health: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Health)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScStone: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScStone)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScFreeze: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScFreeze)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScStun: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScStun)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSleep: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSleep)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScDeepSleep: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScDeepSleep)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::MonsterBoss(entity_refs))
            }
            ArchId::Homunculus => {
                let arch_ref = &self.Homunculus;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Homunculus.storage.get() };
                let entity_refs = HomunculusEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSoulLink: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSoulLink)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScKaite: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScKaite)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScKyrie: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScKyrie)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Homunculus(entity_refs))
            }
            ArchId::Mercenary => {
                let arch_ref = &self.Mercenary;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Mercenary.storage.get() };
                let entity_refs = MercenaryEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    Health: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Health)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScMagnificat: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScMagnificat)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScGloria: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScGloria)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Mercenary(entity_refs))
            }
            ArchId::Pet => {
                let arch_ref = &self.Pet;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Pet.storage.get() };
                let entity_refs = PetEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScEndure: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScEndure)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScAutoguard: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScAutoguard)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Pet(entity_refs))
            }
            ArchId::Elemental => {
                let arch_ref = &self.Elemental;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Elemental.storage.get() };
                let entity_refs = ElementalEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScElementalChange: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScElementalChange)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScTidalWeapon: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScTidalWeapon)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScWaterScreen: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScWaterScreen)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Elemental(entity_refs))
            }
            ArchId::WugRider => {
                let arch_ref = &self.WugRider;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.WugRider.storage.get() };
                let entity_refs = WugRiderEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScWugRider: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScWugRider)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScCartBoost: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScCartBoost)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScIncreaseAgi: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScIncreaseAgi)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::WugRider(entity_refs))
            }
            ArchId::MadogearUser => {
                let arch_ref = &self.MadogearUser;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.MadogearUser.storage.get() };
                let entity_refs = MadogearUserEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScMadogear: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScMadogear)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScHeatBarrel: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScHeatBarrel)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScMagicalBullet: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScMagicalBullet)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::MadogearUser(entity_refs))
            }
            ArchId::GeneticCart => {
                let arch_ref = &self.GeneticCart;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.GeneticCart.storage.get() };
                let entity_refs = GeneticCartEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScCartBoost: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScCartBoost)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScPyrotechnic: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScPyrotechnic)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScThornTrap: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScThornTrap)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::GeneticCart(entity_refs))
            }
            ArchId::RuneKnight => {
                let arch_ref = &self.RuneKnight;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.RuneKnight.storage.get() };
                let entity_refs = RuneKnightEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScEnchantBlade: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScEnchantBlade)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScDeathBound: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScDeathBound)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScMillenniumShield: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScMillenniumShield)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScRebirth: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScRebirth)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::RuneKnight(entity_refs))
            }
            ArchId::RoyalGuard => {
                let arch_ref = &self.RoyalGuard;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.RoyalGuard.storage.get() };
                let entity_refs = RoyalGuardEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScDefender: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScDefender)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScReflectShield: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScReflectShield)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScPrestige: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScPrestige)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScBanding: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScBanding)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::RoyalGuard(entity_refs))
            }
            ArchId::Mechanic => {
                let arch_ref = &self.Mechanic;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Mechanic.storage.get() };
                let entity_refs = MechanicEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScMadogear: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScMadogear)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScPowerThrust: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScPowerThrust)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScWeaponPerfection: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScWeaponPerfection)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Mechanic(entity_refs))
            }
            ArchId::GuillotineCross => {
                let arch_ref = &self.GuillotineCross;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.GuillotineCross.storage.get() };
                let entity_refs = GuillotineCrossEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScCloakingExceed: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScCloakingExceed)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScRollingCutter: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScRollingCutter)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScCrossImpact: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScCrossImpact)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScDarkClaw: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScDarkClaw)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::GuillotineCross(entity_refs))
            }
            ArchId::ShadowChaser => {
                let arch_ref = &self.ShadowChaser;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.ShadowChaser.storage.get() };
                let entity_refs = ShadowChaserEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScReproduce: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScReproduce)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScAutoShadowSpell: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScAutoShadowSpell)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScShadowForm: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScShadowForm)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScFatalMenace: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScFatalMenace)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::ShadowChaser(entity_refs))
            }
            ArchId::Sorcerer => {
                let arch_ref = &self.Sorcerer;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Sorcerer.storage.get() };
                let entity_refs = SorcererEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSpellBreaker: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSpellBreaker)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSoulLink: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSoulLink)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScPreserve: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScPreserve)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Sorcerer(entity_refs))
            }
            ArchId::Warlock => {
                let arch_ref = &self.Warlock;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Warlock.storage.get() };
                let entity_refs = WarlockEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScReadingSpellBook: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScReadingSpellBook)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScFreezingSpell: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScFreezingSpell)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSummonBall: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSummonBall)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Warlock(entity_refs))
            }
            ArchId::Archbishop => {
                let arch_ref = &self.Archbishop;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Archbishop.storage.get() };
                let entity_refs = ArchbishopEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScAssumptio: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScAssumptio)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScKyrie: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScKyrie)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScMagnificat: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScMagnificat)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScBenedictum: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScBenedictum)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Archbishop(entity_refs))
            }
            ArchId::Sura => {
                let arch_ref = &self.Sura;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Sura.storage.get() };
                let entity_refs = SuraEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScGentleTouch: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScGentleTouch)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScPowerThrust: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScPowerThrust)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScRisingDragon: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScRisingDragon)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Sura(entity_refs))
            }
            ArchId::Ranger => {
                let arch_ref = &self.Ranger;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Ranger.storage.get() };
                let entity_refs = RangerEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScWargRider: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScWargRider)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScFearBreeze: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScFearBreeze)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScAimedBolt: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScAimedBolt)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Ranger(entity_refs))
            }
            ArchId::MinstrelWanderer => {
                let arch_ref = &self.MinstrelWanderer;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.MinstrelWanderer.storage.get() };
                let entity_refs = MinstrelWandererEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSongOfLutie: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSongOfLutie)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScDrumOnTheBattlefield: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScDrumOnTheBattlefield)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSaturdayNightFever: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSaturdayNightFever)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::MinstrelWanderer(entity_refs))
            }
            ArchId::SuperNovice => {
                let arch_ref = &self.SuperNovice;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.SuperNovice.storage.get() };
                let entity_refs = SuperNoviceEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScAngelus: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScAngelus)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScBlessing: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScBlessing)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScIncreaseAgi: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScIncreaseAgi)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScFoodStr: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScFoodStr)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScFoodInt: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScFoodInt)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScFoodLuk: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScFoodLuk)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::SuperNovice(entity_refs))
            }
            ArchId::Gunslinger => {
                let arch_ref = &self.Gunslinger;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Gunslinger.storage.get() };
                let entity_refs = GunslingerEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScMadnessCanceler: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScMadnessCanceler)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScLastStand: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScLastStand)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScGatlingFever: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScGatlingFever)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Gunslinger(entity_refs))
            }
            ArchId::Ninja => {
                let arch_ref = &self.Ninja;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Ninja.storage.get() };
                let entity_refs = NinjaEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScCloaking: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScCloaking)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScShadowLeap: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScShadowLeap)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScMirrorImage: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScMirrorImage)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Ninja(entity_refs))
            }
            ArchId::KagerouOboro => {
                let arch_ref = &self.KagerouOboro;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.KagerouOboro.storage.get() };
                let entity_refs = KagerouOboroEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScKunaiExplosion: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScKunaiExplosion)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSwirlingPetal: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSwirlingPetal)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScCrossSlash: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScCrossSlash)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::KagerouOboro(entity_refs))
            }
            ArchId::Rebellion => {
                let arch_ref = &self.Rebellion;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Rebellion.storage.get() };
                let entity_refs = RebellionEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScHeatBarrel: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScHeatBarrel)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScAntiMaterialBlast: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScAntiMaterialBlast)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScEternalChain: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScEternalChain)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Rebellion(entity_refs))
            }
            ArchId::Summoner => {
                let arch_ref = &self.Summoner;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Summoner.storage.get() };
                let entity_refs = SummonerEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScCatnipMeteor: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScCatnipMeteor)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScPickyPeck: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScPickyPeck)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScScarOfTarou: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScScarOfTarou)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Summoner(entity_refs))
            }
            ArchId::StarEmperor => {
                let arch_ref = &self.StarEmperor;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.StarEmperor.storage.get() };
                let entity_refs = StarEmperorEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSolarBurst: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSolarBurst)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScFullMoonKick: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScFullMoonKick)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScFallingStar: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScFallingStar)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::StarEmperor(entity_refs))
            }
            ArchId::SoulReaper => {
                let arch_ref = &self.SoulReaper;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.SoulReaper.storage.get() };
                let entity_refs = SoulReaperEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Velocity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Velocity)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSoulReap: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSoulReap)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScCurseOfSoul: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScCurseOfSoul)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSoulUnity: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSoulUnity)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::SoulReaper(entity_refs))
            }
            ArchId::Dummy => {
                let arch_ref = &self.Dummy;
                let slot_usize = handle.slot_index as usize;
                if slot_usize >= arch_ref.slots.len() {
                    return None;
                }
                let dense = arch_ref.slots[slot_usize];
                let dense_usize = dense as usize;
                if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                    return None;
                }
                let arch_mut_ref = unsafe { &mut *self.Dummy.storage.get() };
                let entity_refs = DummyEntityRefs {
                    Position: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Position)
                            .get_unchecked_mut(dense_usize)
                    },
                    Health: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, Health)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScStone: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScStone)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScFreeze: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScFreeze)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScStun: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScStun)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScSleep: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScSleep)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScBurning: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScBurning)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScPoison: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScPoison)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScBleeding: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScBleeding)
                            .get_unchecked_mut(dense_usize)
                    },
                    ScCurse: unsafe {
                        crate::access_component_field_mut!(arch_mut_ref, ScCurse)
                            .get_unchecked_mut(dense_usize)
                    },
                };
                Some(ArchEntityRefs::Dummy(entity_refs))
            }
        }
    }
}
#[doc = r" Iterate over entities that have *all* requested component types, with mutable access."]
#[doc = r" There is no way to have non-mutable access."]
#[macro_export]
macro_rules! query {
    ($world_expr:expr, | $($QArg:ident: &mut $QTy:ident),* | $body:block) => {
        {
            ecs::if_all_present!((Position,Velocity,Health,ScBerserk,ScQuicken,ScAngelus,ScAdrenaline,ScInspiration);
            $($QTy),* ;
            {
                let len =  $world_expr.Player.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Player.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,Health,ScPoison,ScBleeding,ScCurse);
            $($QTy),* ;
            {
                let len =  $world_expr.MonsterNormal.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.MonsterNormal.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,Health,ScStone,ScFreeze,ScStun,ScSleep,ScDeepSleep);
            $($QTy),* ;
            {
                let len =  $world_expr.MonsterBoss.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.MonsterBoss.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScSoulLink,ScKaite,ScKyrie);
            $($QTy),* ;
            {
                let len =  $world_expr.Homunculus.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Homunculus.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,Health,ScMagnificat,ScGloria);
            $($QTy),* ;
            {
                let len =  $world_expr.Mercenary.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Mercenary.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScEndure,ScAutoguard);
            $($QTy),* ;
            {
                let len =  $world_expr.Pet.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Pet.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScElementalChange,ScTidalWeapon,ScWaterScreen);
            $($QTy),* ;
            {
                let len =  $world_expr.Elemental.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Elemental.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScWugRider,ScCartBoost,ScIncreaseAgi);
            $($QTy),* ;
            {
                let len =  $world_expr.WugRider.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.WugRider.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScMadogear,ScHeatBarrel,ScMagicalBullet);
            $($QTy),* ;
            {
                let len =  $world_expr.MadogearUser.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.MadogearUser.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScCartBoost,ScPyrotechnic,ScThornTrap);
            $($QTy),* ;
            {
                let len =  $world_expr.GeneticCart.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.GeneticCart.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScEnchantBlade,ScDeathBound,ScMillenniumShield,ScRebirth);
            $($QTy),* ;
            {
                let len =  $world_expr.RuneKnight.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.RuneKnight.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScDefender,ScReflectShield,ScPrestige,ScBanding);
            $($QTy),* ;
            {
                let len =  $world_expr.RoyalGuard.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.RoyalGuard.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScMadogear,ScPowerThrust,ScWeaponPerfection);
            $($QTy),* ;
            {
                let len =  $world_expr.Mechanic.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Mechanic.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScCloakingExceed,ScRollingCutter,ScCrossImpact,ScDarkClaw);
            $($QTy),* ;
            {
                let len =  $world_expr.GuillotineCross.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.GuillotineCross.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScReproduce,ScAutoShadowSpell,ScShadowForm,ScFatalMenace);
            $($QTy),* ;
            {
                let len =  $world_expr.ShadowChaser.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.ShadowChaser.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScSpellBreaker,ScSoulLink,ScPreserve);
            $($QTy),* ;
            {
                let len =  $world_expr.Sorcerer.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Sorcerer.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScReadingSpellBook,ScFreezingSpell,ScSummonBall);
            $($QTy),* ;
            {
                let len =  $world_expr.Warlock.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Warlock.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScAssumptio,ScKyrie,ScMagnificat,ScBenedictum);
            $($QTy),* ;
            {
                let len =  $world_expr.Archbishop.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Archbishop.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScGentleTouch,ScPowerThrust,ScRisingDragon);
            $($QTy),* ;
            {
                let len =  $world_expr.Sura.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Sura.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScWargRider,ScFearBreeze,ScAimedBolt);
            $($QTy),* ;
            {
                let len =  $world_expr.Ranger.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Ranger.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScSongOfLutie,ScDrumOnTheBattlefield,ScSaturdayNightFever);
            $($QTy),* ;
            {
                let len =  $world_expr.MinstrelWanderer.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.MinstrelWanderer.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScAngelus,ScBlessing,ScIncreaseAgi,ScFoodStr,ScFoodInt,ScFoodLuk);
            $($QTy),* ;
            {
                let len =  $world_expr.SuperNovice.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.SuperNovice.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScMadnessCanceler,ScLastStand,ScGatlingFever);
            $($QTy),* ;
            {
                let len =  $world_expr.Gunslinger.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Gunslinger.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScCloaking,ScShadowLeap,ScMirrorImage);
            $($QTy),* ;
            {
                let len =  $world_expr.Ninja.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Ninja.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScKunaiExplosion,ScSwirlingPetal,ScCrossSlash);
            $($QTy),* ;
            {
                let len =  $world_expr.KagerouOboro.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.KagerouOboro.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScHeatBarrel,ScAntiMaterialBlast,ScEternalChain);
            $($QTy),* ;
            {
                let len =  $world_expr.Rebellion.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Rebellion.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScCatnipMeteor,ScPickyPeck,ScScarOfTarou);
            $($QTy),* ;
            {
                let len =  $world_expr.Summoner.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Summoner.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScSolarBurst,ScFullMoonKick,ScFallingStar);
            $($QTy),* ;
            {
                let len =  $world_expr.StarEmperor.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.StarEmperor.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Velocity,ScSoulReap,ScCurseOfSoul,ScSoulUnity);
            $($QTy),* ;
            {
                let len =  $world_expr.SoulReaper.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.SoulReaper.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
            ecs::if_all_present!((Position,Health,ScStone,ScFreeze,ScStun,ScSleep,ScBurning,ScPoison,ScBleeding,ScCurse);
            $($QTy),* ;
            {
                let len =  $world_expr.Dummy.dense_len();
                let arch_mut_ref = unsafe {
                    &mut *$world_expr.Dummy.storage.get()
                };
                $(let$QArg = unsafe {
                    crate::access_component_field_mut!(arch_mut_ref, $QTy)
                };
                )*for i in 0..len {
                    $(let$QArg = if cfg!(debug_assertions){
                        &mut $QArg[i]
                    }else {
                        unsafe {
                            $QArg.get_unchecked_mut(i)
                        }
                    };
                    )* $body
                }
            }{}
            );
        }
    };
}
#[macro_export]
macro_rules! extract_components_from_refs {
    ($refs_enum:expr,[$($EComp:ident),*]) => {
        {
            let result:Option<($(&mut $EComp),*)>  = match$refs_enum {
                ArchEntityRefs::Player(refs) => {
                    ecs::if_all_present!((Position,Velocity,Health,ScBerserk,ScQuicken,ScAngelus,ScAdrenaline,ScInspiration);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::MonsterNormal(refs) => {
                    ecs::if_all_present!((Position,Velocity,Health,ScPoison,ScBleeding,ScCurse);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::MonsterBoss(refs) => {
                    ecs::if_all_present!((Position,Velocity,Health,ScStone,ScFreeze,ScStun,ScSleep,ScDeepSleep);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Homunculus(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScSoulLink,ScKaite,ScKyrie);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Mercenary(refs) => {
                    ecs::if_all_present!((Position,Velocity,Health,ScMagnificat,ScGloria);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Pet(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScEndure,ScAutoguard);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Elemental(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScElementalChange,ScTidalWeapon,ScWaterScreen);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::WugRider(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScWugRider,ScCartBoost,ScIncreaseAgi);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::MadogearUser(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScMadogear,ScHeatBarrel,ScMagicalBullet);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::GeneticCart(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScCartBoost,ScPyrotechnic,ScThornTrap);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::RuneKnight(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScEnchantBlade,ScDeathBound,ScMillenniumShield,ScRebirth);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::RoyalGuard(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScDefender,ScReflectShield,ScPrestige,ScBanding);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Mechanic(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScMadogear,ScPowerThrust,ScWeaponPerfection);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::GuillotineCross(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScCloakingExceed,ScRollingCutter,ScCrossImpact,ScDarkClaw);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::ShadowChaser(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScReproduce,ScAutoShadowSpell,ScShadowForm,ScFatalMenace);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Sorcerer(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScSpellBreaker,ScSoulLink,ScPreserve);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Warlock(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScReadingSpellBook,ScFreezingSpell,ScSummonBall);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Archbishop(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScAssumptio,ScKyrie,ScMagnificat,ScBenedictum);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Sura(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScGentleTouch,ScPowerThrust,ScRisingDragon);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Ranger(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScWargRider,ScFearBreeze,ScAimedBolt);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::MinstrelWanderer(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScSongOfLutie,ScDrumOnTheBattlefield,ScSaturdayNightFever);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::SuperNovice(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScAngelus,ScBlessing,ScIncreaseAgi,ScFoodStr,ScFoodInt,ScFoodLuk);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Gunslinger(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScMadnessCanceler,ScLastStand,ScGatlingFever);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Ninja(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScCloaking,ScShadowLeap,ScMirrorImage);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::KagerouOboro(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScKunaiExplosion,ScSwirlingPetal,ScCrossSlash);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Rebellion(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScHeatBarrel,ScAntiMaterialBlast,ScEternalChain);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Summoner(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScCatnipMeteor,ScPickyPeck,ScScarOfTarou);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::StarEmperor(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScSolarBurst,ScFullMoonKick,ScFallingStar);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::SoulReaper(refs) => {
                    ecs::if_all_present!((Position,Velocity,ScSoulReap,ScCurseOfSoul,ScSoulUnity);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                },ArchEntityRefs::Dummy(refs) => {
                    ecs::if_all_present!((Position,Health,ScStone,ScFreeze,ScStun,ScSleep,ScBurning,ScPoison,ScBleeding,ScCurse);
                    $($EComp),* ;
                    {
                        unsafe {
                            Some(($(&mut *refs.$EComp),*))
                        }
                    }{
                        None
                    })
                }
            };
            result
        }
    };
}

// ──────────────────────────────────────────────────────────────
//  Stress test – spawn + 25 massive queries
// ──────────────────────────────────────────────────────────────
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
    query!(world, |p: &mut Position, v: &mut Velocity| {
        p.x += v.x;
    });

    query!(world, |p: &mut Position,
                   h: &mut Health,
                   b: &mut ScBerserk| {
        p.y += 1.0;
    });

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   b: &mut ScBerserk,
                   q: &mut ScQuicken,
                   a: &mut ScAngelus| {
        p.x += v.x * 0.1;
    });

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   m: &mut ScMadogear,
                   hb: &mut ScHeatBarrel,
                   mb: &mut ScMagicalBullet| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   s: &mut ScStone,
                   f: &mut ScFreeze,
                   st: &mut ScStun| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   c: &mut ScCloakingExceed,
                   r: &mut ScRollingCutter,
                   ci: &mut ScCrossImpact,
                   dc: &mut ScDarkClaw| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   sp: &mut ScSpellBreaker,
                   sl: &mut ScSoulLink,
                   pr: &mut ScPreserve| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   fs: &mut ScFoodStr,
                   fa: &mut ScFoodAgi,
                   fv: &mut ScFoodVit,
                   fi: &mut ScFoodInt| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   cart: &mut ScCartBoost,
                   wind: &mut ScWindWalk,
                   agi: &mut ScIncreaseAgi| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   end: &mut ScEndure,
                   ag: &mut ScAutoguard,
                   rs: &mut ScReflectShield,
                   pr: &mut ScPrestige| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   strip: &mut ScStripAccessory,
                   inv: &mut ScInvisibility| {});

    query!(
        world,
        |p: &mut Position,
         v: &mut Velocity,
         h: &mut Health,
         song: &mut ScSongOfLutie,
         drum: &mut ScDrumOnTheBattlefield| {}
    );

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   cat: &mut ScCatnipMeteor,

                   peck: &mut ScPickyPeck| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   sb: &mut ScSolarBurst,
                   fmk: &mut ScFullMoonKick| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   reap: &mut ScSoulReap,
                   curse: &mut ScCurseOfSoul| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   cpw: &mut ScCpWeapon,
                   cpa: &mut ScCpArmor,
                   cps: &mut ScCpShield,
                   cph: &mut ScCpHelm| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   paint: &mut ScPalletPaint,
                   pyro: &mut ScPyrotechnic| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   thorn: &mut ScThornTrap,
                   spore: &mut ScSporeExplosion| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   ma: &mut ScManuAtk,
                   md: &mut ScManuDef| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   sa: &mut ScSplAtk,
                   sd: &mut ScSplDef| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   hb: &mut ScHeatBarrel,
                   amb: &mut ScAntiMaterialBlast| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   edp: &mut ScEdp,
                   maxp: &mut ScMaximizePower| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   rb: &mut ScReboundShield,
                   hb2: &mut ScHeatBarrel| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   h: &mut Health,
                   tarot: &mut ScTarotCard,
                   ref_: &mut ScRefresh,
                   lux: &mut ScLuxAnima,
                   her: &mut ScHermode| {});

    query!(world, |p: &mut Position,
                   v: &mut Velocity,
                   stone: &mut ScStone,
                   f: &mut ScFreeze,
                   s: &mut ScStun,
                   sleep: &mut ScSleep,
                   burn: &mut ScBurning,
                   poi: &mut ScPoison| {});
}
