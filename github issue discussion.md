Skip to content
Navigation Menu
rust-lang
rust

Type / to search
Code
Issues
5k+
Pull requests
895
Actions
Projects
9
Security
6
Insights
Tracking issue: declarative macros 2.0 #39412
Open
Open
Tracking issue: declarative macros 2.0
#39412
@nrc
Description
nrc
opened on Jan 30, 2017 · edited by madsmtm
Member
Tracking issue for declarative macros 2.0 (aka macro aka decl_macro aka macros-by-example).

RFC: https://rust-lang.github.io/rfcs/1584-macros.html

RFC PR: rust-lang/rfcs#1584

cc @rust-lang/compiler

About tracking issues
Tracking issues are used to record the overall progress of implementation.
They are also used as hubs connecting to other relevant issues, e.g., bugs or open design questions.
A tracking issue is however not meant for large scale discussion, questions, or bug reports about a feature.
Instead, open a dedicated issue for the specific matter and add the relevant feature gate label.
Discussion comments will get marked as off-topic or deleted.
Repeated discussions on the tracking issue may lead to the tracking issue getting locked.

Tasks

Complete the draft hygiene prototype and RFC.

Hygiene for items, explicit imports, lexical scopes, and module scopes.

Hygiene for globs and trait methods (w.r.t. extension trait candidates in scope for a method call).

Hygiene for type directed name resolutions (i.e. methods, associated types, and fields).

Support macro in blocks as well as modules.

Implement inter-crate hygiene, except for nested macros.

Implement "hygiene bending" for when users want a name from macro def to "escape".
Examples: New declarative macros, functions and fields not being recognized #91249, Declaring modules inside decl macros 2.0 #46342

Implement inter-crate hygiene for nested macros (pending macro def encoding).

Make private_in_public hygienic?

Make unsafe and lints hygienic (if appropriate)?

