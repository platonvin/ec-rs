This is a little bit a discussion too, but `Discussions` sessions is empty and scary :D

First of all, im impressed by:
1) compilation time*
2) bloat

gecs does not bloat my compile times. does not bloat my dependency tree and does not bloat my binary. It also does most important type of workload (for me) at theoretically maximum perfomance (just loading proper elements from proper component arrays and executing given function on them). Gecs is defenetly an example of zero-cost-abstractions
* on small projects. Im yet to create something complicated with ecs
I would love to use gecs, however:

1) multithreading. Personally, i dont care about "safety", and would leave non-trivial sync up to the user. Something i would want from gecs is passing processing lambda in same way, and gecs giving it to busy-waiting threads with sync using something like atomic doing_work_threads integer for processing corresponding slice of entities in same/different archetypes (same if it is big, different if they are small)
2) simplicity. Gecs is currently sitting at 5.5 kloc. Do you think it is possible to make gecs simpler? 
3) debug functionality being optional

Some complexity comes from different intermediate wrappers. I think 
Some complexity comes from gecs being proc macro. It is likely true that for current way of doing things proc macro is simpler, but quite possible "simpler gecs" could be simpler in declarative macro. I played around a bit and made (nightly-only) declarative gecs-like thing:
* also no multithreading, and needs interior mutability
* single allocation per archetype component arrays (could fit empty into it too)
* debug-only checks
* declarative macro. Im not sure if its possible to have rust-analyzer working with it tho (no syntax highlighting / autocomplete in query macro lambda)
* ~700 loc


Im sorry, i dont **exactly** know what i want. Perhaps, simpler (its hard to figure out what is going on in gecs), less loc, feature to turn off validation, some way of doing mulithreading. Also, example on how things like "system that iterates thorugh enteties which have references (handles) to other entities and fetch data from them) would be appretiated.
How about we make some sort of "gecs but less features, threaded executor api, single allocation per component, no borrow checking rules (full interior mutability) - its up to user"
so no runtime borrows, views, slices of mutable reference, dozens of layers of traits, abstractions over comptime/runtime size.
Just:
define archetypes with components
archetypes 
iterate over components
get entity by component

Gecs looks lovely, but i have read its docs, and i feel like i understand less than before i started.
i have to note that i give no fuck about "safe" code and idiomatic names 