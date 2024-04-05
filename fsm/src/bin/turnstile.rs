use fsm::turnstile;

fn main() {
    let mut fsm = turnstile::Turnstile::new();
    fsm.simulate();
}
