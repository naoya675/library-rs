use dynamic_montgomery_modint::{DefaultId, DynamicMontgomeryModint};
use pollard_rho::factorize;

type Mint = DynamicMontgomeryModint<DefaultId>;

pub fn primitive_root(p: u64) -> u64 {
    if p == 2 {
        return 1;
    }
    let mut qs = factorize(p - 1);
    qs.dedup();
    Mint::set_mod(p);
    for g in 2..p {
        if qs.iter().all(|&q| Mint::from(g).pow((p - 1) / q).value() != 1) {
            return g;
        }
    }
    unreachable!()
}
