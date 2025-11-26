#![allow(unused)]
#![feature(macro_metavar_expr)]

use std::collections::VecDeque;

/// A compact handle that identifies an entity across the World.
///
/// Layout: [LSB..MSB), bit indices
/// - 0..32  : slot_index (u32)
/// - 32..48 : generation  (u16)
/// - 48..56 : arch_id     (u8)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Handle(u64);

impl Handle {
    /// Pack components into a `Handle`.
    #[inline]
    pub fn new(arch_id: u8, slot_index: u32, generation: u16) -> Self {
        let slot_part = slot_index as u64;
        let gen_part = (generation as u64) << 32;
        let arch_part = (arch_id as u64) << 48;
        Handle(slot_part | gen_part | arch_part)
    }

    #[inline]
    pub fn empty() -> Self {
        Handle(0)
    }

    #[inline]
    pub fn arch_id(&self) -> u8 {
        ((self.0 >> 48) & 0xFF) as u8
    }

    #[inline]
    pub fn slot_index(&self) -> u32 {
        (self.0 & 0xFFFF_FFFF) as u32
    }

    #[inline]
    pub fn generation(&self) -> u16 {
        ((self.0 >> 32) & 0xFFFF) as u16
    }
}

// ---------------------------
// Macro utilities
// ---------------------------

/// Compare two identifiers at macro-expansion time and choose branches accordingly.
#[macro_export]
macro_rules! token_match {
    ($A:ident, $B:ident, $Then:tt, $Else:tt) => {{
        // kinda like match
        macro_rules! __eq_ident_helper {
            ($A $A) => { $Then };
            ($A $B) => { $Else };
        }
        __eq_ident_helper!($A $B)
    }};
}

// Helper macro used inside generated macros to test whether an archetype's component-list
// contains all query component idents. It's an n-array matcher: we check the head then recurse.
// It is safe for compile-time expansion and is purely macro-time logic ("zero cost abstraction").
#[macro_export]
macro_rules! if_has_type {
    // multiple tail components
    ($Tgt:ident; $Head:ident, $($Tail:ident),*; $Body:tt) => {
        $crate::token_match!($Tgt, $Head, $Body, {
            $crate::if_has_type!($Tgt; $($Tail),*; $Body)
        })
    };
    // final single element
    ($Tgt:ident; $Head:ident; $Body:tt) => {
        $crate::token_match!($Tgt, $Head, $Body, {})
    };
    // empty archetype component list -> nothing matches
    ($Tgt:ident; ; $Body:tt) => {};
}

// Checks whether an archetype (component list $AC) contains all components in query ($QC).
#[macro_export]
macro_rules! if_arch_matches {
    // query empty => success
    ( ($($AC:ident),*); ; $Body:tt ) => { $Body };

    // more than one query component
    ( ($($AC:ident),*); $QH:ident, $($QT:ident),*; $Body:tt ) => {
        $crate::if_has_type!($QH; $($AC),*; {
            $crate::if_arch_matches!( ($($AC),*); $($QT),*; $Body )
        })
    };

    // single query component
    ( ($($AC:ident),*); $QH:ident; $Body:tt ) => {
        $crate::if_has_type!($QH; $($AC),*; {
            $Body
        })
    };
}

