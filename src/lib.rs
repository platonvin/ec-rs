#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(decl_macro)]

/// Compact 8-byte handle that identifies an entity across the World.
/// expected layout: [LSB..MSB)
/// - 0..32  : slot_index  (u32)
/// - 32..48 : generation  (u16)
/// - 48..56 : arch_id     (u8)
/// - 56..64 : reserved for your fav cat image hash
///
/// In reality we dont really specify layout and leave it up to the compiler.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Handle {
    // we could do some manual bits. But this works too, why not. Could also move fields around and see what happens
    pub slot_index: u32,
    pub generation: u16,
    pub arch_id: u8,
    // pub your_fav_cat_img_hash: u8,
}
const _: () = assert!(std::mem::size_of::<Handle>() <= 8);

#[macro_export]
/// if A same as B {then} {else}
macro_rules! if_ident_eq {
    ($A:ident, $B:ident, $Then:tt, $Else:tt) => {{
        macro_rules! __helper {
            ($A $A) => { $Then };
            ($A $B) => { $Else };
        }
        __helper!($A $B)
    }};
}

/// if needle in haystack {then} {else}
#[macro_export]
macro_rules! if_has_type {
    ($needle:ident; $hay_head:ident $(, $hay_tail:ident)* ; $Then:tt $Else:tt) => {
        $crate::if_ident_eq!($needle, $hay_head, $Then, {
            $crate::if_has_type!($needle; $($hay_tail),* ; $Then $Else)
        })
    };
    ($needle:ident; ; $Then:tt $Else:tt) => { $Else };
}
/// if have's in haystack {then} {else}
#[macro_export]
macro_rules! if_all_present {
    ( ($($haves:ident),*) ; $want_head:ident $(, $want_tail:ident)* ; $Then:tt $Else:tt ) => {
        $crate::if_has_type!($want_head; $($haves),* ; {
            $crate::if_all_present!( ($($haves),*) ; $($want_tail),* ; $Then $Else )
        } $Else )
    };
    ( ($($haves:ident),*) ; ; $Then:tt $Else:tt ) => { $Then };
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
// TODO: remove when nested concat's are a thing
macro_rules! generate_storage_recursive {
    ( $ArchName:ident, { $( $field:ident : $type:ident, )* }, {} ) => {
        #[allow(nonstandard_style)]
        sibling_vecs::sibling_vecs! {
            // Note: ${concat} works here because it is not inside a repetition
            // TODO: make concat work?
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
            $head
         );
    };

    ( $ArchName:ident, { $( $acc:tt )* }, { $head:ident } ) => {
         $crate::invoke_with_concat!(
            $ArchName,
            { $($acc)* },
            $head,
            {},
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
                #[allow(nonstandard_style)]
                $ArchName,
            )*
        }

        impl ArchId {
            #[inline]
            pub fn as_u8(self) -> u8 { self as u8 }
        }

        // ${count($t)} is now in Rust!
        pub const ARCH_COUNT: usize = ${count($ArchName)};

        #[allow(nonstandard_style)]
        pub enum ArchEntityRefs {
            $(
                #[allow(nonstandard_style)]
                $ArchName(${concat($ArchName, EntityRefs)}),
            )*
        }

        $(
            // TODO:
            // nested concat is not supported yet
            #[allow(nonstandard_style)]
            $crate::generate_storage_recursive!( $ArchName, {}, { $($Comp),* } );

            // could be just this:
            // sibling_vecs::sibling_vecs! {
            //     pub struct ${concat($ArchName, ComponentStorage)} {
            //         $( ${concat(component, $Comp)} : $Comp, )*
            //     }
            // }

            #[allow(nonstandard_style)]
            pub struct ${concat($ArchName, EntityRefs)} {
                // we kinda want to store &mut, but its easier with pointers
                #[allow(nonstandard_style)]
                $( pub $Comp: *mut $Comp, )*
            }

            #[allow(nonstandard_style)]
            pub struct $ArchName {
                // Component storage: Vec<ComponentType> (uses Vec for efficient push/swap_remove)
                storage: std::cell::UnsafeCell<${concat($ArchName, ComponentStorage)}>,
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
            }

            #[allow(nonstandard_style)]
            impl $ArchName {
                pub fn new() -> Self {
                    Self {
                        storage: std::cell::UnsafeCell::new(${concat($ArchName, ComponentStorage)}::new()),
                        slots: Vec::new(),
                        dense_to_slot: Vec::new(),
                        free_slots: ::std::collections::VecDeque::new(),
                        #[cfg(debug_assertions)]
                        slot_generations: Vec::new(),
                    }
                }

                /// Spawn an entity into this archetype.
                /// Returns a tuple (slot_index, generation).
                #[allow(nonstandard_style)]
                pub fn spawn(&mut self, $( $Comp: $Comp ),* ) -> $crate::Handle {
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

                    storage.push (
                        $($Comp,)*
                    );

                    #[cfg(debug_assertions)]
                    let generation = self.slot_generations[slot_index as usize];
                    #[cfg(not(debug_assertions))]
                    let generation = 0;

                    let arch_id = ArchId::$ArchName.as_u8();

                    $crate::Handle {
                        arch_id,
                        slot_index,
                        generation,
                    }
                }

                /// Despawn an entity given slot_index. Returns true if successful.
                pub fn despawn(&mut self, handle: $crate::Handle) {
                    let storage = unsafe { &mut *self.storage.get() };

                    let slot_index_usize = handle.slot_index as usize;
                    debug_assert!(slot_index_usize < self.slots.len());

                    let dense_index = self.slots[slot_index_usize];
                    let dense_index_usize = dense_index as usize;

                    debug_assert_ne!(dense_index, u32::MAX);
                    debug_assert!(dense_index_usize < storage.len);

                    let last_dense_index = (storage.len - 1) as u32;
                    let last_dense_index_usize = storage.len - 1;

                    storage.swap_remove(dense_index_usize);

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
                    // self.storage.len -= 1; // already reduced by swap_remove
                    self.free_slots.push_back(handle.slot_index);

                    #[cfg(debug_assertions)]
                    {
                        // increment generation
                        let generation = &mut self.slot_generations[slot_index_usize];
                        debug_assert_eq!(handle.generation, *generation);
                        *generation = generation.wrapping_add(1);
                    }

                    // mark slot as empty
                    self.slots[slot_index_usize] = u32::MAX;
                }

                pub fn dense_len(&self) -> usize { unsafe { (*self.storage.get()).len } }

                pub fn clear_preserve_capacity(&mut self) {
                    let storage = unsafe { &mut *self.storage.get() };

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

                    storage.clear();
                }

                $(
                    #[inline]
                    #[allow(nonstandard_style)]
                    pub fn $Comp(&self) -> &[$Comp] {
                        unsafe { (*self.storage.get()).$Comp() }
                    }
                )*

                // TODO:
                // $(
                //     pub fn ${concat($Comp, _mut)}(&self) -> &mut [$Comp] {
                //         self.storage.$Comp()
                //     }
                // )*
            }
        )*

        // Main world struct (container for all archetypes)
        pub struct $WorldName {
            #[allow(nonstandard_style)]
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

            // WARNING: This function returns a struct containing mutable references (&'a mut T)
            // while only taking a shared reference (&self). This is only possible via
            // unsafe lifetime magic and requires the caller to manage aliasing manually.
            pub unsafe fn get_entity_mut(&self, handle: $crate::Handle) -> Option<ArchEntityRefs> {
                let arch = handle.arch_id;
                let slot_index = handle.slot_index;
                let arch_enum = unsafe { ::std::mem::transmute::<u8, ArchId>(arch) };

                match arch_enum {
                    $(
                        ArchId::$ArchName => {
                            let arch_ref = &self.$ArchName; // Note: &self, not &mut self
                            let slot_usize = slot_index as usize;

                            if slot_usize >= arch_ref.slots.len() { return None; }
                            let dense = arch_ref.slots[slot_usize];
                            let dense_usize = dense as usize;

                            if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } { return None; }

                            // todo: is there ub on such cast?
                            let arch_mut_ref = unsafe { &mut *self.$ArchName.storage.get() };
                            // obtain mutable slices to each requested component vector
                            $(
                                #[allow(nonstandard_style)]
                                let $Comp = unsafe {
                                    crate::access_component_field_mut!(arch_mut_ref, $Comp).get_unchecked_mut(dense_usize)
                                };
                            )*

                            let entity_refs = ${concat($ArchName, EntityRefs)} {
                                $(
                                    $Comp: $Comp,
                                )*
                            };

                            // The return type ArchEntityRefs<'_> now contains mutable references that
                            // the Rust compiler cannot track correctly, leading to UB if misused.
                            Some(ArchEntityRefs::$ArchName(entity_refs))
                        }
                    )*
                }
            }
        }

        // Main "iter through components macro"
        // Double-dollar is basically an "escaped" dollar to make so its expanded NOT by declare_ecs,
        // but passed down to query (as single dollar)
        #[macro_export]
        macro_rules! query {
            // Pattern: query!( world_expr, | arg: &mut Type, ... | { lambda body } )
            ( $world_expr:expr, [ $$( $$QArg:ident : &mut $$QTy:ident ),* ] $body:block ) => {
                {
                    $(
                        // emit this archetype's loop only if it contains ALL requested component types
                        $crate::if_all_present!(($($Comp),*); $$($$QTy),*; {
                            let len = $world_expr.$ArchName.dense_len();
                            // obtain mutable slices to each requested component vector
                            let arch_mut_ref = unsafe { &mut *$world_expr.$ArchName.storage.get() };

                            $$(
                                #[allow(nonstandard_style)]
                                let $$QArg = unsafe {
                                    crate::access_component_field_mut!(arch_mut_ref, $$QTy)
                                };
                            )*

                            // iter through entities in this archetype, load necessary components and execute lambda
                            for i in 0..len {
                                $$(
                                    // bounds checking in debug
                                    #[allow(nonstandard_style)]
                                    let $$QArg = if cfg!(debug_assertions) {
                                        &mut $$QArg[i]
                                    } else {
                                        unsafe { $$QArg.get_unchecked_mut(i) }
                                    };
                                )*
                                unsafe {
                                    // so weird to do this to lambdas lol
                                    $body
                                }
                            }
                        } {});
                    )*
                }
            };
        }

        #[macro_export]
        macro_rules! extract_components_from_refs {
            // Pattern:
            // extract_components_from_refs!( enum_with_refs_variable, [Position, Velocity] )
            (
                $refs_enum:expr,
                [ $$( $$EComp:ident ),* ]
            ) => {{
                // We need to use `match` on the enum to see which archetype it is
                let result: Option<( $$( &mut $$EComp ),* )> = match $refs_enum {
                    $(
                        ArchEntityRefs::$ArchName(refs) => {
                            // Check if this archetype contains ALL requested components [ $( $Comp ),* ]
                            $crate::if_all_present!( ($($Comp),*); $$($$EComp ),*; {
                                // If all components are present, return the tuple of references
                                Some((
                                    $$(
                                        unsafe {&mut *refs.$$EComp}
                                    ),*
                                ))
                            }
                            // The provided archetype did not have those fields
                            {None}
                            )
                        }
                    ),*
                };
                result
            }}
        }
    };
}
