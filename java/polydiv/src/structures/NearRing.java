package polydiv.src.structures;

public abstract class NearRing<T extends NearRing<T, A, M>, A extends Group<T>, M extends SemiGroup<T>>
		extends DoubleMagma<T, A, M> {

	protected NearRing(A additive, M multiplicative) {
		super(additive, multiplicative);
	}

	/**
	 * Get the zero element. Must be the same as the multiplicative identity, i.e.
	 * `M.getIdentity() == getZero()` for all elements.
	 *
	 */
	public T getZero() {
		return additiveStruct.getIdentity();
	}

	public T add(T... summands) {
		T sum = self();
		for (T x : summands) {
			sum = sum.add(x);
		}
		return sum;
	}

	public T sub(T x) {
		return additiveStruct.op(self(), additiveStruct.getInverse(x));
	}

	public T sub(T... operands) {
		T diff = self();
		for (T x : operands) {
			diff = diff.sub(x);
		}
		return diff;
	}

}
