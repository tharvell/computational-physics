use std::f64::consts::*;

fn nu_decay (nu_0: f64, tau: f64, t: f64) -> f64 {
    return nu_0*E.powf(-t/tau);
}

pub fn euler_nu_decay(tau: f64, nu_0: f64, timesteps: u64, stepsize: f64 ) -> f64 {
    let mut t: f64 = 0.0;
    let mut nu: f64 = nu_0; 
    for _ in 0..timesteps {
        println!("{},{}", t, nu); 
        nu = nu_decay(nu, tau, t); 
        t = t + stepsize   
    }
    return nu ;
}