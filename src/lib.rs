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
                // injected metadata
                slots: u32,
                dense_to_slot: u32,
                free_slots: u32,
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
        #[derive(Clone)]
        pub enum ArchEntityRefs {
            $(
                #[allow(nonstandard_style)]
                $ArchName(${concat($ArchName, EntityRefs)}),
            )*
        }
        pub trait AsArchEntityEnumRef {
            fn as_arch_ref(&self) -> ArchEntityRefs;
        }
        impl AsArchEntityEnumRef for ArchEntityRefs {
            fn as_arch_ref(&self) -> ArchEntityRefs {self.clone()}
        }

        $(
            #[allow(nonstandard_style)]
            $crate::generate_storage_recursive!( $ArchName, {}, { $($Comp),* } );

            // We have them as separate structs because enum variants are not types and we need to return exact variant for exact arch.
            #[allow(nonstandard_style)]
            #[derive(Clone)]
            pub struct ${concat($ArchName, EntityRefs)} {
                $( pub $Comp: *mut $Comp, )*
            }

            impl AsArchEntityEnumRef for ${concat($ArchName, EntityRefs)} {
                fn as_arch_ref(&self) -> ArchEntityRefs {
                    ArchEntityRefs::$ArchName(self.clone())
                }
            }

            #[allow(nonstandard_style)]
            pub struct $ArchName {
                /// Component storage (including slots).
                pub storage: std::cell::UnsafeCell<${concat($ArchName, ComponentStorage)}>,
                /// Number of valid items in the `free_slots` component array (in storage).
                pub free_slots_len: usize,
                /// Generation counter per slot (u16). Only used in debug builds.
                #[cfg(debug_assertions)]
                pub slot_generations: Vec<u16>,
            }

            #[allow(nonstandard_style)]
            impl $ArchName {
                pub fn new() -> Self {
                    Self {
                        storage: std::cell::UnsafeCell::new(${concat($ArchName, ComponentStorage)}::new()),
                        free_slots_len: 0,
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

                    // 1) determine slot index (recycle or new)
                    let slot_index: u32 = if self.free_slots_len > 0 {
                        self.free_slots_len -= 1;
                        unsafe { *storage.free_slots().get_unchecked(self.free_slots_len) }
                    } else {
                        // use current length (so at new allocated entity) as the new slot index
                        // effectively extending the slot map (aka no free slots)
                        // if there is not enough space in storage... Its problem of the storage
                        storage.len() as u32
                    };

                    #[cfg(debug_assertions)]
                    {
                        if (slot_index as usize) >= self.slot_generations.len() {
                            self.slot_generations.push(0u16);
                        }
                    }

                    let dense_index = storage.len() as u32;

                    // 2) push components & mappings to storage
                    storage.push( $( $Comp, )*
                        dense_index, // slot: maps to entity at dense index
                        slot_index,  // dense_to_slot: maps from entity "at this" to (index of) slot
                        // not how dense_index and slot_index might be equal (e.g. if entities are never freed)
                        0 // placeholder data for free slots deque
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

                /// Despawn an entity given its handle. Note: since this is architecture-specific despawn,
                /// i.e. in release mode arch_id is actually not used in any way.
                pub fn despawn(&mut self, handle: $crate::Handle) {
                    let storage = unsafe { &mut *self.storage.get() };
                    debug_assert_eq!(handle.arch_id, ArchId::$ArchName.as_u8());
                    // handle has slot index
                    // we go to slot at slot index
                    // slot contains dense index
                    // dense index is index of (*this) entity components in arrays of components
                    // including component "mapping back to slot index"

                    let slot_index_usize = handle.slot_index as usize;

                    // look up dense index from the 'slots' component
                    let dense_index = unsafe { *storage.slots().get_unchecked(slot_index_usize) };
                    let dense_index_usize = dense_index as usize;

                    debug_assert_ne!(dense_index, u32::MAX);
                    debug_assert!(dense_index_usize < storage.len());

                    let last_dense_index_usize = storage.len() - 1;

                    // swap remove from last to 'dense'
                    // on deletion, we dont care about components of deleted entity
                    // (from now on, component arrays wont be mentioned since functionally its safe as AoS)
                    // the way we keep data truly sparse is by swapping entity "to delete" with last one
                    // and then treat last one as free. So last one is moved to place of deleted one
                    // but wait, how do all Handles still point to same entity?
                    // Since it is impossible to make all Handles magically point to new entity,
                    // we have layer of indirection - slots - that our Handles point to (by index not ptr, since those can move in memory)
                    // in lifetime of entity, its slot is never moving
                    // it can, however, change content - index of entity data in sparse array

                    storage.swap_remove(dense_index_usize);

                    // fix up the swap
                    // our swap_remove fucked up guarantee of "slot" itself not moving. Gotta bring it back
                    let was_last = dense_index_usize == last_dense_index_usize;
                    if !was_last {
                        // entity that was last is now at 'dense'

                        // we look into backward mapping into slot index index of stil alive entity, moved into new (hole) spot
                        // so we get index of slot of still alive entity in array of slots. Lets update it to point to new location
                        let moved_slot_index = unsafe { *storage.dense_to_slot().get_unchecked(dense_index_usize) };

                        // update the sparse map: moved entity's slot now points to 'dense_index'
                        unsafe {
                            *storage.slots_mut().get_unchecked_mut(moved_slot_index as usize) = dense_index;
                        }
                    }

                    // return slot to free list
                    // we store the freed slot index in the `free_slots` component array, used effectively as a stack
                    // could just use a stack but free_slots is guaranteed to be at most size of allocated arrays
                    // `free_slots_len` if effectively a stack pointer
                    unsafe {
                        let free_slots_ptr = storage.free_slots_mut().as_mut_ptr();
                        *free_slots_ptr.add(self.free_slots_len) = handle.slot_index;
                    }
                    self.free_slots_len += 1;

                    #[cfg(debug_assertions)]
                    {
                        let generation = &mut self.slot_generations[slot_index_usize];
                        debug_assert_eq!(handle.generation, *generation);
                        *generation = generation.wrapping_add(1);
                    }

                    unsafe {
                        let slots_ptr = storage.slots_mut().as_mut_ptr();
                        *slots_ptr.add(slot_index_usize) = u32::MAX;
                    }
                }

                pub fn dense_len(&self) -> usize {
                    unsafe { (*self.storage.get()).len() }
                }

                pub fn clear_preserve_capacity(&mut self) {
                    let storage = unsafe { &mut *self.storage.get() };

                    #[cfg(debug_assertions)]
                    for generation in &mut self.slot_generations {
                        *generation = generation.wrapping_add(1);
                    }

                    self.free_slots_len = 0;
                    storage.clear();
                }


                /// Returns mutable pointers to all entity components.
                pub unsafe fn get_entity_mut(&self, handle: $crate::Handle) -> ${concat($ArchName, EntityRefs)} {
                    let arch_enum = unsafe { std::mem::transmute::<u8, ArchId>(handle.arch_id) };
                    debug_assert_eq!(arch_enum, ArchId::$ArchName);

                    let storage = unsafe { &mut *self.storage.get() };
                    let slot_usize = handle.slot_index as usize;

                    let dense = unsafe { *storage.slots().get_unchecked(slot_usize) };
                    let dense_usize = dense as usize;

                    if dense == u32::MAX || dense_usize >= storage.len() {
                        panic!();
                    }

                    let entity_refs = ${concat($ArchName, EntityRefs)} {
                        $( $Comp: unsafe {
                            crate::access_component_field_mut!(storage, $Comp)
                                .get_unchecked_mut(dense_usize)
                        }, )*
                    };

                    // ArchEntityRefs::$ArchName(entity_refs)
                    entity_refs
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

            /// Returns mutable pointers to all entity components.
            pub unsafe fn get_entity_mut(&self, handle: $crate::Handle) -> ArchEntityRefs {
                let arch_enum = unsafe { std::mem::transmute::<u8, ArchId>(handle.arch_id) };
                match arch_enum {
                    $(
                        ArchId::$ArchName => {
                            let arch = &self.$ArchName;
                            ArchEntityRefs::$ArchName(arch.get_entity_mut(handle))
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
                $$refs_enum_or_exact_struct:expr,
                [ $$( $$EComp:ident ),* ]
            ) => {{
                let refs_enum = $$refs_enum_or_exact_struct.as_arch_ref();
                let result: Option<( $$( &mut $$EComp ),* )> = match refs_enum {
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
