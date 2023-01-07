package java.polydiv;

import java.polydiv.exceptions.ConstructorException;
import java.polydiv.structures.*;

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
	protected int num;

	public Natural(int n) {
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
}
