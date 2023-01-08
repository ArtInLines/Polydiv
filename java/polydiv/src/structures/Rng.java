package polydiv.src.structures;

public abstract class Rng<T extends Rng<T, A, M>, A extends AlbelianGroup<T>, M extends SemiGroup<T>>
		extends NearRing<T, A, M> {

	protected Rng(A additive, M multiplicative) {
		super(additive, multiplicative);
	}
}
