#![allow(unused)]
#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]

#[macro_export]
macro_rules! sibling_vecs {
    (
        $vis:vis struct $name:ident {
            $( $field:ident : $type:ty ),* $(,)?
        }
    ) => {
        // Main struct for all sibling sub-vecs.
        $vis struct $name {
            ptr: *mut u8,
            len: usize,
            cap: usize,
        }

        impl $name {
            pub const N: usize = ${count($type)};

            pub fn new() -> Self {
                Self {
                    ptr: std::ptr::null_mut(),
                    len: 0,
                    cap: 0,
                }
            }
            pub fn with_capacity(cap: usize) -> Self {
                let mut s = Self::new();
                s.reallocate_to(cap);
                s
            }

            pub fn len(&self) -> usize {
                self.len
            }
            pub fn capacity(&self) -> usize {
                self.cap
            }

            const fn type_infos() -> [(usize, usize); Self::N] {
                [ $( (std::mem::size_of::<$type>(), std::mem::align_of::<$type>()) ),* ]
            }

            fn offsets(cap: usize) -> [usize; Self::N] {
                let infos = Self::type_infos();
                let mut out = [0; Self::N];
                let mut current_offset = 0;

                let mut i = 0;
                // TODO: should we manually unroll with macro? Where does it start optimizing away?
                while i < Self::N {
                    let (size, align) = infos[i];

                    if align > 0 {
                        let remainder = current_offset % align;
                        if remainder != 0 {
                            current_offset += align - remainder;
                        }
                    }

                    out[i] = current_offset;
                    current_offset += cap * size;
                    i += 1;
                }
                out
            }

            fn layout(cap: usize) -> std::alloc::Layout {
                if cap == 0 {
                    return std::alloc::Layout::new::<u8>();
                }

                let infos = Self::type_infos();
                let offsets = Self::offsets(cap);

                let last_idx = Self::N - 1;
                let (last_size, _) = infos[last_idx];
                let total_size = offsets[last_idx] + (cap * last_size);

                let mut max_align = 1;
                let mut i = 0;
                while i < Self::N {
                    let (_, align) = infos[i];
                    if align > max_align { max_align = align; }
                    i += 1;
                }

                std::alloc::Layout::from_size_align(total_size, max_align).unwrap()
            }

            // Helper function for allocation-related stuff.
            fn reallocate_to(&mut self, new_cap: usize) {
                let old_cap = self.cap;
                if new_cap == old_cap { return; }

                // deallocate
                if new_cap == 0 {
                    if old_cap > 0 {
                        unsafe {
                            std::alloc::dealloc(self.ptr, Self::layout(old_cap));
                        }
                    }
                    self.ptr = std::ptr::null_mut();
                    self.cap = 0;
                    self.len = 0;
                    return;
                }

                let old_layout = Self::layout(old_cap);
                let new_layout = Self::layout(new_cap);

                unsafe {
                    // alloc or realloc
                    let new_ptr = if old_cap == 0 {
                        std::alloc::alloc(new_layout)
                    } else {
                        // TODO: realloc nullptr?
                        std::alloc::realloc(self.ptr, old_layout, new_layout.size())
                    };

                    if new_ptr.is_null() { std::alloc::handle_alloc_error(new_layout); }

                    self.ptr = new_ptr;
                    self.cap = new_cap;
                    // TODO: thing is, if we stay in-memory we do actually want shifting
                    // but when we cant, and realloc would move, we would rather avoid copy-all-then-move and straight up copy once but properly

                    if self.len > 0 && old_cap > 0 {
                        let old_offsets = Self::offsets(old_cap);
                        let new_offsets = Self::offsets(new_cap);
                        let infos = Self::type_infos();

                        // shift data (if realloc)
                        // reverse because otherwise we will overwrite data of next sub-vec
                        // from 1 cause 0th does not need to be shifted
                        for i in (1..Self::N).rev() {
                            let (size, _) = infos[i];
                            let size_bytes = self.len * size;

                            let src = self.ptr.add(old_offsets[i]);
                            let dst = self.ptr.add(new_offsets[i]);

                            std::ptr::copy(src, dst, size_bytes);
                        }
                    }
                }
            }

            fn grow(&mut self) {
                // does not actually matter, it is intended to never really shrink
                let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
                self.reallocate_to(new_cap);
            }


            pub fn swap_remove(&mut self, index: usize) -> ($( $type ),*) {
                debug_assert!(index < self.len);

                let offsets = Self::offsets(self.cap);
                let last_idx = self.len - 1;

                unsafe {
                    let result = (
                        $(
                            {
                                let offset = offsets[${index()}];
                                let base = self.ptr.add(offset) as *mut $type;
                                let dst = base.add(index);

                                let val = std::ptr::read(dst);

                                if index != last_idx {
                                    let src = base.add(last_idx);
                                    std::ptr::copy_nonoverlapping(src, dst, 1);
                                }

                                val
                            }
                        ),*
                    );
                    self.len -= 1;

                    result
                }
            }

            pub fn clear(&mut self) {
                if self.len == 0 { return; }

                let offsets = Self::offsets(self.cap);
                let len = self.len;

                unsafe {
                    $(
                        if std::mem::needs_drop::<$type>() {
                            let offset = offsets[${index()}];
                            let base = self.ptr.add(offset) as *mut $type;
                            for i in 0..len {
                                std::ptr::drop_in_place(base.add(i));
                            }
                        }
                    )*
                }
                self.len = 0;
            }

            #[allow(nonstandard_style)]
            pub fn push(&mut self, $( $field : $type ),* ) {
                if self.len == self.cap {
                    self.grow();
                }

                let offsets = Self::offsets(self.cap);

                unsafe {
                    $(
                        // since we are inside a repetition $()* for methods,
                        // ${index()} gives us the index of the current iteration
                        let offset = offsets[${index()}];
                        let type_base = self.ptr.add(offset) as *mut $type;
                        type_base.add(self.len).write($field);
                    )*
                }
                self.len += 1;
            }

            // nice thing is that there is no bounds checking and its up to user
            pub fn as_slices(&self) -> ( $( &[$type] ),* ) {
                let offsets = Self::offsets(self.cap);
                unsafe {
                    (
                        $(
                            std::slice::from_raw_parts(
                                self.ptr.add(offsets[${index()}]) as *const $type,
                                self.len
                            )
                        ),*
                    )
                }
            }

            // nice thing is that there is no bounds checking and its up to user
            pub fn as_mut_slices(&mut self) -> ( $( &mut [$type] ),* ) {
                let offsets = Self::offsets(self.cap);
                unsafe {
                    (
                        $(
                            std::slice::from_raw_parts_mut(
                                self.ptr.add(offsets[${index()}]) as *mut $type,
                                self.len
                            )
                        ),*
                    )
                }
            }

            $(
                #[allow(nonstandard_style)]
                pub fn $field(&self) -> &[$type] {
                     let offsets = Self::offsets(self.cap);
                     let idx = ${index()};
                     unsafe {
                         std::slice::from_raw_parts(
                             self.ptr.add(offsets[idx]) as *const $type,
                             self.len
                         )
                     }
                }
            )*

            $(
                #[allow(nonstandard_style)]
                pub fn ${concat($field, _mut)}(&self) -> &mut [$type] {
                     let offsets = Self::offsets(self.cap);
                     let idx = ${index()};
                     unsafe {
                         std::slice::from_raw_parts_mut(
                             self.ptr.add(offsets[idx]) as *mut $type,
                             self.len
                         )
                     }
                }
            )*
        }

        impl Drop for $name {
            fn drop(&mut self) {
                if self.cap > 0 {
                    debug_assert!(!self.ptr.is_null());
                    let offsets = Self::offsets(self.cap);

                    unsafe {
                        $(
                            // should be fine to just drop but anyways
                            if std::mem::needs_drop::<$type>() {
                                let offset = offsets[${index()}];
                                let base = self.ptr.add(offset) as *mut $type;
                                for i in 0..self.len {
                                    std::ptr::drop_in_place(base.add(i));
                                }
                            }
                        )*
                        std::alloc::dealloc(self.ptr, Self::layout(self.cap));
                    }
                }
            }
        }
    };
}

