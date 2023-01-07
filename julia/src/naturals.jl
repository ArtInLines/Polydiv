module NaturalType

import Base.convert
using ..Structures
import ..Structures: op, id, inv, add_struct, mult_struct


struct NaturalAdd{T<:Unsigned} <: Monoid{T}
    a::T
end
NaturalAdd(x::Integer) = NaturalAdd(Unsigned(x))

op(x::NaturalAdd, y::NaturalAdd)::NaturalAdd = NaturalAdd(x.a + y.a)
id(::Type{NaturalAdd{T}}) where {T} = NaturalAdd(0)
id(::Type{NaturalAdd}) = NaturalAdd(0)

Base.show(io::IO, n::NaturalAdd) = print(io, "NaturalAdd(", n.a, ")")

# Testing
# Testing
@assert id(NaturalAdd(1)) === id(NaturalAdd) === NaturalAdd(0)
a = NaturalAdd(15)
@assert op(a, id(NaturalAdd)) === op(id(NaturalAdd), a) === a
@assert op(NaturalAdd(4), NaturalAdd(7)) === NaturalAdd(11)



struct NaturalMul{T<:Unsigned} <: Monoid{T}
    a::T
end
NaturalMul(x::Integer) = NaturalMul(Unsigned(x))

op(x::NaturalMul, y::NaturalMul)::NaturalMul = NaturalMul(x.a * y.a)
id(::Type{NaturalMul{T}}) where {T} = NaturalMul(1)
id(::Type{NaturalMul}) = NaturalMul(1)

Base.show(io::IO, n::NaturalMul) = print(io, "NaturalMul(", n.a, ")")

# Testing
@assert id(NaturalMul(3)) === id(NaturalMul) === NaturalMul(1)
a = NaturalMul(15)
@assert op(a, id(NaturalMul)) === op(id(NaturalMul), a) === a
@assert op(NaturalMul(4), NaturalMul(7)) === NaturalMul(28)



struct Natural{T<:Unsigned} <: DoubleMonoid{T}
    a::T
end
Natural() = Natural(0)
Natural(x::Integer) = Natural(Unsigned(x))
Natural(n::NaturalAdd{<:Unsigned}) = Natural(n.a)
Natural(n::NaturalMul{<:Unsigned}) = Natural(n.a)

add_struct(n::Natural) = NaturalAdd(n.a)
add_struct(::Type{Natural{T}}) where {T} = NaturalAdd{T}
mult_struct(n::Natural) = NaturalMul(n.a)
mult_struct(::Type{Natural{T}}) where {T} = NaturalMul{T}

Base.show(io::IO, n::Natural) = print(io, "Natural(", n.a, ")")

convert(::Type{Natural}, a::NaturalAdd) = Natural(a)
convert(::Type{Natural}, a::NaturalMul) = Natural(a)

# Testing
Natural(0)
zero(Natural())

end