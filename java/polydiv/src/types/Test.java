package polydiv.src.types;

public class Test {
	public static void main(String[] args) {
		Rational r1 = new Rational(3);
		Rational r2 = new Rational(1, 3);
		System.out.println(r1.mul(r2));
		System.out.println(r1.add(r2));
	}
}
