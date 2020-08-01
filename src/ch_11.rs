/// Defining traits using the Self type.
use std::fmt::Debug;

pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}

struct CherryTree {}

impl Spliceable for CherryTree {
    fn splice(&self, other: &Self) -> Self {
        // ignore the actual logic for "splice".
        CherryTree {}
    }
}

struct Mammoth {}

impl Spliceable for Mammoth {
    fn splice(&self, other: &Self) -> Self {
        // This has to be a Mammoth instance. Ie: we can't return a CherryTree instance here, thus
        // making some kind of CherryTree-Mammoth hybrid.

        Mammoth {}
        // CherryTree{}
    }
}

// Given the limitations of having to return `Self` in our `Spliceable` trait, let's try this again,
// but having our new trait return a trait object instead.

pub trait MegaSpliceable {
    fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable>;
}

impl MegaSpliceable for Mammoth {
    fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable> {
        Box::new(CherryTree {}) // now we can mix and match them!
                                // Box::new(Mammoth {}) // this also works!
    }
}

impl MegaSpliceable for CherryTree {
    fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable> {
        Box::new(Mammoth {}) // now we can mix and match them!
    }
}

/// fully qualified method calls

fn test_fqmc() {
    let line = "asdf asdf asdf";
    let words: Vec<String> = line
        .split_whitespace()
        //
        // There are multiple ways to specific the method call that will go inside the map
        // combinator:
        //
        // using the fqmc:
        // .map(<str as ToString>::to_string)
        //
        // off the trait:
        // .map(ToString::to_string)
        //
        // off the struct:
        .map(str::to_string)
        .collect();
}

/// Next, we explore 3 ways traits can describe relationships between types.

/// 1. Traits with associated types

fn dump<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug, // note how we're using Debug as a trait bound on the associated Item type
{
    for (index, value) in iter.enumerate() {
        // The Debug trait on the Iterator::Item type is needed for this println:
        println!("index: {}, value: {:?}", index, value);
    }
}

/// Let's define a trait with an associated type. This allows use to search a *string slice* for a
/// chosen *type*, and return the *match* as another chosen type.

trait Pattern {
    type Match;

    fn search(&self, string: &str) -> Option<Self::Match>;
}

/// You can search a string for a particular *character*, and return the *match* as a usize.

impl Pattern for char {
    type Match = usize;

    fn search(&self, string: &str) -> Option<Self::Match> {
        string
            .char_indices()
            .find(|(_, c)| c == self)
            .map(|(pos, _)| pos)
    }
}

// associated types are perfect for cases where each implementation has *one* specific related type:
// each type of Task productes a particular type of Output; each type of Pattern looks for a
// particular type of Match. However, some relationships among types are not like this...

/// 2. generic traits
/// (plus operator overloading)

/// 3. Buddy traits
///

pub trait Rng {
    fn next_32(&mut self) -> u32;
}

/// This is a buddy trait is called Rand, which takes a reference with the Rng trait bound.
///
/// (and it can only be implemented on a type that already
/// implements Sized)

pub trait Rand: Sized {

    /// Returns a random value of the implementor. Depends on accepting a type with trait bound Rng,
    /// which it likely uses for determining the random value.

    fn rand<R: Rng>(rng: &mut R) -> Self;
}

// let's see it in action:

struct SomeRng {}
impl Rng for SomeRng {
    fn next_32(&mut self) -> u32 {
        99 // dummy value
    }
}

impl Rand for bool {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        rng.next_32() % 2 == 0
    }
}
