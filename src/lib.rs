#![allow(unused)]
#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]

pub use sibling_vecs;

/// Compact handle that identifies an entity across the World.
///
/// Layout: [LSB..MSB), bit indices
/// - 0..32  : slot_index  (u32)
/// - 32..48 : generation  (u16)
/// - 48..56 : arch_id     (u8)
/// - 56..64 : reserved for your fav anime waifu name hash
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Handle {
    pub slot_index: u32,
    pub generation: u16,
    pub arch_id: u8,
    // pub your_fav_waifu_name_hash: u8,
}

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
// contains all query component idents.
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

#[macro_export]
macro_rules! invoke_with_concat {
    ($ArchName:ident, { $($acc:tt)* }, $Comp:ident, { $($tail:tt)* }, $field_name:ident) => {
        $crate::generate_storage_recursive!(
            $ArchName,
            { $($acc)* $field_name : $Comp, },
            { $($tail)* }
        );
    }
}

#[macro_export]
macro_rules! generate_storage_recursive {
    ( $ArchName:ident, { $( $field:ident : $type:ident, )* }, {} ) => {
         sibling_vecs::sibling_vecs! {
            // Note: ${concat} works here because it is not inside a repetition
            pub struct ${concat($ArchName, ComponentStorage)} {
                $( $field : $type, )*
            }
         }
    };

    ( $ArchName:ident, { $( $acc:tt )* }, { $head:ident, $($tail:ident),* } ) => {
         $crate::invoke_with_concat!(
            $ArchName,
            { $($acc)* },
            $head,
            { $($tail),* },
            // ${concat(component, $head)}
            $head
         );
    };

    ( $ArchName:ident, { $( $acc:tt )* }, { $head:ident } ) => {
         $crate::invoke_with_concat!(
            $ArchName,
            { $($acc)* },
            $head,
            {},
            // ${concat(component, $head)}
            $head
         );
    };
}

#[macro_export]
macro_rules! concatenate_component_storage {
    ( $a:ident ) => {${concat($a, ComponentStorage)}};
}

#[macro_export]
macro_rules! access_component_field {
    ($storage:expr, $Comp:ident) => {
        $storage.$Comp()
    };
}
#[macro_export]
macro_rules! access_component_field_mut {
    ($storage:expr, $Comp:ident) => {
        $storage.${concat($Comp, _mut)}()
    };
}

