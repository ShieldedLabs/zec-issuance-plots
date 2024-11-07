use crate::{
    halving::halving_height,
    subsidy::Subsidy::{PosterityFund, NU5},
};

pub fn display_nu5_vs_zpf_crossover_heights() -> Result<(), Box<dyn std::error::Error>> {
    const START_HALVING: u64 = 2;
    const END_HALVING: u64 = 4;

    for halving in START_HALVING..END_HALVING {
        let height = halving_height(halving);
        println!("Halving {halving} at height {height}.");
    }

    let start_height = halving_height(START_HALVING);
    let end_height = halving_height(END_HALVING);

    let zpf = PosterityFund(start_height);
    let iter = NU5.into_iter().zip(zpf);

    let iter_window = iter
        .skip(u64_as_usize(start_height))
        .take(u64_as_usize(end_height - start_height));

    let mut larger_subsidy = zpf;

    for ((height, nu5_cb, _), (height2, zpf_cb, _)) in iter_window {
        assert_eq!(height, height2);

        let (next_larger, cb) = if nu5_cb > zpf_cb {
            (NU5, nu5_cb)
        } else {
            (zpf, zpf_cb)
        };

        if next_larger != larger_subsidy {
            larger_subsidy = next_larger;
            println!(
                "Crossover at {height}; now {larger_subsidy:?} coinbase amount is larger: {cb}"
            );
        }
    }
    Ok(())
}

// For readability at call-site:
fn u64_as_usize(u: u64) -> usize {
    usize::try_from(u).unwrap()
}
