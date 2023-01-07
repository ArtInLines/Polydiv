package java.polydiv.structures;

public abstract class Field<T extends Field<T, A, M>, A extends AlbelianGroup<T>, M extends AlbelianGroup<T>>
		extends DivisionRing<T, A, M> {

	/*
	 * The following is true for all albelian groups.
	 * (-a -b) =? -(a b)
	 * thus we can optimize div()
	 *
	 * Proof:
	 * if (a x) = e, then x = -a
	 * (a b) -(a b) = e
	 * e = a -a = a -a b -b = a (-a b) -b = a (b -a) -b = (a b) (-a -b)
	 */
	@Override
	public T div(T... operands) {
		return div(getOne().mul(operands));
	}
}