// The big ahh generator macro
#[macro_export]
macro_rules! declare_ecs {
    (
        world: $WorldName:ident,
        archetypes: {
            $( $ArchName:ident : ( $( $Comp:ident ),* ) ),* $(,)?
        }
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum ArchId {
            $(
                $ArchName,
            )*
        }

        impl ArchId {
            pub fn as_u8(self) -> u8 { self as u8 }
        }

        // ${count($t)} is now in Rust!
        pub const ARCH_COUNT: usize = ${count($ArchName)};

        $(
            // TODO:
            // nested concat is not supported yet
            $crate::generate_storage_recursive!( $ArchName, {}, { $($Comp),* } );
            // could be just this:
            // sibling_vecs::sibling_vecs! {
            //     pub struct ${concat($ArchName, ComponentStorage)} {
            //         $( ${concat(component, $Comp)} : $Comp, )*
            //     }
            // }

            #[allow(non_snake_case)]
            pub struct $ArchName {
                // Component storage: Vec<ComponentType> (uses Vec for efficient push/swap_remove)
                // $( pub $Comp: Vec<$Comp>, )*
                storage: ${concat($ArchName, ComponentStorage)},
                // mapping SlotIndex -> DenseIndex
                slots: Vec<u32>,
                // mapping DenseIndex -> SlotIndex
                dense_to_slot: Vec<u32>,
                // recycled slot free list
                free_slots: ::std::collections::VecDeque<u32>,
                // generation counter per slot (u16). Only used in debug builds.
                #[cfg(debug_assertions)]
                slot_generations: Vec<u16>,
                // number of active entities (dense length)
                pub len: usize,
            }

            impl $ArchName {
                pub fn new() -> Self {
                    Self {
                        // $( $Comp: Vec::new(), )*
                        storage: ${concat($ArchName, ComponentStorage)}::new(),
                        slots: Vec::new(),
                        dense_to_slot: Vec::new(),
                        free_slots: ::std::collections::VecDeque::new(),
                        #[cfg(debug_assertions)]
                        slot_generations: Vec::new(),
                        len: 0,
                    }
                }

                /// Spawn an entity into this archetype.
                /// Returns a tuple (slot_index, generation).
                pub fn spawn(&mut self, $( $Comp: $Comp ),* ) -> $crate::Handle {
                    let slot_index: u32 = if let Some(idx) = self.free_slots.pop_front() {
                        idx
                    } else {
                        let idx = self.slots.len() as u32;
                        self.slots.push(u32::MAX);
                        #[cfg(debug_assertions)]
                        self.slot_generations.push(0u16);
                        idx
                    };

                    let dense_index = self.len as u32;
                    let dense_index_usize = self.len;

                    self.slots[slot_index as usize] = dense_index;

                    if dense_index_usize < self.dense_to_slot.len() {
                        self.dense_to_slot[dense_index_usize] = slot_index;
                    } else {
                        self.dense_to_slot.push(slot_index);
                    }

                    // $( self.$Comp.push($Comp); )*
                    self.storage.push (
                        $($Comp,)*
                    );

                    self.len += 1;

                    #[cfg(debug_assertions)]
                    let generation = self.slot_generations[slot_index as usize];
                    #[cfg(not(debug_assertions))]
                    let generation = 0u16;

                    let arch_id = ArchId::$ArchName.as_u8();

                    $crate::Handle {
                        arch_id,
                        slot_index,
                        generation,
                    }
                }

                /// Despawn an entity given slot_index. Returns true if successful.
                pub fn despawn(&mut self, handle: $crate::Handle) {
                    let slot_index_usize = handle.slot_index as usize;
                    debug_assert!(slot_index_usize < self.slots.len());

                    let dense_index = self.slots[slot_index_usize];
                    let dense_index_usize = dense_index as usize;

                    debug_assert!(dense_index != u32::MAX);
                    debug_assert!(dense_index_usize < self.len);

                    let last_dense_index = (self.len - 1) as u32;
                    let last_dense_index_usize = self.len - 1;

                    // swap-remove components in component arrays
                    // $( self.$Comp.swap_remove(dense_index_usize); )*
                    self.storage.swap_remove(dense_index_usize);


                    // fix up mappings if we moved an entity into dense_index
                    let was_despawned_last: bool = dense_index_usize == last_dense_index_usize;
                    if !was_despawned_last {
                        let swapped_slot = self.dense_to_slot[last_dense_index_usize];
                        // moved entity now resides at dense_index
                        self.slots[swapped_slot as usize] = dense_index;
                        self.dense_to_slot[dense_index_usize] = swapped_slot;
                    }
                    // since component vectors have already been physically shrunk by 1 due to `swap_remove`,
                    // we don't need to do anything extra

                    // reduce length and recycle slot
                    self.len -= 1;
                    self.free_slots.push_back(handle.slot_index);

                    #[cfg(debug_assertions)]
                    {
                        // increment generation
                        let generation = &mut self.slot_generations[slot_index_usize];
                        debug_assert!(handle.generation == *generation);
                        *generation = generation.wrapping_add(1);
                    }

                    // mark slot as empty
                    self.slots[slot_index_usize] = u32::MAX;
                }

                pub fn dense_len(&self) -> usize { self.len }

                pub fn clear_preserve_capacity(&mut self) {
                    #[cfg(debug_assertions)]
                    for generation in &mut self.slot_generations {
                        // invalidate old handles
                        *generation = generation.wrapping_add(1);
                    }

                    // reset mapping arrays to empty
                    for slot in &mut self.slots {
                        *slot = u32::MAX;
                    }
                    self.dense_to_slot.clear();
                    self.free_slots.clear();

                    // $(
                    //     self.$Comp.clear();
                    // )*
                    self.storage.clear();

                    self.len = 0;
                }

                $(
                    pub fn $Comp(&self) -> &[$Comp] {
                        self.storage.$Comp()
                    }
                )*

                // $(
                //     pub fn ${concat($Comp, _mut)}(&self) -> &mut [$Comp] {
                //         self.storage.$Comp()
                //     }
                // )*
            }
        )*

        // Main world struct (container for all archetypes)
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

            pub fn arch_by_id_mut(&mut self, id: ArchId) -> ArchRefMut<'_> {
                match id {
                    $(
                        ArchId::$ArchName => ArchRefMut::$ArchName(&mut self.$ArchName),
                    )*
                }
            }

            pub fn get_slot_dense_index_and_check(&mut self, handle: $crate::Handle) -> Option<(ArchId, u32)> {
                let arch = handle.arch_id;
                let slot_index = handle.slot_index;
                let arch_enum = unsafe { ::std::mem::transmute::<u8, ArchId>(arch) };
                match arch_enum {
                    $(
                        ArchId::$ArchName => {
                            let arch_ref = &mut self.$ArchName;
                            let slot_usize = slot_index as usize;
                            if slot_usize >= arch_ref.slots.len() {
                                return None;
                            }
                            let dense = arch_ref.slots[slot_usize];
                            let dense_usize = dense as usize;
                            if dense == u32::MAX || dense_usize >= arch_ref.len {
                                return None;
                            }
                            #[cfg(debug_assertions)]
                            {
                                let generation = arch_ref.slot_generations[slot_usize];
                                if generation != handle.generation {
                                    return None;
                                }
                            }
                            Some((ArchId::$ArchName, dense))
                        }
                    )*
                }
            }
        }

        pub enum ArchRefMut<'a> {
            $(
                $ArchName(&'a mut $ArchName),
            )*
        }

        // Main "iter through components macro"
        // Note how it is expanding macro, generatated in macro
        // to pull this off we sometimes need to use $$ instead of $
        #[macro_export]
        macro_rules! query {
            // Pattern: query!( world_expr, | arg: &mut Type, ... | { lambda body } )
            ( $world_expr:expr, | $$( $$QArg:tt : &mut $$QTy:tt ),* | $body:block ) => {
                {
                    let world_mut_ref = &mut $world_expr;

                    $(
                        // emit this archetype's loop only if it contains ALL requested component types
                        // double dollar because thats how nested macros work
                        $crate::if_arch_matches!( ($($Comp),*); $$($$QTy),*; {
                            let len = world_mut_ref.$ArchName.dense_len();

                            // obtain mutable slices to each requested component vector
                            $$(
                                let $$QArg = unsafe {
                                    crate::access_component_field_mut!(world_mut_ref.$ArchName.storage, $$QTy)
                                    };
                            )*

                            // iter through entities in this archetype, load necessary components and execute lambda
                            for i in 0..len {
                                $$(
                                    // bounds checking in debug
                                    let $$QArg = if cfg!(debug_assertions) {
                                        &mut $$QArg[i]
                                    } else {
                                        unsafe { $$QArg.get_unchecked_mut(i) }
                                    };
                                )*
                                // so weird to do this to lambdas lol
                                $body
                            }
                        });
                    )*
                }
            };
        }
    };
}
