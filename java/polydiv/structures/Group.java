package java.polydiv.structures;

public interface Group<T> extends Monoid<T> {
	public abstract T getInverse(T x);
}
