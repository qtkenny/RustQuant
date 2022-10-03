use crate::{dnorm, pnorm, BlackScholes};

// ############################################################################
// STRUCTS
// ############################################################################

/// Struct to contain common Black-Scholes Greeks/sensitivities.
///
/// Implemented using the closed-form derivatives from the Black-Scholes model.
pub struct Greeks {
    /// Price sensitivity.
    Delta: (f64, f64),
    /// Price elasticity (measure of leverage, gearing).
    Lambda: (f64, f64),
    /// Price convexity.
    Gamma: (f64, f64),
    /// Volatility sensitivity.
    Vega: (f64, f64),
    /// Time sensitivity.
    Theta: (f64, f64),
    /// Interest rate sensitivity.
    Rho: (f64, f64),
    /// Dividend sensitivity.
    Phi: (f64, f64),
    /// In-the-money probabilities.
    Zeta: (f64, f64),
}

// ############################################################################
// FUNCTIONS
// ############################################################################

/// Function that computes the Black-Scholes Greeks/sensitivities.
pub fn Greeks(S: f64, K: f64, v: f64, r: f64, T: f64, q: f64) -> Greeks {
    let sqrtT: f64 = T.sqrt();
    let df: f64 = (-r * T).exp();
    let b: f64 = r - q;
    let ebrT: f64 = ((b - r) * T).exp();
    let Fp: f64 = S * (b * T).exp();
    let std: f64 = v * sqrtT;
    let d: f64 = (Fp / K).ln() / std;
    let d1: f64 = d + 0.5 * std;
    let d2: f64 = d1 - std;

    let nd1: f64 = dnorm(d1);
    // let nd2: f64 = dnorm(d2);
    let Nd1: f64 = pnorm(d1);
    let Nd2: f64 = pnorm(d2);

    // let nd1_: f64 = dnorm(-d1);
    // let nd2_: f64 = dnorm(-d2);
    let Nd1_: f64 = pnorm(-d1);
    let Nd2_: f64 = pnorm(-d2);

    let BS = BlackScholes(S, K, v, r, T, q);

    Greeks {
        Delta: (ebrT * Nd1, ebrT * (Nd1 - 1.0)),
        Lambda: (ebrT * Nd1 * S / BS.0, ebrT * (Nd1 - 1.0) * S / BS.1),
        Gamma: (
            (nd1 * ebrT) / (S * v * sqrtT),
            (nd1 * ebrT) / (S * v * sqrtT),
        ),
        Vega: (S * ebrT * nd1 * sqrtT, S * ebrT * nd1 * sqrtT),
        Theta: (
            -(S * ebrT * nd1 * v) / (2. * T.sqrt()) - (b - r) * S * ebrT * Nd1 - r * K * df * Nd2,
            -(S * ebrT * nd1 * v) / (2. * T.sqrt()) + (b - r) * S * ebrT * Nd1_ + r * K * df * Nd2_,
        ),
        Rho: (T * K * df * Nd2, -T * K * df * Nd2_),
        Phi: (-T * S * ebrT * Nd1, T * S * ebrT * Nd1_),
        Zeta: (Nd2, Nd2_),
    }
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::assert_approx_equal;

    #[test]
    fn TEST_greeks() {
        for strike in 1..=1000 {
            let S: f64 = 100.0;
            let v: f64 = 0.2;
            let r: f64 = 0.05;
            let T: f64 = 1.0;
            let q: f64 = 0.03;

            let g = Greeks(S, strike as f64, v, r, T, q);

            // ################################################################
            // DELTA
            // ################################################################

            // Delta: long call (short put) in (0,1)
            assert!(g.Delta.0 >= 0.0 && g.Delta.0 <= 1.0);

            // Delta: short call (long put) in (-1,0)
            assert!(g.Delta.1 >= -1.0 && g.Delta.1 <= 0.0);

            // Delta: D_call - D_put = 1
            assert_approx_equal(g.Delta.0 - g.Delta.1, 1.0, 0.1);

            // ################################################################
            // VEGA
            // ################################################################

            // Vega is the same for calls/puts, and always positive.
            assert_approx_equal(g.Vega.0, g.Vega.1, 1e-10);
            assert!(g.Vega.0 >= 0.0 && g.Vega.1 >= 0.0);

            // ################################################################
            // GAMMA
            // ################################################################

            // Gamma always within (0,1)
            assert!(g.Gamma.0 >= 0.0 && g.Gamma.0 <= 1.0);
            assert!(g.Gamma.1 >= 0.0 && g.Gamma.1 <= 1.0);

            // ################################################################
            // RHO
            // ################################################################

            // Rho > 0 for long calls
            assert!(g.Rho.0 > 0.0);

            // Rho < 0 for long puts
            assert!(g.Rho.1 < 0.0);

            // ################################################################
            // LAMBDA
            // ################################################################

            // ################################################################
            // THETA
            // ################################################################

            println!(
                "strike = {}, call theta = {}, put theta = {}",
                strike, g.Theta.0, g.Theta.1
            );

            // ################################################################
            // PHI/EPSILON
            // ################################################################

            // ################################################################
            // ZETA
            // ################################################################

            // println!(
            //     "{} \t Lambda: \tCall = {}, \tPut = {}",
            //     strike, g.Lambda.0, g.Lambda.1
            // );
            // assert_approx_equal(g.Lambda.0 - g.Lambda.1, 1.0, 0.5);
            // assert!(g.Lambda.0 >= 0.0 && g.Lambda.0 <= 1.1);
            // assert!(g.Lambda.1 >= 0.0 && g.Lambda.1 <= 1.0);
            // println!("DELTA PARITY = {}", g.Delta.0 - g.Delta.1);
            // println!("THETA = {}, {}", g.Theta.0, g.Theta.1);
            // assert!(g.Theta.1 == 69.0);
            // Greeks(S, K, v, r, T, q)
            // println!("strike = {}, call delta = {}, put delta = {}", strike, g.Delta.0, g.Delta.1);
            // assert!(g.Theta.0 <= 0.0 && g.Theta.1 <= 0.0);
        }
    }
}