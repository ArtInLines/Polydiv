package polydiv.src;

import polydiv.src.structures.*;

class AdditiveInteger implements AlbelianGroup<Integer> {
	@Override
	public Integer op(Integer lhs, Integer rhs) {
		return new Integer(lhs.getNum() + rhs.getNum());
	}

	@Override
	public Integer getIdentity() {
		return new Integer(0);
	}

	@Override
	public Integer getInverse(Integer x) {
		return new Integer(-x.getNum());
	}
}

class MultiplicativeInteger implements Monoid<Integer> {
	@Override
	public Integer op(Integer lhs, Integer rhs) {
		return new Integer(lhs.getNum() * rhs.getNum());
	}

	@Override
	public Integer getIdentity() {
		return new Integer(1);
	}
}

public class Integer extends NearRing<Integer, AdditiveInteger, MultiplicativeInteger> {
	private static final AdditiveInteger additive = new AdditiveInteger();
	private static final MultiplicativeInteger multiplicative = new MultiplicativeInteger();

	protected int num;

	public Integer(int num) {
		super(additive, multiplicative);
		this.num = num;
	}

	@Override
	public Integer self() {
		return this;
	}

	public int getNum() {
		return this.num;
	}

	public void setNum(int num) {
		this.num = num;
	}

	@Override
	public String toString() {
		return getNum() + "";
	}
}
