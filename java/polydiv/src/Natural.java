package polydiv.src;

import polydiv.src.exceptions.ConstructorException;
import polydiv.src.structures.*;

class AdditiveNatural implements Monoid<Natural> {
	@Override
	public Natural op(Natural lhs, Natural rhs) {
		return new Natural(lhs.getNum() + rhs.getNum());
	}

	@Override
	public Natural getIdentity() {
		return new Natural(0);
	}
}

class MultiplicativeNatural implements Monoid<Natural> {
	@Override
	public Natural op(Natural lhs, Natural rhs) {
		return new Natural(lhs.getNum() * rhs.getNum());
	}

	@Override
	public Natural getIdentity() {
		return new Natural(1);
	}
}

public class Natural extends DoubleMonoid<Natural, AdditiveNatural, MultiplicativeNatural> {
	private static final AdditiveNatural additive = new AdditiveNatural();
	private static final MultiplicativeNatural multiplicative = new MultiplicativeNatural();

	protected int num;

	public Natural(int n) {
		super(additive, multiplicative);
		if (n < 0)
			throw new ConstructorException("Naturals can't be constructed with negative numbers.");
		num = n;
	}

	@Override
	public Natural self() {
		return this;
	}

	public int getNum() {
		return num;
	}

	public void setNum(int n) {
		num = n;
	}

	@Override
	public String toString() {
		return getNum() + "";
	}
}