// ---------------------------
// The big ahh generator macro
// ---------------------------
//
// `declare_ecs!` generates:
//  - component types are assumed to be declared by the user (we do not generate component types).
//  - for each archetype: a struct with fields for each component Vec<T>, slots vector, dense_to_slot,
//    free_slots, len, and spawn/despawn logic.
//  - an ArchId enum and conversions.
//  - a World struct with named archetype fields and a small `arch_by_id_mut()` helper (match-based O(1)).
//  - a `query!` macro that can be called like:
//        query!(world, |p: &mut Position, v: &mut Velocity| { ... })
#[macro_export]
macro_rules! declare_ecs {
    (
        world: $WorldName:ident,
        archetypes: {
            $( $ArchName:ident : ( $( $Comp:ident ),* ) ),* $(,)?
        }
    ) => {
        use std::collections::VecDeque;

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum ArchId {
            $(
                $ArchName,
            )*
        }

        impl ArchId {
            #[inline]
            pub fn as_u8(self) -> u8 { self as u8 }
        }

        // ${count($t)} is now in Rust!
        pub const ARCH_COUNT: usize = ${count($ArchName)};

        // -------------------------
        // 3) Archetype structs + impls
        // For each archetype we generate a struct with:
        //  - component storage: Vec<ComponentType>
        //  - slots -> dense mapping (Vec<u32>)
        //  - dense_to_slot (Vec<u32>)
        //  - free_slots (VecDeque<u32>)
        //  - slot_generations (Vec<u16>) -- used to check stale handles; only meaningful with debug asserts
        //  - len: active count
        // -------------------------
        $(
            #[allow(non_snake_case)]
            pub struct $ArchName {
                $( pub $Comp: Vec<$Comp>, )*
                // mapping SlotIndex -> DenseIndex
                slots: Vec<u32>,
                // mapping DenseIndex -> SlotIndex
                dense_to_slot: Vec<u32>,
                // recycled slot free list
                free_slots: VecDeque<u32>,
                // generation counter per slot (u16). We increment on reuse to invalidate stale handles.
                slot_generations: Vec<u16>,
                // number of active entities (dense length)
                pub len: usize,
            }

            impl $ArchName {
                pub fn new() -> Self {
                    Self {
                        $( $Comp: Vec::new(), )*
                        slots: Vec::new(),
                        dense_to_slot: Vec::new(),
                        free_slots: VecDeque::new(),
                        slot_generations: Vec::new(),
                        len: 0,
                    }
                }

                /// Spawn an entity into this archetype.
                /// Returns a tuple (slot_index, generation).
                /// Reason we dont have generic spawn macro is all archetypes are static (by design)
                /// You would simply not be able to select a new archetype.
                /// It is just easier to give them names and refer by names.
                /// TODO: auto names, proper naming conventions
                pub fn spawn(&mut self, $( $Comp: $Comp ),* ) -> (u32, u16) {
                    // 1) allocate or reuse a slot index (u32)
                    let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
                        idx
                    } else {
                        let idx = self.slots.len() as u32;
                        self.slots.push(u32::MAX); // placeholder mapped to dense index
                        self.slot_generations.push(0u16);
                        idx
                    };

                    // dense index that this entity will occupy
                    let dense_index = self.len as u32;

                    // 2) update mapping Slot->Dense
                    self.slots[slot_index as usize] = dense_index;

                    // 3) update dense_to_slot
                    if (dense_index as usize) < self.dense_to_slot.len() {
                        self.dense_to_slot[dense_index as usize] = slot_index;
                    } else {
                        self.dense_to_slot.push(slot_index);
                    }

                    // 4) push component data
                    $( self.$Comp.push($Comp); )*

                    // 5) finalize
                    self.len += 1;
                    let generation = self.slot_generations[slot_index as usize];
                    (slot_index, generation)
                }

                /// Despawn an entity given slot_index. Returns true if successful.
                /// This performs swap-remove on component arrays and updates mappings,
                /// which means there never a hole in entities and executing queries is always max perfomance in that regard.
                /// Notice how it does not take archetype - it is figured out from slot index.
                pub fn despawn(&mut self, slot_index: u32) -> bool {
                    let slot_index_usize = slot_index as usize;
                    if slot_index_usize >= self.slots.len() { return false; }
                    let dense_index = self.slots[slot_index_usize];

                    // invalid slot sentinel or out-of-range dense index
                    if dense_index == u32::MAX || (dense_index as usize) >= self.len { return false; }

                    let last_dense_index = (self.len - 1) as u32;

                    // 1) swap-remove component arrays
                    $( self.$Comp.swap_remove(dense_index as usize); )*

                    // 2) fix up mappings if we moved an entity into dense_index
                    if dense_index != last_dense_index {
                        let swapped_slot = self.dense_to_slot[last_dense_index as usize];
                        self.slots[swapped_slot as usize] = dense_index;
                        self.dense_to_slot[dense_index as usize] = swapped_slot;
                    }

                    // 3) reduce length and recycle slot
                    self.len -= 1;
                    self.free_slots.push_back(slot_index);

                    // increment generation so old handles are invalidated (debug-time check)
                    let generation = &mut self.slot_generations[slot_index_usize];
                    *generation = generation.wrapping_add(1);

                    // mark slot as empty
                    self.slots[slot_index_usize] = u32::MAX;
                    true
                }

                /// Internal helper: for query expansion we will need slices for component arrays.
                /// This returns the length (dense len) and a tuple of `&mut [T]` for each component.
                #[inline]
                pub fn dense_len(&self) -> usize { self.len }

                pub fn clear_preserve_capacity(&mut self) {
                    for generation in &mut self.slot_generations {
                        // invalidate old handles
                        *generation = generation.wrapping_add(1);
                    }

                    // reset mapping arrays to empty
                    for slot in &mut self.slots {
                        *slot = u32::MAX;
                    }
                    self.dense_to_slot.clear(); // no dense entries
                    self.free_slots.clear();

                    // drop elements but keep capacity
                    $(
                        // look into archetype macro generated
                        self.$Comp.clear();
                    )*

                    self.len = 0;
                }
            }
        )*

        // -------------------------
        // 4) World struct (container for all archetypes)
        // -------------------------
        pub struct $WorldName {
            $( pub $ArchName: $ArchName, )*
        }

        impl $WorldName {
            pub fn new() -> Self {
                Self {
                    $( $ArchName: $ArchName::new(), )*
                }
            }

            pub fn clear_preserve_capacity(&mut self) {
                $(
                    self.$ArchName.clear_preserve_capacity();
                )*
            }

            /// Lookup a mutable reference to an archetype by `ArchId`.
            ///
            /// Implemented as a `match` so it's an O(1) index (compiler optimizes).
            pub fn arch_by_id_mut(&mut self, id: ArchId) -> ArchRefMut<'_> {
                match id {
                    $(
                        ArchId::$ArchName => ArchRefMut::$ArchName(&mut self.$ArchName),
                    )*
                }
            }

            /// Get components for a raw handle. This performs generation check in debug builds.
            ///
            /// Returns `None` if the handle is stale/invalid or the slot doesn't exist in the arch.
            pub fn get_slot_dense_index_and_check(&mut self, handle: Handle) -> Option<(ArchId, u32)> {
                let arch = handle.arch_id();
                let slot_index = handle.slot_index();
                // convert arch u8 to ArchId
                let arch_enum = unsafe { std::mem::transmute::<u8, ArchId>(arch) };
                match arch_enum {
                    $(
                        ArchId::$ArchName => {
                            // bounds check
                            let arch_ref = &mut self.$ArchName;
                            let slot_usize = slot_index as usize;
                            if slot_usize >= arch_ref.slots.len() {
                                return None;
                            }
                            let dense = arch_ref.slots[slot_usize];
                            if dense == u32::MAX || (dense as usize) >= arch_ref.len {
                                return None;
                            }
                            // debug-only generation check
                            #[cfg(debug_assertions)]
                            {
                                let generation = arch_ref.slot_generations[slot_usize];
                                if generation != handle.generation() {
                                    return None;
                                }
                            }
                            Some((ArchId::$ArchName, dense))
                        }
                    )*
                }
            }
        }

        /// Enum used to return a concrete mutable reference to any generated archetype.
        /// Each variant is named after the archetype and holds `&'a mut ArchetypeType`.
        pub enum ArchRefMut<'a> {
            $(
                $ArchName(&'a mut $ArchName),
            )*
        }

        // -------------------------
        // 5) Query macro
        //    Usage:
        //      query!(world_expr, | $($QArg : &mut $QTy),* | { /* body */ })
        //
        //  Explanation:
        //  - The macro expands to: for each declared archetype, if that archetype contains ALL
        //    requested component types, execute the loop that iterates its dense array and binds
        //    the requested component mutable references to the user-supplied `$QArg` names.
        //  - We purposely emit one loop per archetype (compile-time filtered) to avoid runtime matching.
        //  - Borrowing: since all `$QArg` references come from the same archetype struct fields,
        //    creating `&mut` slices and then indexing them by `i` is safe. The macro contains the
        //    `unsafe` block to manage the temporary borrowing of `world_expr` only once.
        // -------------------------
        #[macro_export]
        macro_rules! query {
            // Pattern: query!( world_expr, | arg: &mut Type, ... | { body } )
            ( $world_expr:expr, | $$( $$QArg:tt : &mut $$QTy:tt ),* | $body:block ) => {
                {
                    // single borrow of the provided world expression
                    let world_borrow = &mut $world_expr;

                    $(
                        // Emit this archetype's loop only if it contains ALL requested component types.
                        $crate::if_arch_matches!( ($($Comp),*); $$($$QTy),*; {
                            let len = world_borrow.$ArchName.dense_len();

                            // Obtain mutable slices to each requested component vector.
                            $$(
                                let $$QArg = world_borrow.$ArchName.$$QTy.as_mut_slice();
                            )*

                            // Index-based loop so we can create multiple &mut references safely.
                            for i in 0..len {
                                $$(
                                    let $$QArg = &mut $$QArg[i];
                                )*
                                $body
                            }
                        });
                    )*
                }
            };
        }
    };
}
