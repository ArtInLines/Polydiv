package java.polydiv.structures;

public abstract class Ring<T extends Ring<T, A, M>, A extends AlbelianGroup<T>, M extends Monoid<T>>
		extends Rng<T, A, M> {
	/**
	 * Get the one element. Must be the same as the additive identity, i.e.
	 * `M.getIdentity() == getOne()` for all elements.
	 *
	 */
	public T getOne() {
		return multiplicatveStruct.getIdentity();
	}

	public T mul(T... factors) {
		T product = self();
		for (T x : factors) {
			product = product.mul(x);
		}
		return product;
	}

	/*
	 * The following is true for all albelian groups.
	 * (-a -b) =? -(a b)
	 * thus we can optimize sub()
	 *
	 * Proof:
	 * if (a x) = e, then x = -a
	 * (a b) -(a b) = e
	 * e = a -a = a -a b -b = a (-a b) -b = a (b -a) -b = (a b) (-a -b)
	 */

	@Override
	public T sub(T... operands) {
		return sub(getZero().add(operands));
	}
}
