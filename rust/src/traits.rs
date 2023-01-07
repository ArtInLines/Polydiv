// //!

// TODO: Split this file into seperate modules & files
// TODO: Think of a useful API-Structure (e.g. having modules for finite and infinite structures)
// TODO: Offer a "parseable" trait for each trait here, that allows parsing a string, printing and performing operations on said string

/// The [size](https://en.wikipedia.org/wiki/Cardinality) of a set (also known as its *Cardinality*)
/// is equal to the number of elements in the set.
///
/// We also include the notion of infinite sets
/// and further split those into countably and uncountably
/// infinite sets.
///
/// To [compare](https://en.wikipedia.org/wiki/Cardinality#Comparing_sets) the sizes of two sets
/// (especially if they might contain inifinitely many elements)
/// one has to map each element in each set to
/// a unique element in the other set, respectively.
/// This process is also known as building a [bijection](https://en.wikipedia.org/wiki/Bijection).
///
/// We take the [Axiom of Choice](https://en.wikipedia.org/wiki/Axiom_of_choice) as granted,
/// and thus the [law of trichotomy](https://en.wikipedia.org/wiki/Trichotomy_(mathematics))
/// is applicable to set sizes. We can therefore say
/// that each set *A* falls into one of the following categories:
/// 1. *A* is a finite set
/// 2. *A* has the same size as the Natural Numbers (countably infinite, also known as recursively enumerable)
/// 3. *A* is strictly bigger than the Natural Numbers (uncountably infinite)
pub enum SetSize {
    Finite(u64),
    CountablyInfinite,
    UncountablyInfinite,
}

/// A [Set](https://en.wikipedia.org/wiki/Set_(mathematics)) is a mathematical model for a collection of objects.
/// A Set is not ordered and contains each element exactly once.
///
/// This Type builds an underlying Abstraction, that is used by more useful mathematical structures, like Groups or Fields.
/// All elements in a Set - as in any collection in Rust - have to be of the same type. To support several types, entail them via enums or structs.
///
/// To implement this trait, you only need to implement the `size`-method. The `contains`-method has a default implementation,
/// but should be overwritten for any Set, that doesn't cover the all possible elements of the type `T`.
#[allow(unused_variables)]
pub trait Set<T> {
    /// Returns the size of the set.
    fn size(&self) -> SetSize;

    /// Checks, whether an item is contained in this set.
    /// The default implementation returns true for all items
    /// that can be coerced into the Set-Element's type *T*.
    /// If your Set is more complicated, you have to override this method.
    fn contains(&self, item: &T) -> bool {
        true
    }
}

/// An Enumerable Set is a set, that can be iterated over. Only `Finite` or `CountablyInfinite` Sets are enumerable.
///
/// This trait implements the `Set` trait, but requires that the `IntoIterator` trait is also defined, as enumerable sets should per definition be iterable.
#[allow(unused_variables)]
pub trait EnumerableSet<T>: IntoIterator<Item = T> {
    /// Returns the size of the set.
    fn size(&self) -> SetSize {
        SetSize::CountablyInfinite
    }

    /// Checks, whether an item is contained in this set.
    /// The default implementation returns true for all items
    /// that can be coerced into the Set-Element's type *T*.
    /// If your Set is more complicated, you have to override this method.
    fn contains(&self, item: &T) -> bool {
        true
    }
}
impl<T, S: EnumerableSet<T>> Set<T> for S {
    fn size(&self) -> SetSize {
        <Self as EnumerableSet<T>>::size(&self)
    }
    fn contains(&self, item: &T) -> bool {
        <Self as EnumerableSet<T>>::contains(&self, item)
    }
}

pub trait Commutative {}
pub trait Associative {}
pub trait LeftDistributive {}
pub trait RightDistributive {}
impl<T: LeftDistributive + RightDistributive> Distributive for T {}
pub trait Distributive {}
impl<T: Distributive> LeftDistributive for T {}
impl<T: Distributive> RightDistributive for T {}