/// Compact handle that identifies an entity across the World.
///
/// expected layout: [LSB..MSB)
/// - 0..32  : slot_index  (u32)
/// - 32..48 : generation  (u16)
/// - 48..56 : arch_id     (u8)
/// - 56..64 : reserved for your fav cat image hash
/// in reality we dont really specify it and leave it up to the compiler
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Handle {
    pub slot_index: u32,
    pub generation: u16,
    pub arch_id: u8,
    // pub your_fav_cat_img_hash: u8,
}
// const _: () = {
//     assert!(std::mem::size_of::<Handle>() <= 8);
// };

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
// TODO: remove when nested concat's are a thing
macro_rules! generate_storage_recursive {
    ( $ArchName:ident, { $( $field:ident : $type:ident, )* }, {} ) => {
        //  sibling_vecs::sibling_vecs! {
        #[allow(nonstandard_style)]
        $crate::sibling_vecs! {
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
        // Note how it is expanding macro, generatated in macro
        // to pull this off we sometimes need to use $$ instead of $
        #[macro_export]
        macro_rules! query {
            // Pattern: query!( world_expr, | arg: &mut Type, ... | { lambda body } )
            ( $world_expr:expr, [ $$( $$QArg:ident : &mut $$QTy:ident ),* ] $body:tt ) => {
                {
                    $(
                        // emit this archetype's loop only if it contains ALL requested component types
                        // double dollar because thats how nested macros work
                        $crate::if_arch_matches!( ($($Comp),*); $$($$QTy),*; {
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
                                    // $body
                                }
                            }
                        });
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
                            $crate::if_arch_matches!( ($($Comp),*); $$($$EComp ),*; {
                                // If all components are present, return the tuple of references
                                Some((
                                    // LOL we can do single dollar sign here
                                    $$(
                                        refs.$$EComp
                                    ),*
                                ))
                            });
                            // The provided archetype did not have those fields
                            None
                        }
                    ),*
                };
                result
            }};
        }
    };
}
