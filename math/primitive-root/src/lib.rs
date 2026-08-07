use dynamic_montgomery_modint::{DefaultId, DynamicMontgomeryModint};
use pollard_rho::factors;

type Mint = DynamicMontgomeryModint<DefaultId>;

pub fn primitive_root(p: u64) -> u64 {
    if p == 2 {
        return 1;
    }
    let qs = factors(p - 1);
    Mint::set_mod(p);
    for g in 2..p {
        if qs.iter().all(|&(q, _)| Mint::from(g).pow((p - 1) / q).value() != 1) {
            return g;
        }
    }
    unreachable!()
}
