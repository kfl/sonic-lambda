use sonic_lambda::*;


fn main() {
    let mut arena = TermArena::default();
    let x0 = arena.var(0);
    let id = arena.lam(x0); // λ 0     a.k.a     λ x. x
    let e1 = arena.app(id, id);
    let env0 = Env::empty();
    let e2 = arena.app(e1, e1);
    println!("The term {:?} evaluates to {:?}", arena.get(e1), eval(&arena, e1, &env0));
    println!("The term {:?} evaluates to {:?}", arena.get(e2), eval(&arena, e2, &env0));

}
