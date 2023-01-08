package polydiv.src.structures;

public interface Monoid<T> extends SemiGroup<T> {
	public abstract T getIdentity();
}
