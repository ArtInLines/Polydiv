package java.polydiv.structures;

public abstract class DoubleMagma<T extends DoubleMagma<T, A, M>, A extends Magma<T>, M extends Magma<T>> {
	protected A additiveStruct;
	protected M multiplicatveStruct;

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
