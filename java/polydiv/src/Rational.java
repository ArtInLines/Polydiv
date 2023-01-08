package polydiv.src;

import polydiv.src.structures.*;

class AdditiveRational
		implements AlbelianGroup<Rational> {
	@Override
	public Rational op(Rational lhs, Rational rhs) {
		if (lhs.getDenominator() != rhs.getDenominator()) {
			int numer = lhs.getNumerator() * rhs.getDenominator() + rhs.getNumerator() * lhs.getDenominator();
			int denom = lhs.getDenominator() * rhs.getDenominator();
			return new Rational(numer, denom);
		} else {
			return new Rational(lhs.getNumerator() + rhs.getNumerator(), lhs.getDenominator());
		}
	}

	@Override
	public Rational getIdentity() {
		return new Rational(0);
	}

	@Override
	public Rational getInverse(Rational x) {
		return new Rational(-x.getNumerator(), x.getDenominator());
	}
}

class MultiplicativeRational
		implements AlbelianGroup<Rational> {
	@Override
	public Rational getIdentity() {
		return new Rational(1);
	}

	@Override
	public Rational getInverse(Rational x) {
		return new Rational(x.getDenominator(), x.getNumerator());
	}

	@Override
	public Rational op(Rational lhs, Rational rhs) {
		return new Rational(lhs.getNumerator() * rhs.getNumerator(), lhs.getDenominator() * rhs.getDenominator());
	}
}

public class Rational extends Field<Rational, AdditiveRational, MultiplicativeRational> {
	private static final AdditiveRational additive = new AdditiveRational();
	private static final MultiplicativeRational multiplicative = new MultiplicativeRational();

	protected int numer;
	protected int denom;

	public Rational(int n) {
		super(additive, multiplicative);
		numer = n;
		denom = 1;
	}

	public Rational(int numer, int denom) {
		super(additive, multiplicative);
		this.numer = numer;
		this.denom = denom;
	}

	@Override
	public Rational self() {
		return this;
	}

	public int getNumerator() {
		return this.numer;
	}

	public void setNumerator(int numer) {
		this.numer = numer;
	}

	public int getDenominator() {
		return this.denom;
	}

	public void setDenominator(int denom) {
		this.denom = denom;
	}

	@Override
	public String toString() {
		return getNumerator() + " / " + getDenominator();
	}
}
