package java.polydiv.structures;

public abstract class DivisionRing<T extends DivisionRing<T, A, M>, A extends AlbelianGroup<T>, M extends Group<T>>
		extends Ring<T, A, M> {
	public T div(T x) {
		return multiplicatveStruct.op(self(), multiplicatveStruct.getInverse(x));
	}

	public T div(T... operands) {
		T quotient = self();
		for (T x : operands) {
			quotient = quotient.div(x);
		}
		return quotient;
	}
}
