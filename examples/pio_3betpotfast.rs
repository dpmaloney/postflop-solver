//use criterion::{criterion_group, criterion_main, Criterion};
use postflop_solver::*;

fn main() {
    // ranges of OOP and IP in string format
    let oop_range = "AA,KK,QQ,JJ,TT,99,88,AK,AQ,AJ,ATs,ATo:0.75,A9s,A8s,A5s:0.5,A4s:0.5,A3s:0.5,A2s:0.5,KQ,KJs,KJo:0.75,KTs,KTo:0.25,K9s,QJs,QJo:0.5,QTs,Q9s,JTs,JTo:0.25,J9s,J8s,T9s,T8s,T7s:0.45,98s,97s,96s:0.45,87s,86s:0.75,85s:0.45,76s:0.75,75s:0.75,74s:0.45,65s:0.75,64s:0.5,63s:0.45,54s:0.75,53s:0.5,52s:0.45,43s:0.5,42s:0.45,32s:0.45";
    let ip_range = "AA:0.25,99,88,77,66,55,44,33,22,AQo,AJ,AT,A9,A8,A7s,A6s,A5s,A4s,A3s,A2s,KQ,KJ,KT,K9,K8s,K7s,K6s,K5s,K4s,K3s,K2s,QJ,QT,Q9,Q8s,Q7s,Q6s,Q5s,Q4s,Q3s,Q2s,JT,J9,J8s,J7s,J6s,T9,T8s,T7s,T6s,98,97s,96s,95s:0.5,87s,86s,85s:0.5,76s,75s,74s:0.5,65s,64s,63s:0.5,54s,53s:0.5,43s";

    let card_config = CardConfig {
        range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
        flop: flop_from_str("QsJh2h").unwrap(),
        turn: NOT_DEALT,
        river: NOT_DEALT,
    };

    // bet sizes -> 60% of the pot, geometric size, and all-in
    // raise sizes -> 2.5x of the previous bet
    // see the documentation of `BetSizeCandidates` for more details
    let flop_sizes = BetSizeOptions::try_from(("100%", "100%")).unwrap();
    let turn_sizes = BetSizeOptions::try_from(("100%", "100%")).unwrap();
    let river_sizes = BetSizeOptions::try_from(("100%", "100%")).unwrap();

    let turn_donk_sizes = DonkSizeOptions::try_from("100%").unwrap();
    let river_donk_sizes = DonkSizeOptions::try_from("100%").unwrap();

    let tree_config = TreeConfig {
        initial_state: BoardState::Flop,
        starting_pot: 180,
        effective_stack: 910,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [flop_sizes.clone(), flop_sizes.clone()], // [OOP, IP]
        turn_bet_sizes: [turn_sizes.clone(), turn_sizes.clone()],
        river_bet_sizes: [river_sizes.clone(), river_sizes.clone()],
        turn_donk_sizes: Some(turn_donk_sizes.clone()), // use default bet sizes
        river_donk_sizes: Some(river_donk_sizes.clone()),
        add_allin_threshold: 0.0, // add all-in if (maximum bet size) <= 1.5x pot
        force_allin_threshold: 0.0, // force all-in if (SPR after the opponent's call) <= 0.15
        merging_threshold: 0.0,
    };

    // build the game tree
    let action_tree = ActionTree::new(tree_config).unwrap();
    let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();

    game.allocate_memory(false);

    let max_num_iterations = 1000;
    let target_exploitability = game.tree_config().starting_pot as f32 * 0.1; // 0.5% of the pot
    solve(&mut game, max_num_iterations, target_exploitability, true);

    
}



// fn criterion_benchmark(c: &mut Criterion) {
//     let mut group = c.benchmark_group("25samples");
//     // Configure Criterion.rs to detect smaller differences and increase sample size to improve
//     // precision and counteract the resulting noise.
//     group.sample_size(25);
//     group.bench_function("solve", |b| b.iter(|| solve_basic()));
//     group.finish()
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);