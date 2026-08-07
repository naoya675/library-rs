use pollard_rho::factors;

pub fn euler_phi(n: usize) -> usize {
    factors(n as u64)
        .iter()
        .map(|&(p, e)| {
            let p = p as usize;
            p.pow(e - 1) * (p - 1)
        })
        .product()
}
