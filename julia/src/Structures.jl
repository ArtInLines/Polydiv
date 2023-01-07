module Structures

import Base.one, Base.zero, Base.+, Base.-, Base.*, Base./

export AlgebraicStructure
export Magma
export SemiGroup
export Monoid
export Group
export AlbelianGroup
export DoubleMagma
export DoubleMonoid
export NearRing
export Rng
export Ring
export DivisionRing
export Field

export op, id, inv, add_struct, mult_struct, one, zero, +, -, *, /


abstract type AlgebraicStructure{T,Output} end
# Should implement the functions:
#	op(Self, Self) -> Output
function op end

abstract type Magma{T} <: AlgebraicStructure{T,T} end
# Should implement the functions:
#	op(Self, Self) -> Self

abstract type SemiGroup{T} <: Magma{T} end
# Should implement the functions:
#	op(Self, Self) -> Self
# `op` should be associative

abstract type Monoid{T} <: SemiGroup{T} end
# Should implement the functions:
#	op(Self, Self) -> Self
#	id(Type{Self}) -> Self
# `op` should be associative
# id(T) should provide the identity element
function id(::T)::T where {T<:Monoid}
    id(T)
end

abstract type Group{T} <: Monoid{T} end
# Should implement the functions:
#	op(Self, Self) -> Self
#	id(T) -> Self
#	inv(Self) -> Self
# `op` should be associative
# id(T) should provide the identity element
# op(a, inv(a)) = id(T) for all a
function inv end

abstract type AlbelianGroup{T} <: Group{T} end
# Should implement the functions:
#	op(Self, Self) -> Self
#	id(Self) -> Self
#	id(Type{Self}) -> Self
#	inv(Self) -> Self
# `op` should be associative
# id(Self) should provide the identity element
# op(a, inv(a)) = id(Self) for all a
# `op` should be commutative


abstract type DoubleMagma{T} end
# Shold implement the functions:
#	add_struct(T) -> Magma{T}
#	mult_struct(T) -> Magma{T}
function add_struct end
function mult_struct end

abstract type DoubleMonoid{T} <: DoubleMagma{T} end
# Shold implement the functions:
#	add_struct(T) -> Monoid{T}
#	add_struct(Type{T}) -> Type{Monoid{T}}
#	mult_struct(T) -> Monoid{T}
#	mult_struct(Type{T}) -> Type{Monoid{T}}

+(a::DoubleMonoid{T}, b::DoubleMonoid{T}) where {T} = op(add_struct(a), add_struct(b))
*(a::DoubleMonoid{T}, b::DoubleMonoid{T}) where {T} = op(mult_struct(a), mult_struct(b))
zero(t::Type{DoubleMonoid{T}}) where {T} = id(add_struct(t))
zero(a::DoubleMonoid{T}) where {T} = id(add_struct(a))
one(t::Type{DoubleMonoid{T}}) where {T} = id(mult_struct(t))
one(a::DoubleMonoid{T}) where {T} = id(mult_struct(a))

abstract type NearRing{T} <: DoubleMonoid{T} end
# Should implement the functions:
#	add_struct(T) -> Group{T}
#	mult_struct(T) -> SemiGroup{T}
#	convert(Type{Self}, Additive)
#	convert(Type{Self}, Multiplicative)
# All other functions have sensible default implementations based on these
# Multiplication should also be right- xor left-distributive over addition

-(a::NearRing{T}, b::NearRing{T}) where {T} = op(add_struct(a), inv(add_struct(b)))

abstract type Rng{T} <: NearRing{T} end
# Misses the identity so it misses the I in RIng
# Should implement the functions:
#	add_struct(T) -> AlbelianGroup{T}
#	mult_struct(T) -> SemiGroup{T}
#	convert(Type{Self}, Additive)
#	convert(Type{Self}, Multiplicative)
# All other functions have sensible default implementations based on these
# Multiplication should also be distributive over addition

abstract type Ring{T} <: Rng{T} end
# Should implement the functions:
#	add_struct(T) -> AlbelianGroup{T}
#	mult_struct(T) -> Monoid{T}
#	convert(Type{Self}, Additive)
#	convert(Type{Self}, Multiplicative)
# All other functions have sensible default implementations based on these
# Multiplication should also be distributive over addition

abstract type DivisionRing{T} <: Ring{T} end
# Should implement the functions:
#	add_struct(T) -> AlbelianGroup{T}
#	mult_struct(T) -> Group{T}
#	convert(Type{Self}, Additive)
#	convert(Type{Self}, Multiplicative)
# All other functions have sensible default implementations based on these
# Multiplication should also be distributive over addition

/(a::DivisionRing{T}, b::DivisionRing{T}) where {T} = op(mult_struct(a), inv(mult_struct(b)))

abstract type Field{T} <: DivisionRing{T} end
# Should implement the functions:
#	add_struct(T) -> AlbelianGroup{T}
#	mult_struct(T) -> AlbelianGroup{T}
#	convert(Type{Self}, Additive)
#	convert(Type{Self}, Multiplicative)
# All other functions have sensible default implementations based on these
# Multiplication should also be distributive over addition

end