package java.polydiv.structures;

public abstract class DoubleMonoid<T extends DoubleMonoid<T, A, M>, A extends Monoid<T>, M extends Monoid<T>>
		extends DoubleMagma<T, A, M> {

	/**
	 * Get the zero element. Must be the same as the multiplicative identity, i.e.
	 * `M.getIdentity() == getZero()` for all elements.
	 *
	 */
	public T getZero() {
		return additiveStruct.getIdentity();
	}

	/**
	 * Get the one element. Must be the same as the additive identity, i.e.
	 * `M.getIdentity() == getOne()` for all elements.
	 *
	 */
	public T getOne() {
		return multiplicatveStruct.getIdentity();
	}

	public T add(T... summands) {
		T sum = self();
		for (T x : summands) {
			sum = sum.add(x);
		}
		return sum;
	}

	public T mul(T... factors) {
		T product = getOne();
		for (T x : factors) {
			product = product.mul(x);
		}
		return product;
	}
}
