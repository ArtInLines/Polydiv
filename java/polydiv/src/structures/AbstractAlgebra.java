package polydiv.src.structures;

public interface AbstractAlgebra<I, O> {
	public abstract O op(I lhs, I rhs);
}
