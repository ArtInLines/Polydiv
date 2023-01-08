package polydiv.src.structures;

public abstract class DoubleMagma<T extends DoubleMagma<T, A, M>, A extends Magma<T>, M extends Magma<T>> {
	protected final A additiveStruct;
	protected final M multiplicatveStruct;

	protected DoubleMagma(A additive, M multiplicative) {
		additiveStruct = additive;
		multiplicatveStruct = multiplicative;
	}

	public abstract T self();

	/**
	 * Add two elements
	 *
	 * @param x Right-hand summand
	 * @return The Sum of x and y
	 */
	public T add(T x) {
		return additiveStruct.op(self(), x);
	}

	/**
	 * Add two elements
	 *
	 * @param x Right-hand multiplicand
	 * @return The Product of x and y
	 */
	public T mul(T x) {
		return multiplicatveStruct.op(self(), x);
	}
}