Add variant ast::ItemKind::MacroDef for macro_rules! items and macro items (PR syntax: add ast::ItemKind::MacroDef, simplify hygiene info #40220).

Encode macros in the crate metadata using TokenStream, not String.

Fix Inferred types should be checked for privacy violations #30476 (private_in_public details).

Fix span issues If an error is not affected by a macro, don't print macro notes #30506 and Spans for Paths can be incorrect #39450 (PR macros: improve Span's expansion information #40597).

Land macro behind a feature gate (PR Initial implementation of declarative macros 2.0 #40847).

Future-proof matchers (e.g. $e:expr) by employing a simpler, more general grammar.

Allow fragments (e.g. $e where $e:expr) to be parsed in more contexts (c.f. Macro matching is incorrect for macros invoked in macro expansions #26361).

Decide whether we want macro invocations in identifier positions and/or eager expansion.

Remove $:meta matcher (Remove $:meta matcher in decl macros #49629)
Potentially blocking issues:

Declarative macros: no error about duplicate trait items #71614
Activity

nrc
added 
A-macros
Area: All kinds of macros (custom derive, macro_rules!, proc macros, ..)
 
B-RFC-approved
Blocker: Approved by a merged RFC but not yet implemented.
 
B-unstable
Blocker: Implemented in the nightly compiler and unstable.
 
T-lang
Relevant to the language team
 on Jan 30, 2017
CryZe
CryZe commented on Jan 30, 2017
CryZe
on Jan 30, 2017
Contributor
@nrc Typo in Issue Name: "issuse"


nikomatsakis
changed the title [-]Tracking issuse: declarative macros 2.0[/-] [+]Tracking issue: declarative macros 2.0[/+] on Jan 31, 2017

jseyfried
mentioned this on Jan 31, 2017
Simplify TokenTree and fix macro_rules! bugs #39419

nrc
mentioned this on Feb 2, 2017
Macros by example 2.0 (macro!) rfcs#1584
jseyfried
jseyfried commented on Feb 7, 2017
jseyfried
on Feb 7, 2017 · edited by dtolnay
Contributor
Tasks
(dtolnay edit: moved the checklist up to the OP)

nrc
nrc commented on Feb 7, 2017
nrc
on Feb 7, 2017
Member
Author
We need to RFC a whole bunch of stuff here. In particular, I would like to propose some new syntax for declaring macros and we should RFC the changes to matchers.


jseyfried
mentioned this in 2 issues on Feb 8, 2017
Macro expansion often produces invalid Span values #23480
gdb has the wrong source location with many macros #18285

jseyfried
mentioned this in 2 pull requests on Mar 3, 2017
Macros 1.2: Fast-track to stabilize function-like procedural macros rfcs#1913
syntax: add ast::ItemKind::MacroDef, simplify hygiene info #40220
tikue
tikue commented on Mar 10, 2017
tikue
on Mar 10, 2017
Contributor
Can the hygiene RFC mention pattern hygiene? This in particular scares me:

// Unsuspecting user's code
#[allow(non_camel_case_types)]
struct i(i64);

macro_rules! ignorant_macro {
    () => {
        let i = 0;
        println!("{}", i);
    };
}

fn main() {
    // oh no!
    ignorant_macro!();
}
jseyfried
jseyfried commented on Mar 13, 2017
jseyfried
on Mar 13, 2017 · edited by jseyfried
Contributor
@tikue I'm not sure patterns need special treatment with respect to hygiene.

For example, on the hygiene prototype,

#[allow(non_camel_case_types)]
struct i(i64);

macro ignorant_macro() {
    let i = 0; // ERROR: let bindings cannot shadow tuple structs
    println!("{}", i); 
}

fn main() {
    ignorant_macro!(); // NOTE: in this macro invocation
}
This makes sense to me since let i = 0; is shadowing a tuple struct.
In particular, if let i = 0; were removed then the following use of i would resolve to the tuple struct, no matter where ignorant_macro is used.

Note the symmetry to this example:

#[allow(non_camel_case_types)]
struct i(i64);

fn ignorant_fn() {
    let i = 0; // ERROR: let bindings cannot shadow tuple structs
    println!("{}", i); 
}
If the tuple struct i isn't in scope at macro ignorant_macro() { ... }, then the let i = 0; does not shadow it and there is no error. For example, the following compiles on the hygiene prototype:

mod foo {
    pub macro ignorant_macro() {
        let i = 0;
        println!("{}", i); // Without `let i = 0;`, there would be no `i` in scope here.
    }
}

// Unsuspecting user's code
#[allow(non_camel_case_types)]
struct i(i64);

fn main() {
    foo::ignorant_macro!();
}

jseyfried
mentioned this on Mar 17, 2017
macros: improve Span's expansion information #40597

kennytm
mentioned this on Jun 29, 2017
Only match a fragment specifier if it starts with certain tokens. #42913

Mark-Simulacrum
added 
C-tracking-issue
Category: An issue tracking the progress of sth. like the implementation of an RFC
 on Jul 22, 2017
alexreg
alexreg commented on Aug 17, 2017
alexreg
on Aug 17, 2017
Contributor
Has there been any progress on Macros 2.0 lately?


SergioBenitez
mentioned this on Sep 15, 2017
Compile with stable Rust rwf2/Rocket#19

jan-hudec
mentioned this on Oct 5, 2017
Tracking issue for RFC 1566: Procedural macros #38356

Geal
mentioned this on Dec 2, 2017
follow and test macros 2.0 rust-bakery/nom#632

dtolnay
mentioned this on Jan 27, 2018
Using Macros 2.0 to generate a module does not allow access to items #47797
mark-i-m
mark-i-m commented on Jan 30, 2018
mark-i-m
on Jan 30, 2018 · edited by mark-i-m
Contributor
Note for those who haven't seen yet: macros 2.0 is apparently slated to be stable later this year, according to the proposed roadmap (rust-lang/rfcs#2314)...

On the one hand, that's pretty exciting 🎉!

On the other hand, I was really surprised that the feature is so close to being done and so little is known about it by the broader community... The RFC is really vague. The unstable book only has a link to this issue. This issue(s) in the issue tracker mostly have detailed technical discussions. And I can't find that much info anywhere about what's changed or how stuff works.

I don't mean to complain, and I really truly appreciate all the hard work by those implementing, but I would also appreciate more transparency on this.

alexreg
alexreg commented on Jan 30, 2018
alexreg
on Jan 30, 2018
Contributor
@marcbowes Yeah, I'm kind of worried too. It seems like there's quite a lot of work left for stabilisation this year. I offered to work on opt-out hygiene for identifiers myself, but have received no response yet... Some greater transparency would be nice, as you say.

petrochenkov
petrochenkov commented on Jan 30, 2018
petrochenkov
on Jan 30, 2018
Contributor
Macros 2.0 are not even in the RFC stage yet - the @jseyfried's RFC linked in the issue was never submitted, there's only very high level RFC 1584.
The implementation is purely experimental and mostly unspecified, and large part of it (not related to hygiene) is reused from macro_rules! without fixing problems that macros 2.0 were supposed to fix.

Some greater transparency would be nice

The problem is that no work happen right now, so there's nothing to reveal :(
@nrc is busy, @jseyfried is busy, I worked on macros 2.0 a bit and would really like to dig into them more, but I'm busy too, unfortunately.
We need someone to adopt the feature, support and extend it, and gain expert-level knowledge of it, otherwise we will end up breaking hygiene and syntax details left and right after stabilization.

alexreg
alexreg commented on Jan 30, 2018
alexreg
on Jan 30, 2018
Contributor
@petrochenkov Ah, fair enough. I mean, that's a shame, but it makes sense at least. I guess this is an open call to anyone who might be able to take ownership of this feature, since no one comes to mind? I could still have a go at a little sub-feature, but I'm certainly in no position to take ownership of this.

petrochenkov
petrochenkov commented on Jan 31, 2018
petrochenkov
on Jan 31, 2018 · edited by petrochenkov
Contributor
@alexreg

I could still have a go at a little sub-feature

Yes, please do!
Hygiene opt-out is one of the primary missing parts and implementing it would be useful in any case.

IIRC, two questions will need to be decided on during implementation:

Syntax for "unhygienic" identifiers (@jseyfried tentatively suggested #ident in Initial implementation of declarative macros 2.0 #40847)
What exactly hygienic context the identifier introduced with #ident in a macro m will have - context of m's invocation? context after expanding all macros ("no hygiene")? something else?
alexreg
alexreg commented on Jan 31, 2018
alexreg
on Jan 31, 2018
Contributor
Syntax for "unhygienic" identifiers (@jseyfried tentatively suggested #ident in #40847)

Yeah, this was the plan. :-)

What exactly hygienic context the identifier introduced with #ident in a macro m will have - context of m's invocation? context after expanding all macros ("no hygiene")? something else?

What's your inclination? I'm leaning towards the context of m's invocation, but curious to hear your thoughts...

petrochenkov
petrochenkov commented on Jan 31, 2018
petrochenkov
on Jan 31, 2018
Contributor
@alexreg

I'm leaning towards the context of m's invocation

I think this is what should be implemented first, just because this is a more conservative alternative.

On the other hand, it makes writing internal helper macros harder, e.g.

macro m_helper() {
    struct #S;
}

macro m() {
    m_helper!(); // `S` has this context
    let s = S; // OK
}

fn main() {
    m!(); // `S` is not accessible here
    let s = S; // ERROR
}
alexreg
alexreg commented on Jan 31, 2018
alexreg
on Jan 31, 2018
Contributor
@petrochenkov Yeah, good point. I wonder if adding syntax like m_helper!#() (reuse the current context for the invocation) would help with that, or if there's a more elegant way that covers all use cases without too much pain?

pierzchalski
pierzchalski commented on Jan 31, 2018
pierzchalski
on Jan 31, 2018
Contributor
At that point you might start providing utility macros like lift!(m_helper!(...)) (move all tokens generated by m_helper!(...) up one context, essentially pretending foo! was called by main in the example above):

macro m_helper() {
    struct #S;
    struct T;
}

macro m() {
    lift!(m_helper!()); // Sets caller context of `m_helper!` to caller context of `m!`.
    let s = S; // Not OK: `S` is in callers context.
    let t = T; // Not OK: `T` is in `m_helper!` context.
}

fn main() {
    m!();
    let s = S; // OK: `S` is in `main` context.
}
As a bonus, I think this would be possible to implement with some minor extensions to the proc macro API (mostly getting and setting the parent of an arbitrary scope/span, rather than only having access to the def and call site scopes).

alexreg
alexreg commented on Feb 3, 2018
alexreg
on Feb 3, 2018
Contributor
@pierzchalski Yeah, that's not a bad idea at all. Thoughts, @jseyfried / @petrochenkov?

pierzchalski
pierzchalski commented on Feb 3, 2018
pierzchalski
on Feb 3, 2018
Contributor
@alexreg Actually, I just realised this is the use-case I was looking for for call_from in the proc macro RFC I put up.

alexreg
alexreg commented on Feb 3, 2018
alexreg
on Feb 3, 2018
Contributor
@pierzchalski I'll give that RFC a read tomorrow. Anyway, I certainly won't be including this lift macro or similar into my RFC; only the basic hygiene opt-out syntax. If the lift macro could go in another crate eventually, that would be ideal.

Incidentally, you seem to have a good knowledge of macro expansion and hygiene. Could I tempt you to contribute to rust-lang/rustc-dev-guide#15? :-)

pierzchalski
pierzchalski commented on Feb 5, 2018
pierzchalski
on Feb 5, 2018
Contributor
@alexreg Unfortunately my understanding of expansion and hygiene is a bit abstract - you'll notice the reference-level explanation in the RFC was rather thin! But if the RFC is accepted and I end up implementing it then I'd definitely like to document what I discover along the way.


smangelsdorf
mentioned this on Feb 18, 2018
Path extractors don't check path strings have correct names gotham-rs/gotham#145
dtolnay
dtolnay commented on Apr 3, 2018
dtolnay
on Apr 3, 2018
Member
I filed #49629 to consider dropping support for #[$m:meta] in favor of #[$($meta:tt)*] now that attributes are allowed to contain an arbitrary token stream.


SimonSapin
mentioned this on Apr 4, 2018
Tracking issue for "macro naming and modularisation" (RFC #1561) #35896
dead-claudia
dead-claudia commented on Aug 30, 2018
dead-claudia
on Aug 30, 2018
Stylistic question: why is it macro foo() and not macro foo!(), like the way it's called?

mark-i-m
mark-i-m commented on Aug 30, 2018
mark-i-m
on Aug 30, 2018
Contributor
I believe the question mark isn't considered part of the name.

dead-claudia
dead-claudia commented on Aug 31, 2018
dead-claudia
on Aug 31, 2018
@mark-i-m You mean exclamation point, not question mark, right?

Also, in the docs, it usually refers to macros including the exclamation point as if it were part of the name, such as println! or vec!. As a concrete example, here's one page in the book that uses it exclusively.

mark-i-m
mark-i-m commented on Aug 31, 2018
mark-i-m
on Aug 31, 2018
Contributor
Oh, yes, I meant exclamation mark. I might be wrong, but I believe the compiler itself doesn't count the exclamation mark as party of the ident.

dead-claudia
dead-claudia commented on Aug 31, 2018
dead-claudia
on Aug 31, 2018
@mark-i-m I would expect the compiler not to, but I'm speaking of the language itself, not the implementation. I'm suggesting matching what people think, not what computers process.

mark-i-m
mark-i-m commented on Aug 31, 2018
mark-i-m
on Aug 31, 2018
Contributor
That said, I don't see any reason why we cannot do something different from the internal representation.

If we did want to make ! part of the name, we would also want to do it in macro imports in the 2018 edition.

dead-claudia
dead-claudia commented on Aug 31, 2018
dead-claudia
on Aug 31, 2018
@mark-i-m If you change how you do it in imports, you could even integrate them into use, to avoid the need to deal with separate syntax altogether:

use mod::foo::some_macro!;
use mod::foo::{some_fn, some_macro!};
Even as recently as a couple days ago, I had to google how to import a macro from a peer module, and this would make it a million times simpler. It also wouldn't require any special attribute or whatever - it'd just work.

Similarly, exporting macros could be simply pub macro foo!() or similar. Alternatively, because ! could serve as a delimiter, you could remove macro altogether and just do pub fn foo!($a: ident) { ... }, or you could reserve macro foo! for macros with multiple syntax rules (like vec!), and let fn foo! be for ones with only a single value (like how try! could be written if this RFC gets implemented.

mark-i-m
mark-i-m commented on Sep 3, 2018
mark-i-m
on Sep 3, 2018
Contributor
@isiahmeadows you can do use foo::bar::some_macro in the 2018 edition. It is part of the module/macro system changes being stabilized.

Long term IIUC, the plan is to get rid of the attributes altogether (currently, you still need them to export macros).

I strongly prefer not to conflate functions and macros. They are very different.

dhardy
dhardy commented on Sep 6, 2018
dhardy
on Sep 6, 2018
Contributor
use mod::foo::{some_fn, some_macro!};

The ! suffix is not technically necessary here, but since this may be the only thing pointing out that it is indeed a macro, seems like a good idea (if feasible and not too late — probably is too late).

But of course if foo and foo! are two distinct identifiers, they should not both be allowed to exist in the same scope.

mark-i-m
mark-i-m commented on Sep 6, 2018
mark-i-m
on Sep 6, 2018
Contributor
Macros live in another namespace. You can have a macro foo and a function foo and a type foo all coexisting.

Ekleog
Ekleog commented on Sep 9, 2018
Ekleog
on Sep 9, 2018
This makes me wonder. If I have a crate mycrate defining function foo and procedural macro foo, then if I do use mycrate::foo, which one is imported into scope? both? (I'm starting from the idea that some day in the hopefully near future the -derive crates and the not--derive crates will be merge-able)

Adding the ! as something to remove the ambiguity would likely help (in addition to being more explicit about what happens, which is always a good thing).

Assuming it's still time to do it, though… otherwise, add the ! suffix as a possibility and a warning to push using it?

Nemo157
Nemo157 commented on Sep 9, 2018
Nemo157
on Sep 9, 2018
Contributor
@Ekleog yes, both (along with potentially a module foo and type foo as well).

SimonSapin
SimonSapin commented on Sep 10, 2018
SimonSapin
on Sep 10, 2018 · edited by SimonSapin
Contributor
If I have a crate mycrate defining function foo and procedural macro foo

You cannot do that today:

error: `proc-macro` crate types cannot export any items other than functions tagged with `#[proc_macro_derive]` currently
 --> src/lib.rs:1:1
  |
1 | pub fn some_public_function() {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
However in an hypothetical future Rust where that restriction is lifted somehow, or if a non-proc-macro crate re-exports a proc macro and also a function of the same name, then yes use will import both.


Centril
mentioned this on Sep 23, 2018
Strange behavior decl_macro + try_blocks + type_ascription #54455

Centril
mentioned this on Oct 9, 2018
Allow group capture, choice and optional arguments rfcs#2170
Extend macro_rules! to make macros with pre-bracket items rfcs#2334
Macros Derive PlopAhead and PlopBehind rfcs#2390
Allow else and else if blocks to attach to the end of macro expantions rfcs#2335
Nokel81
Nokel81 commented on Oct 12, 2018
Nokel81
on Oct 12, 2018
Contributor
Should small proposals for Macros2.0 be brought up here or in a separate issue?


johnthagen
mentioned this on Oct 22, 2018
Explain why macros cannot be imported from the same crate edition-guide#115

ia0
mentioned this on May 29, 2019
MBE: more checks at definition time #61053

GrayJack
mentioned this on Jun 26, 2019
Parse new macro declarative expression tree-sitter/tree-sitter-rust#45

JordiOfRivia
mentioned this on Jul 12, 2019
]: The attribute get is currently unknown to the compiler and may have meaning added to it in the future rwf2/Rocket#1050
GrayJack
GrayJack commented on Jul 13, 2019
GrayJack
on Jul 13, 2019
Contributor
What the status of this?

petrochenkov
petrochenkov commented on Jul 13, 2019
petrochenkov
on Jul 13, 2019
Contributor
What the status of this?

Bugs in macros 2.0 are occasionally fixed, but the feature is not on the 2019 roadmap.


kngwyu
mentioned this on Jul 28, 2019
Support decl-macro racer-rust/racer#1059

dtolnay
mentioned this on Aug 3, 2019
Tracking issue for syn 1.0 dtolnay/syn#687

oxalica
mentioned this on Sep 9, 2019
Make macro scope a real name scope and fix some details rust-analyzer#1795

bors
added a commit that references this issue on Sep 10, 2019
Merge #1795

Verified
c3d96f6

songroom2016
mentioned this on Sep 14, 2019
error[E0658]: macro is experimental rwf2/Rocket#1134

petrochenkov
mentioned this on Jan 31, 2020
Span hygiene data should be serialized to crate metadata #68686
johnw42
johnw42 commented on Feb 15, 2020
johnw42
on Feb 15, 2020 · edited by johnw42
I'd like to clarify the rules about visibility and propose a mechanism for breaking hygiene in a controlled way.

I believe visibility should be resolved relative to where the macro is defined. For example, imagine a module like this:

fn foo<T>(arg: T) {...}
pub macro call_foo($arg:expr) => {
    foo($arg)
}
With the current rules, expanding call_foo! in a different module doesn't work because foo isn't public. It should be should be fine because foo is visible where call_foo! is defined.

As for breaking hygiene, I think some additional flexibility would be very useful, and I think some solution needs to be introduced at the same time as the new hygiene rules, because otherwise it becomes impossible to break hygiene in ways that macros can depend on now (e.g. when defining a new type). I propose a built-in macro whose argument is an identifier that will appear verbatim in the expansion of a macro where it's used. Let's call it verbatim! for now (although I don't think that's a great name for it). It could, for example, be used to define an "anaphoric" map whose argument is an implied closure with a fixed argument name:

pub macro map_it($iter:expr, $body:expr) {
    $iter.map(|verbatim!(it)| $body)
}
It could also be used to translate an old macro that defines a type in the current module:

// before:
macro_rules! define_foo {
    () => {
        struct Foo {...}
    }
}

// after:
pub macro define_foo() => {
    struct verbatim!(Foo) {...}
}
Special names like self and Self, which are treated as identifiers for the purpose of macro expansion, should be implicitly verbatim. With the current rules, this doesn't work because $body can't refer to self:

macro_rules! paranoid_method {
    ($name:ident, $body:expr) => {
        pub fn $name(&mut self) {
            self.verify_preconditions();
            $body;
            self.verify_postconditions();
        }
    }
}
Currently the only workaround is to pass self as an additional argument to the macro, which is pretty silly since passing any other identifier would produce a syntactically invalid expansion.

One last thing on my wishlist is to extend verbatim! to support multiple arguments, which are concatenated together to produce a new verbatim identifier. This could be used to do something like define a pair of related methods:

/// Defines a pair of conversion methods, `as_$name` and
/// `into_$name`, where `into_$name` consumes `self` and 
/// `as_$name` clones `self`.
macro conversions($name:ident, $ty:ty, $body:expr) => {
    pub fn verbatim!(into_, $name)(self) -> $ty { $body }
    pub fn verbatim!(as_, $name)(&self) -> $ty {
        self.clone().verbatim!(into_, $name)()
    }
}
spearman
spearman commented on Feb 15, 2020
spearman
on Feb 15, 2020
I'd like to clarify the rules about visibility and propose a mechanism for breaking hygiene in a controlled way.

I believe visibility should be resolved relative to where the macro is defined. For example, imagine a module like this:

fn foo<T>(arg: T) {...}
pub macro call_foo($arg:expr) => {
    foo($arg)
}
With the current rules, expanding call_foo! in a different module doesn't work because foo isn't public. It should be should be fine because foo is visible where call_foo! is defined.

Could you give an example of this? This seems to work: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=a3e26640f0fb170b9a5a620250155f1c

You might need an absolute path $crate::path::to::foo($arg), but I'm not sure if this applies to decl macros or only macro_rules.

jjpe
jjpe commented on Feb 15, 2020
jjpe
on Feb 15, 2020 · edited by jjpe
It could, for example, be used to define an "anaphoric" map

I know it's just meant as an example, but anaphoric macros were a bad idea in Common Lisp, and I believe they're a bad idea here. The issue with them is that they don't scale: imagine that you end up in a situation where you have a nested usage of map_it. What does it refer to?
Of course here it's easy enough to figure out in this contrived example, but at scale it causes ambiguity in the mind of the programmer.

verbatim!(it)

I would like to note that that requires macros to be expandable to identifiers. I don't believe they currently have that capacity, so that would need to be added as well.
As for the name, I believe this is one instance where another cue from Lisps might help. While they use ~ (Closure) or , (CL) for interpolation, we could similarly use a sigil for identifier interpolation (leaving the default case to be verbatim instead). I'm not sure how that would interact with identifier concatenation though.

mayabyte
mayabyte commented on Mar 4, 2020
mayabyte
on Mar 4, 2020 · edited by mayabyte
Weather or not anaphoric macros are allowed, the ability to create new identifiers from ones passed into the macro seems highly desirable. Consider the following case with macro_rules!:

macro_rules! example {
    ($($a:ident),*) => {
        $(
            // assume each $a is an identifier for a HashMap<K,V>, where each 
            // K and V can have a different type
            let side_effect = $a.remove("some_key");
            ... // do some other stuff
        ),*
        ...
        $( 
            // do some more things with each side effect created earlier, 
            // like putting them in a tuple
        ),*
    };
}
This doesn't work because side_effect has to have a fixed name, so it'd be shadowed each 'iteration'. (Achieving something like this is possible, but it's hacky and involves a lot of boilerplate).

Being able to do something like one of these:

let side_effect_${a} = ...
let verbatim!(side_effect_, $a) = ...
would be really nice.

eddyb
eddyb commented on Mar 12, 2020
eddyb
on Mar 12, 2020
Member
Doesn't side_effect_$ already successfully tokenize as side_effect_ $?

petrochenkov
petrochenkov commented on Apr 17, 2020
petrochenkov
on Apr 17, 2020
Contributor
Some status update (copypasted from Zulip https://rust-lang.zulipchat.com/#narrow/stream/213817-t-lang/topic/decl.20macro.20syntax/near/194354354):

macro items have multiple components to figure out before stabilizing, syntactic and semantic:

Span::def_site, pretty far away, requires implementing cross-crate hygiene at least, and then formalizing stuff in hygiene.rs more carefully, in application to type-relative paths in particular.
Syntax of the macro's left side (macro "parameters"), requires major design work to figure out future-compatibility (currently done with FIRST and FOLLOW sets) and figuring out how to match or not match all arms at the same time.
Syntax of the macro's right side (macro body), requires a syntax for hygiene opt-out perhaps, maybe simplifying the use of repetitions (MBE: more checks at definition time #61053 (comment)), but otherwise seems ok, it's just an arbitrary token stream.
Surface syntax of the macro. The last year I almost wrote and RFC to set the top-level syntax macro single_arm() {} + macro multiple_arms { (lhs1) {rhs1} (lhs2) {rhs2} } in stone, but them recalled that people wanted the macros want to control what delimiters they are invoked with (e.g. restrict vec![] to only use square brackets), and that added more questions to the surface syntax, and I didn't write anything.
petrochenkov
petrochenkov commented on Apr 17, 2020
petrochenkov
on Apr 17, 2020
Contributor
One more issue is expanding fragments like $e:expr as token streams rather than AST pieces (this is also mentioned in the top comment and links to #26361), but this is equally applicable to macro_rules where it should be doable in a (almost) backward-compatible way.


marmeladema
mentioned this on May 3, 2020
Invalid lowering of macro in closure #71820

syntacticsugarglider
mentioned this on May 26, 2020
Add support for supertraits/generics/etc. noocene/protocol#4
jgarvin
jgarvin commented on Jun 27, 2020
jgarvin
on Jun 27, 2020
Do macros 2.0 address the self both is and isn't an identifier inconsistencies described here?: https://danielkeep.github.io/tlborm/book/mbe-min-non-identifier-identifiers.html

Or the confusing behavior of macro_rules invoking other macro_rules macros not behaving the same as calling directly?:
https://danielkeep.github.io/tlborm/book/mbe-min-captures-and-expansion-redux.html

petrochenkov
petrochenkov commented on Jun 27, 2020
petrochenkov
on Jun 27, 2020
Contributor
Do macros 2.0 address the self both is and isn't an identifier inconsistencies described here?

self is always an identifier, keywords are a (reserved) subset of identifiers.
I don't think macros 2.0 change anything here.

Or the confusing behavior of macro_rules invoking other macro_rules macros not behaving the same as calling directly?

This is not decided and needs design.

jgarvin
jgarvin commented on Jun 27, 2020
jgarvin
on Jun 27, 2020 · edited by jgarvin
@petrochenkov are you saying the the author is mistaken, or that rust has changed since it was written? A number of confusing examples are provided at the link.

petrochenkov
petrochenkov commented on Jun 27, 2020
petrochenkov
on Jun 27, 2020
Contributor
@jgarvin
Neither. The author is right in the sense that all the examples are correct etc., but he uses a different definition of "identifier" than the language and that's probably the source of the confusion.
(Anyway, this is pretty off-topic for this issue.)


yvt
added a commit that references this issue on Aug 11, 2020
feat(portkit): move `pp_*` macros to `crate::pptext`

32733ff
A1-Triard
A1-Triard commented on Sep 5, 2020
A1-Triard
on Sep 5, 2020
Does macros 2.0 allow to parse generic parameters definition?
With current macro_rules! the best approximation is < $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ $(,)?>, but it does not cover all possible cases.


Nugine
mentioned this on Sep 11, 2020
Eager expansion Nugine/const-str#2
mexus
mexus commented on Sep 20, 2020
mexus
on Sep 20, 2020 · edited by mexus
Does anybody happen to know if there are any plans to put "macros 2.0" into the 2021 roadmap?

mark-i-m
mark-i-m commented on Sep 20, 2020
mark-i-m
on Sep 20, 2020
Contributor
Given the ruat 2021 posts I've read so far and my vague knowledge of the state of things, it seems unlikely. There is still some design work needed and there doesn't seem to be anyone interested in pushing it over the finish line atm.


fmease
mentioned this on Sep 23, 2020
Unresolved import for declarative macros 2.0 rust-analyzer#6059

janpetschexain
mentioned this on Nov 6, 2020
XN-1163 replace macro use xaynetwork/xaynet#592
IQBigBang
IQBigBang commented on Feb 25, 2021
IQBigBang
on Feb 25, 2021 · edited by IQBigBang
This is maybe out of scope of this discussion, but now that Rust has (or will soon have) constant functions and miri enabling it to run rust code at compile-time, wouldn't it be possible to support in-crate procedural macros which don't work essentialy as a compiler plugin? This seems like something I personally would expect a new macro system to enable.


github-actions
added a commit that references this issue on Mar 29, 2021
ices/82865.rs: fixed with errors

0df697a

JohnTitor
added a commit that references this issue on Mar 29, 2021
ices/82865.rs: fixed with errors (#704)

Verified
60f106c

cherryblossom000
mentioned this on Apr 22, 2021
single_component_path_imports: ignore pub(crate) use some_macro; rust-clippy#7120
spearman
spearman commented on Aug 17, 2021
spearman
on Aug 17, 2021
Private decl macro shows up in generated docs (rustc 1.56.0-nightly (0035d9dce 2021-08-16))

// src/lib.rs
#![feature(decl_macro)]
#[derive(Clone, PartialEq)]
pub struct Foo;
macro macro1 ($foo:ty) {
  impl Eq for $foo { }
}
macro_rules! macro2 {
  ($foo:ty) => {
    impl Copy for $foo { }
  }
}
macro1!(Foo);
macro2!(Foo);
macro1 is shown in generated cargo doc, macro2 is not.


zjijz
mentioned this on Nov 1, 2021
Inherent impl hygiene difference between declarative macros and macro_rules! #90466

nikomatsakis
mentioned this on Dec 14, 2021
New declarative macros, functions and fields not being recognized #91249

joshtriplett
added 
S-tracking-design-concerns
Status: There are blocking design concerns.
 on Jan 26, 2022
eddyb
eddyb commented on Feb 22, 2022
eddyb
on Feb 22, 2022
Member
Before being reminded that Span::def_site() on the proc macro side is what hygiene is tracked under, I came up with this silly little snippet:

macro_rules! foo_impl {
    // ...
}

macro_rules! define_foo_with_hygiene {
    ($hygienic_crate:ident, $hygienic_foo_impl:ident) => {
        #[macro_export]
        macro_rules! foo {
            ($($inputs:tt)*) => {
                $hygienic_foo_impl!($hygienic_crate; $($inputs)*);
            };
        }
    };
}

macro define_foo() {
    define_foo_with_hygiene!($crate, foo_impl);
}
define_foo!();
The idea there is that by using $crate from inside a macro (as opposed to macro_rules!), the "hygiene 2.0" aspect was captured, and private parts of $crate can now be accessed through it (in a kind of objcap-y way, heh).
(Similarly, passing foo_impl from the macro would allow foo_impl! to remain unexported)

And you would only need to stabilize "macro" IDENT "(" ")" "{" TOKEN_TREE* "}" syntax, completely obscuring any further changes to MBE input LHS pattern syntax, parsing/choice, or RHS expansion.

But if a proc macro can use Span::def_site() to achieve the same thing, this trick is probably less useful.


c410-f3r
mentioned this on Apr 7, 2022
Kickstart the inner usage of macro_metavar_expr #95761

github-actions
mentioned this on Jun 4, 2022
utils: BLOCKED) make this macro pub(crate) when supported. recmo/uint#104

SpriteOvO
mentioned this on Jul 1, 2022
Pattern-based formatter SpriteOvO/spdlog-rs#12

RalfJung
added 
F-decl_macro
`#![feature(decl_macro)]`
 on Aug 9, 2022
danii
danii commented on Oct 4, 2022
danii
on Oct 4, 2022
Contributor
Should metavariables be prevented from being named self or as keywords? I feel like metavariable keywords could be used for future expansions to macros, like associated macros with $self.

danielrab
danielrab commented on Oct 8, 2022
danielrab
on Oct 8, 2022
yeah, I agree, no keywords should probably be valid names for metavariables, in case we want to make any of them special somehow

eap314
eap314 commented on Nov 12, 2022
eap314
on Nov 12, 2022 · edited by eap314
These suggestions might be out of scope for this discussion, but it beats the RFC process, so

hygiene could be (mostly) achieved by having macros (mangle) prefix functions with h_ and normal functions be (mangle) prefixed uh_
If you don't want hygiene, you could use a keyword like unhygienic to prefix with uh_
unhygienic macro count {}
macro count2 {}

count!();
unhygienic { count2!(); }

// hygiene bending
macro count3 {
() => { unhygienic { let x = 4 } }
}
To bring up the out of scope part,

Proc macros can be preprocessed, removing the need for a separate crate
You can proc use a crate to bring it into scope during preprocessing
A1-Triard
A1-Triard commented on Nov 14, 2022
A1-Triard
on Nov 14, 2022
* hygiene could be (mostly) achieved by having macros (mangle) prefix functions with h_ and normal functions be (mangle) 
As far as I understand it would not work for #[no_mangle] functions. If this is correct, then it means the suggestion just does not work.


lukaslueg
mentioned this on Mar 26, 2023
Support new macro syntax lukaslueg/macro_railroad_ext#17
lygstate
lygstate commented on Aug 2, 2023
lygstate
on Aug 2, 2023
Contributor
When this will be stablized :)

clarfonthey
clarfonthey commented on Aug 3, 2023
clarfonthey
on Aug 3, 2023
Contributor
Presumably it will be stabilised after all of the requisite tasks in the issue description are completed.

cybersoulK
cybersoulK commented on Aug 9, 2023
cybersoulK
on Aug 9, 2023 · edited by cybersoulK
I might not fully grasp the nuances of macros 2.0, but these are the benefits I anticipate:

1 - I should be able to use crates, types, and functions that I define in my library directly without needing $crate (suggesting deprecation of $crate).
2 - Every item created within the macro should expand; after all, that's the core purpose of macros.

This reasoning is sound:

If you desire certain types or functions to remain private, simply define them outside the macro where they remain accessible to the macro.
If an item is dynamic and changes based on token input, it still needs to be expanded into the scope. Given that it's a distinct type, it shouldn't just disappear.
The types or variables introduced during expansion shouldn't conflict with tokens, even if their names are identical. Think of library-defined code as being in red, and tokens in blue; they are distinct entities.
The user decides where the macro expands. They can enclose it using a mod { macro!() } or restrict its scope using a block { macro!() }.
At its core, macros operate on this principle: Library code + User code (Tokens) = Output Expansion.
I'm concerned that we might be overcomplicating the concept of hygiene.


cybersoulK
mentioned this on Aug 9, 2023
Tracking Issue for RFC 3086: macro metavariable expressions #83527
cybersoulK
cybersoulK commented on Aug 9, 2023
cybersoulK
on Aug 9, 2023 · edited by cybersoulK
From my previous points, I believe the confusion we're facing stems from how to handle situations where Library code intertwines with User code.
It's naturally expected for them to merge within macros, since, by definition, a macro represents the library code's blueprint being replicated into the user's domain. Perhaps there's a need for enhanced tools to manage instances when two textual terms conflict, rather than sidestepping the issue and introducing myriad limitations to macros.

yea, i don't see why this is hard to do, if the macro has a chance of word collision.

mod macro_mod {
macro!();
}

pub use macro_mod::Type as NewNamedType;
i would not be against if the macro 2.0 used mod by default when expanding in open scopes, such as

use crate::macro;
macro!();
pub use macro::Type as NewNamedType;

use crate::macro as macro_renamed;
macro_renamed!();
pub use macro_renamed::*;

//automatically created macro mod and macro_renamed mod.
But then there must be a differentiation of macros that contain function calls and variable assignments, and should be expanded directly, such as:

match ... {
   A(name) => macro!(name),
}
jjpe
jjpe commented on Aug 9, 2023
jjpe
on Aug 9, 2023 · edited by jjpe
@cybersoulK do everyone here a favor please, and don't multipost. Everyone who is subscribed to this gets a separate email about each post, and the signal:noise ratio is dropping real fast ATM.

To be clear: posting is fine, but maybe don't drive people up the wall with constant useless notifications.

dengyunsheng250
dengyunsheng250 commented on Aug 22, 2023
dengyunsheng250
on Aug 22, 2023
Has there been any progress on Macros 2.0 lately?

jhpratt
jhpratt commented on Aug 22, 2023
jhpratt
on Aug 22, 2023
Member
No, but there will hopefully be a general increase in knowledge in what the issues are, as the proposal for a macros working group has been accepted. The WG will be created shortly!


photino
mentioned this on Sep 4, 2023
Run the framework in the stable Rust zino-rs/zino#51
tgross35
tgross35 commented on Sep 14, 2023
tgross35
on Sep 14, 2023 · edited by tgross35
Contributor
It could be interesting to allow a way to reverse parsing to match from right to left. This can be faked using an accumulator and recursively popping tokens until you get a match on a single one, but this is a pretty messy pattern. A more simple way to create a reverse TT muncher would make it much easier to apply precedence to contents, such as when evaluating math.

// Works
macro first {
    ($first:tt $($rest:tt)*) => { println!("first item is {}", $first) };
}

// Fails to compile (multiple parsing options)
macro last {
    #[reverse] // maybe this could make it work
    ($($rest:tt)* $last:tt) => { println!("last item is {}", $last) }
}
Edit: an alternative to this would just be adjusting the rules for multiple parsing such that it tries right to left if it gets stuck. I think this would accomplish the same thing without needing an annotation.

clarfonthey
clarfonthey commented on Sep 15, 2023
clarfonthey
on Sep 15, 2023
Contributor
The main issue with RTL parsing is that, at least for now, parsing doesn't actually store the tokens being returned in a way that lets you iterate backwards. You could allow this and do it without allocating if you permit parsing the tokens twice (once to find the end of the macro invocation, the second time to do the actual parsing), but it's still a performance penalty to allow parsing from the other direction.

Of course, the usual way, recursively expanding an intermediate macro in reverse order, is probably slower than offering a dedicated method that allows this. If this were done, it would be nice to also have proc_macro::TokenStream implement DoubleEndedIterator to reflect that, and allow proc macros to also take advantage of that.

Ygg01
Ygg01 commented on Sep 15, 2023
Ygg01
on Sep 15, 2023 · edited by Ygg01
Here is to hoping two big issues are getting solved (hygiene bending and nesting macros).

I tried solving macro hygiene but one pre-requisite was learning how other languages do it. Learning Racket is where I lost my motivation.

petrochenkov
petrochenkov commented on Sep 20, 2023
petrochenkov
on Sep 20, 2023
Contributor
Here is to hoping two big issues are getting solved (hygiene bending and nesting macros).

I tried solving macro hygiene but one pre-requisite was learning how other languages do it. Learning Racket is where I lost my motivation.

The issue about hygiene is "Tracking issue for Span::def_site()", this issue is blocked on it.
In general, declarative macros are a syntactic sugar for procedural macros, so the "procedural macros 2.0" need to be implemented first.

Racket needs to be learned just enough to understand https://users.cs.utah.edu/plt/scope-sets/ in detail, to be able to possibly tweak it to match Rust realities.


fmease
mentioned this on Oct 14, 2023
Rename macro_rules! to macro! rfcs#293

egkoppel
mentioned this on Mar 10, 2024
List of nightly features required popcorn-2/popcorn-2#74

tgross35
mentioned this on May 11, 2024
Macro literal fragment specifiers should forward to exact tokens #124989
lolbinarycat
lolbinarycat commented on May 11, 2024
lolbinarycat
on May 11, 2024
Contributor
one usecase of macros where rust does worse than C is local unhyginic macros used in the definition of a single array or function.

some way to bind specific variables to to those from the outer scope, or to disable macro hygine for certain identifies, would be appreciated.

perhaps i need to call a function 10 times, and each time i need to pass 7 different arguments, 6 of which are always the same local variables, and only the 7th changes. C could easily manage this with a define and undef, but in rust, you would have to pass all the local variables to each macro call, which wouldn't actually simplify the code at all.

felix91gr
felix91gr commented on May 11, 2024
felix91gr
on May 11, 2024
Contributor
which wouldn't actually simplify the code at all.

That is true for this version of "simplify":

Simpler code: shorter code.

But it is not true for the following version of "simplify":

Simpler code: code that is easier to understand and maintain.

I don't think locally disabling hygiene helps at all, if what we seek is simpler code in the second sense.

ifsheldon
ifsheldon commented on May 11, 2024
ifsheldon
on May 11, 2024
one usecase of macros where rust does worse than C is local unhyginic macros used in the definition of a single array or function.

I think "worse" here is generally better. I've read a pretty bad C codebase once, which is full of unhygine macros that means a lot of sloppy auto copy-and-paste. IDEs could not even help me to analyze the code. I really appreciate Rust didn't just choose the simple way to do macros.

perhaps i need to call a function 10 times, and each time i need to pass 7 different arguments, 6 of which are always the same local variables, and only the 7th changes.

Make a closure please.

tmccombs
tmccombs commented on May 11, 2024
tmccombs
on May 11, 2024
Contributor
Make a closure please.

A closure can't do everything a macro can. For example it can't return from the parent function.

mayabyte
mayabyte commented on May 12, 2024
mayabyte
on May 12, 2024
perhaps i need to call a function 10 times, and each time i need to pass 7 different arguments, 6 of which are always the same local variables, and only the 7th changes.

I think what you're describing is partial application which is something that can already be achieved with existing crates. Combining this with the Try operator could achieve the early-return functionality described above as well.

lolbinarycat
lolbinarycat commented on May 13, 2024
lolbinarycat
on May 13, 2024
Contributor
it turns out the existing macro_rules! macros can already access identifiers from their parent scope, so this type of local macro actually already works, for when you need to do something that partial application can't do.

oli-obk
oli-obk commented on May 20, 2024
oli-obk
on May 20, 2024
Contributor
Tracking issues are not intended as places for discussions. This issue already has over 100 comments and lots of subscribers. Please open separate issues and link them, instead of having the discussion here.


jflatow
mentioned this on Jun 7, 2024
Encountered incremental compilation error with find_field(a5f984281f1e0bb0-ee8ea99685b18732) #125678

ojeda
mentioned this on Aug 5, 2024
Rust wanted features Rust-for-Linux/linux#354

kennytm
mentioned this on Sep 22, 2024
Declarative macro_rules! attribute macros rfcs#3697
safinaskar
safinaskar commented on Oct 13, 2024
safinaskar
on Oct 13, 2024 · Hidden as off-topic
tgross35
tgross35 commented on Oct 13, 2024
tgross35
on Oct 13, 2024 · Hidden as off-topic
safinaskar
safinaskar commented on Oct 13, 2024
safinaskar
on Oct 13, 2024 · Hidden as off-topic

MajoraSans
mentioned this on Oct 21, 2024
Tracking issue for experimental features in 1.49 implemented in gccrs Rust-GCC/gccrs#3210

GrigorenkoPV
mentioned this on Oct 22, 2024
assert_eq! is not 100% hygienic #131446
safinaskar
safinaskar commented on Feb 7
safinaskar
on Feb 7 · Hidden as resolved

rustbot
added 
A-hygiene
Area: Macro hygiene
 on Feb 7
safinaskar
safinaskar commented on Feb 7
safinaskar
on Feb 7 · Hidden as resolved

matthewjasper
added 
A-decl-macros-2-0
Area: Declarative macros 2.0 (#39412)
 
WG-macros
Working group: Macros
 on Feb 7

ratmice
mentioned this on Apr 1
docs.tockos.org macro documentation is pulled to top-level crate, making discoverability hard for things semantically part of a module tock/tock#4383

sdww0
mentioned this on Apr 15
Implement IoPortAllocator asterinas/asterinas#1518

Manishearth
mentioned this on May 22
Add support for custom bakes to databake unicode-org/icu4x#6576

lcian
mentioned this on Jun 5
feat(logs): add macro-based API getsentry/sentry-rust#827

GotenJBZ
mentioned this on Jun 21
Crate-Level Support for #[safe_math] Attribute GotenJBZ/safe-math-rs#49

fee1-dead
mentioned this on Jun 27
Sort-of RFC: add min!, max! macros once namespacing lands #53501

brvtalcake
mentioned this on Jul 16
Add support for declarative macros v2 Daniel-Aaron-Bloom/eager2#16

fmease
marked Generic (lifetime, type, const) parameters are unhygenic #145023 as a duplicate of this issue on Aug 7

fmease
mentioned this on Aug 7
Generic (lifetime, type, const) parameters are unhygenic #145023

andriyDev
mentioned this on Sep 17
rust-analyzer does not suggest macro matchers for autocomplete. google/googletest-rust#701

nik-rev
mentioned this on Oct 2
add explicit marker to export macros nik-rev/derive-aliases#1
max-kamps
max-kamps commented 28 days ago
max-kamps
28 days ago
Not sure if this is known (or even an intentional limitation?), but for some reason macros can't refer to type parameters or const parameters at their definition site:

https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=fee4172d5f73a52e62614e77a3546d72

#![feature(decl_macro)]
use std::fmt::Debug;


fn test<TypeParam: Default + Debug, const CONST_PARAM: usize>() {
    #[derive(Default, Debug)]
    struct LocalType(usize);
    
    macro can_refer_to_local_type() { dbg!(LocalType::default()) }
    can_refer_to_local_type!();
    
    macro cant_refer_to_type_param() { dbg!(TypeParam::default()) }
    cant_refer_to_type_param!();
    // Error: failed to resolve: use of undeclared type `TypeParam`
    
    
    const LOCAL_CONST: usize = 812;

    macro can_refer_to_local_const() { dbg!(LOCAL_CONST) }
    can_refer_to_local_const!();
    
    macro cant_refer_to_const_param() { dbg!(CONST_PARAM) }
    cant_refer_to_const_param!();
    // Error: cannot find value `CONST_PARAM` in this scope
    // you might have meant to refer to this const parameter
}

peter-lyons-kehl
mentioned this 2 weeks ago
Forbidding lints doesn't really work in macros #110613
platonvin
Add a comment
new Comment
Markdown input: edit mode selected.
Write
Preview
Use Markdown to format your comment
Remember, contributions to this repository should follow its contributing guidelines, security policy and code of conduct.
Metadata
Assignees
No one assigned
Labels
A-decl-macros-2-0
Area: Declarative macros 2.0 (#39412)
A-hygiene
Area: Macro hygiene
A-macros
Area: All kinds of macros (custom derive, macro_rules!, proc macros, ..)
B-RFC-approved
Blocker: Approved by a merged RFC but not yet implemented.
B-unstable
Blocker: Implemented in the nightly compiler and unstable.
C-tracking-issue
Category: An issue tracking the progress of sth. like the implementation of an RFC
F-decl_macro
`#![feature(decl_macro)]`
S-tracking-design-concerns
Status: There are blocking design concerns.
T-lang
Relevant to the language team
WG-macros
Working group: Macros
Type
No type
Projects
No projects
Milestone
No milestone
Relationships
None yet
Development
No branches or pull requests
NotificationsCustomize
You're not receiving notifications from this thread.

Participants
@alexreg
@eddyb
@Nemo157
@jgarvin
@lygstate
Issue actions
Footer
© 2025 GitHub, Inc.
Footer navigation
Terms
Privacy
Security
Status
Community
Docs
Contact
Manage cookies
Do not share my personal information