///
pub trait AlgebraicStructure: Sized {
    type Domain: Set<Self>;
    type Output;
    fn domain() -> Self::Domain;
    fn contains(&self) -> bool {
        Self::domain().contains(self)
    }
    fn size(&self) -> SetSize {
        Self::domain().size()
    }
    fn op(&self, rhs: &Self) -> Self::Output;
    fn mut_op(&mut self, rhs: &Self);
}

pub trait Magma: AlgebraicStructure<Output = Self> {}
///
pub trait SemiGroup: Magma + Associative {}

///
pub trait Monoid: SemiGroup {
    fn identity(&self) -> Self;
}
///
pub trait Group: Monoid {
    fn inverse(&self, item: &Self) -> Self;
}

pub trait NearRing: Sized
where
    <Self as NearRing>::AdditiveGroup: 'static,
    Self: Into<&'static <Self as NearRing>::AdditiveGroup>,
    Self: From<<Self as NearRing>::AdditiveGroup>,
{
    type AdditiveGroup: Group<Domain = Self>;
    type MultiplicativeGroup: SemiGroup<Domain = Self>;

    fn additive_group(&self) -> Self::AdditiveGroup;

    fn multiplicative_group(&self) -> Self::MultiplicativeGroup;

    fn additive_identity(&self) -> Self {
        self.additive_group().identity().into()
    }

    fn additive_inverse(&self, item: &Self) -> Self {
        self.additive_group().inverse(item).into()
    }

    fn add(&self, lhs: &Self, rhs: &Self) -> Self {
        Group::op(self.additive_group(), lhs, rhs)
    }

    fn mut_add(&self, lhs: &mut Self, rhs: &Self) {
        self.additive_group().mut_op(lhs, rhs)
    }

    fn subtract(&self, lhs: &Self, rhs: &Self) -> Self {
        self.add(lhs, &self.additive_inverse(rhs))
    }

    fn mut_subtract(&self, lhs: &mut Self, rhs: &Self) {
        self.mut_add(lhs, &self.additive_inverse(rhs))
    }

    fn multiply(&self, lhs: &Self, rhs: &Self) -> Self {
        self.multiplicative_group().op(lhs, rhs)
    }

    fn mut_multiply(&self, lhs: &mut Self, rhs: &Self) {
        self.multiplicative_group().mut_op(lhs, rhs)
    }
}

// ///
// pub trait Rng: NearRing
// where
//     Self::AdditiveGroup: CommutativeGroup,
// {
// }

// pub trait Ring: Rng
// where
//     Self::AdditiveGroup: CommutativeGroup,
//     Self::MultiplicativeGroup: Monoid,
// {
//     fn multiplicative_identity(&self) -> Self {
//         self.multiplicative_group().identity()
//     }
// }

// pub trait CommutativeRing: Ring
// where
//     Self::AdditiveGroup: CommutativeGroup,
//     Self::MultiplicativeGroup: Monoid + Magma + Commutative,
// {
// }

// pub trait DivisionRing: Ring
// where
//     Self::AdditiveGroup: CommutativeGroup,
//     Self::MultiplicativeGroup: Group,
// {
//     fn multiplicative_inverse(&self, item: &Self) -> Self {
//         self.multiplicative_group().inverse(item)
//     }

//     fn divide(&self, lhs: &Self, rhs: &Self) -> Self {
//         self.multiply(lhs, &self.multiplicative_inverse(rhs))
//     }

//     fn mut_divide(&self, lhs: &mut Self, rhs: &Self) {
//         self.mut_multiply(lhs, &self.multiplicative_inverse(rhs))
//     }
// }

// pub trait Field: DivisionRing
// where
//     Self::AdditiveGroup: CommutativeGroup,
//     Self::MultiplicativeGroup: CommutativeGroup,
// {
// }
