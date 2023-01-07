package java.polydiv.structures;

public interface Monoid<T> extends SemiGroup<T> {
	public abstract T getIdentity();
}
