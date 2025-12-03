#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]

/// Compact 8-byte handle that identifies an entity across the World.  
/// Expected layout (LSB → MSB):  
/// - 0..32  : slot_index  (u32)  
/// - 32..48 : generation  (u16)  
/// - 48..56 : arch_id     (u8)  
/// - 56..64 : reserved for your favourite cat image hash  
///  
/// The actual field order is left to the compiler.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Handle {
    pub slot_index: u32,
    pub generation: u16,
    pub arch_id: u8,
    // pub your_fav_cat_img_hash: u8,
}
const _: () = assert!(std::mem::size_of::<Handle>() <= 8);

/// if A same as B {then} {else}
#[macro_export]
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

/// if every `want` appears in `haves` {then} {else}
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
// TODO: remove when nested concat's are stable
macro_rules! generate_storage_recursive {
    ( $ArchName:ident, { $( $field:ident : $type:ident, )* }, {} ) => {
        #[allow(nonstandard_style)]
        sibling_vecs::sibling_vecs! {
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
    ( $a:ident ) => { ${concat($a, ComponentStorage)} };
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

/// Main ECS generator macro. Invoke it somewhere to generate types, functions and acess macros.
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

        pub const ARCH_COUNT: usize = ${count($ArchName)};

        #[allow(nonstandard_style)]
        pub enum ArchEntityRefs {
            $(
                #[allow(nonstandard_style)]
                $ArchName(${concat($ArchName, EntityRefs)}),
            )*
        }

        $(
            #[allow(nonstandard_style)]
            $crate::generate_storage_recursive!( $ArchName, {}, { $($Comp),* } );

            #[allow(nonstandard_style)]
            pub struct ${concat($ArchName, EntityRefs)} {
                $( pub $Comp: *mut $Comp, )*
            }

            #[allow(nonstandard_style)]
            pub struct $ArchName {
                /// Component storage
                pub storage: std::cell::UnsafeCell<${concat($ArchName, ComponentStorage)}>,
                /// Maps SlotIndex -> DenseIndex
                pub slots: Vec<u32>,
                /// Maps DenseIndex -> SlotIndex
                pub dense_to_slot: Vec<u32>,
                /// Recycled slot free list
                pub free_slots: std::collections::VecDeque<u32>,
                /// Generation counter per slot (u16). Only used in debug builds.
                #[cfg(debug_assertions)]
                pub slot_generations: Vec<u16>,
            }

            #[allow(nonstandard_style)]
            impl $ArchName {
                pub fn new() -> Self {
                    Self {
                        storage: std::cell::UnsafeCell::new(${concat($ArchName, ComponentStorage)}::new()),
                        slots: Vec::new(),
                        dense_to_slot: Vec::new(),
                        free_slots: std::collections::VecDeque::new(),
                        #[cfg(debug_assertions)]
                        slot_generations: Vec::new(),
                    }
                }

                /// Spawn an entity into this archetype.
                /// Returns the handle (slot_index, generation).
                /// Since you know which arch it comes from, you can access it with zero arch lookup overhead.
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

                    storage.push( $($Comp,)* );

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

                /// Despawn an entity given its handle.
                /// Since you know which arch it comes from, there is no arch lookup overhead.
                pub fn despawn(&mut self, handle: $crate::Handle) {
                    let storage = unsafe { &mut *self.storage.get() };
                    // this despawn is on specific arch, this just checks correctness
                    // if you need "idk which arch just delete this" use world.despawn
                    debug_assert_eq!(handle.arch_id, ArchId::$ArchName.as_u8());

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

                $(
                    #[inline]
                    #[allow(nonstandard_style)]
                    pub fn $Comp(&self) -> &[$Comp] {
                        unsafe { (*self.storage.get()).$Comp() }
                    }
                )*
            }
        )*

        /// The world contains one archetype struct per declared archetype.
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
                $( self.$ArchName.clear_preserve_capacity(); )*
            }

            /// Returns mutable references to an entity's components.
            /// **Safety**: The returned references may alias arbitrarily; the caller must ensure
            /// exclusive access.
            pub unsafe fn get_entity_mut(&self, handle: $crate::Handle) -> Option<ArchEntityRefs> {
                // our layout is kinda like an array, but with named elements. This compiles to effectively
                // (unchecked) array element access. TODO: does compiler move elements around to optimize access?
                let arch_enum = unsafe { std::mem::transmute::<u8, ArchId>(handle.arch_id) };
                match arch_enum {
                    $(
                        ArchId::$ArchName => {
                            let arch_ref = &self.$ArchName;
                            let slot_usize = handle.slot_index as usize;

                            if slot_usize >= arch_ref.slots.len() { return None; }
                            let dense = arch_ref.slots[slot_usize];
                            let dense_usize = dense as usize;

                            if dense == u32::MAX || dense_usize >= unsafe { (*arch_ref.storage.get()).len } {
                                return None;
                            }

                            let arch_mut_ref = unsafe { &mut *self.$ArchName.storage.get() };

                            let entity_refs = ${concat($ArchName, EntityRefs)} {
                                $( $Comp: unsafe {
                                    crate::access_component_field_mut!(arch_mut_ref, $Comp)
                                        .get_unchecked_mut(dense_usize)
                                }, )*
                            };

                            Some(ArchEntityRefs::$ArchName(entity_refs))
                        }
                    )*
                }
            }
        }

        /// Iterate over entities that have *all* requested component types, with mutable access.
        /// There is no way to have non-mutable access.
        #[macro_export]
        macro_rules! query {
            // Pattern: query!( world_expr, | arg: &mut Type, ... | { lambda body } )
            ( $world_expr:expr, | $$( $$QArg:ident : &mut $$QTy:ident ),* | $body:block ) => {
                {
                    // emit this archetype's loop only if it contains ALL requested component types
                    $(
                        // since its not-trivial, ill explain:
                        // we expand this loop from top-level declare_ecs macro. I.e. we expand this loop FOR EACH architecture
                        // and ($($Comp),*) turns into list of components for this architecture
                        // to make so something does not try to expand right away and instead turns into
                        // syntax for expansion for generated macro, we "escape" expansion symbol $ with $ (so doulbe dollar $$)
                        $crate::if_all_present!( ($($Comp),*) ; $$($$QTy),* ; {
                            let len = $world_expr.$ArchName.dense_len();
                            // obtain mutable slices to each requested component vector. `Storage` is in UnsafeCell
                            let arch_mut_ref = unsafe { &mut *$world_expr.$ArchName.storage.get() };

                            $$(
                                let $$QArg = unsafe {
                                    crate::access_component_field_mut!(arch_mut_ref, $$QTy)
                                };
                            )*

                            // iter through entities in this archetype, load necessary components and execute lambda
                            for i in 0..len {
                                $$(
                                    let $$QArg = if cfg!(debug_assertions) {
                                        &mut $$QArg[i]
                                    } else {
                                        unsafe { $$QArg.get_unchecked_mut(i) }
                                    };
                                )*
                                $body
                            }
                        } {});
                    )*
                }
            };
        }

        #[macro_export]
        macro_rules! extract_components_from_refs {
            (
                $$refs_enum:expr,
                [ $$( $$EComp:ident ),* ]
            ) => {{
                let result: Option<( $$( &mut $$EComp ),* )> = match $$refs_enum {
                    $(
                        ArchEntityRefs::$ArchName(refs) => {
                            $crate::if_all_present!( ($($Comp),*) ; $$($$EComp),* ; {
                                unsafe { Some(( $$( &mut *refs.$$EComp ),* )) }
                            } { None })
                        }
                    ),*
                };
                result
            }};
        }
    };
}
