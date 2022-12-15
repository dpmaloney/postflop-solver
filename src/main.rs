
use anyhow::anyhow;
use postflop_solver::*;
use tpe::*;
use rand::SeedableRng as _;

fn run_solve(size1: i32, size2: i32, size3: i32, raise1: f32, raise2: f32) -> f64{
    let oop_range = "5c4c: 0.4, 5d4d: 0.4, 5d5c: 0.08, 5h4h: 0.4, 5h5c: 0.08, 5h5d: 0.08, 5s4s: 0.4, 5s5c: 0.08, 5s5d: 0.08, 5s5h: 0.08, 6c5c: 0.31, 6d5d: 0.31, 6d6c: 0.04, 6h5h: 0.31, 6h6c: 0.04, 6h6d: 0.04, 6s5s: 0.31, 6s6c: 0.04, 6s6d: 0.04, 6s6h: 0.04, 7c5c: 0.03, 7c6c: 0.28, 7d5d: 0.03, 7d6d: 0.28, 7d7c: 0.07, 7h5h: 0.03, 7h6h: 0.28, 7h7c: 0.07, 7h7d: 0.07, 7s5s: 0.03, 7s6s: 0.28, 7s7c: 0.07, 7s7d: 0.07, 7s7h: 0.07, 8c6c: 0.1, 8c7c: 0.185, 8d6d: 0.1, 8d7d: 0.185, 8d8c: 0.14, 8h6h: 0.1, 8h7h: 0.185, 8h8c: 0.14, 8h8d: 0.14, 8s6s: 0.1, 8s7s: 0.185, 8s8c: 0.14, 8s8d: 0.14, 8s8h: 0.14, 9c7c: 0.08, 9c8c: 0.29, 9d7d: 0.08, 9d8d: 0.29, 9d9c: 0.4, 9h7h: 0.08, 9h8h: 0.29, 9h9c: 0.4, 9h9d: 0.4, 9s7s: 0.08, 9s8s: 0.29, 9s9c: 0.4, 9s9d: 0.4, 9s9h: 0.4, Tc6c: 0.415, Tc7c: 0.09, Tc8c: 0.105, Tc9c: 0.605, Tc9d: 0.24, Tc9h: 0.24, Tc9s: 0.24, Td6d: 0.415, Td7d: 0.09, Td8d: 0.105, Td9c: 0.24, Td9d: 0.605, Td9h: 0.24, Td9s: 0.24, TdTc: 1, Th6h: 0.415, Th7h: 0.09, Th8h: 0.105, Th9c: 0.24, Th9d: 0.24, Th9h: 0.605, Th9s: 0.24, ThTc: 1, ThTd: 1, Ts6s: 0.415, Ts7s: 0.09, Ts8s: 0.105, Ts9c: 0.24, Ts9d: 0.24, Ts9h: 0.24, Ts9s: 0.605, TsTc: 1, TsTd: 1, TsTh: 1, Jc4c: 0.39, Jc5c: 0.365, Jc6c: 0.49, Jc7c: 0.33, Jc8c: 0.11, Jc9c: 0.165, Jc9d: 0.175, Jc9h: 0.175, Jc9s: 0.175, JcTc: 0.435, JcTd: 0.265, JcTh: 0.265, JcTs: 0.265, Jd4d: 0.39, Jd5d: 0.365, Jd6d: 0.49, Jd7d: 0.33, Jd8d: 0.11, Jd9c: 0.175, Jd9d: 0.165, Jd9h: 0.175, Jd9s: 0.175, JdTc: 0.265, JdTd: 0.435, JdTh: 0.265, JdTs: 0.265, JdJc: 1, Jh4h: 0.39, Jh5h: 0.365, Jh6h: 0.49, Jh7h: 0.33, Jh8h: 0.11, Jh9c: 0.175, Jh9d: 0.175, Jh9h: 0.165, Jh9s: 0.175, JhTc: 0.265, JhTd: 0.265, JhTh: 0.435, JhTs: 0.265, JhJc: 1, JhJd: 1, Js4s: 0.39, Js5s: 0.365, Js6s: 0.49, Js7s: 0.33, Js8s: 0.11, Js9c: 0.175, Js9d: 0.175, Js9h: 0.175, Js9s: 0.165, JsTc: 0.265, JsTd: 0.265, JsTh: 0.265, JsTs: 0.435, JsJc: 1, JsJd: 1, JsJh: 1, Qc2c: 0.49, Qc3c: 0.435, Qc4c: 0.295, Qc5c: 0.04, Qc6c: 0.11, Qc7c: 0.195, Qc8c: 0.16, Qc9d: 0.18, Qc9h: 0.18, Qc9s: 0.18, QcTd: 0.445, QcTh: 0.445, QcTs: 0.445, QcJd: 0.42, QcJh: 0.42, QcJs: 0.42, Qd2d: 0.49, Qd3d: 0.435, Qd4d: 0.295, Qd5d: 0.04, Qd6d: 0.11, Qd7d: 0.195, Qd8d: 0.16, Qd9c: 0.18, Qd9h: 0.18, Qd9s: 0.18, QdTc: 0.445, QdTh: 0.445, QdTs: 0.445, QdJc: 0.42, QdJh: 0.42, QdJs: 0.42, QdQc: 1, Qh2h: 0.49, Qh3h: 0.435, Qh4h: 0.295, Qh5h: 0.04, Qh6h: 0.11, Qh7h: 0.195, Qh8h: 0.16, Qh9c: 0.18, Qh9d: 0.18, Qh9s: 0.18, QhTc: 0.445, QhTd: 0.445, QhTs: 0.445, QhJc: 0.42, QhJd: 0.42, QhJs: 0.42, QhQc: 1, QhQd: 1, Qs2s: 0.49, Qs3s: 0.435, Qs4s: 0.295, Qs5s: 0.04, Qs6s: 0.11, Qs7s: 0.195, Qs8s: 0.16, Qs9c: 0.18, Qs9d: 0.18, Qs9h: 0.18, QsTc: 0.445, QsTd: 0.445, QsTh: 0.445, QsJc: 0.42, QsJd: 0.42, QsJh: 0.42, QsQc: 1, QsQd: 1, QsQh: 1, Kc2c: 0.225, Kc3c: 0.21, Kc4c: 0.035, Kc5c: 0.12, Kc6c: 0.14, Kc7c: 0.225, Kc8d: 0.12, Kc8h: 0.12, Kc8s: 0.12, Kc9d: 0.525, Kc9h: 0.525, Kc9s: 0.525, KcTc: 0.44, KcTd: 0.425, KcTh: 0.425, KcTs: 0.425, KcJc: 0.065, KcJd: 0.49, KcJh: 0.49, KcJs: 0.49, KcQc: 1, KcQd: 0.285, KcQh: 0.285, KcQs: 0.285, Kd2d: 0.225, Kd3d: 0.21, Kd4d: 0.035, Kd5d: 0.12, Kd6d: 0.14, Kd7d: 0.225, Kd8c: 0.12, Kd8h: 0.12, Kd8s: 0.12, Kd9c: 0.525, Kd9h: 0.525, Kd9s: 0.525, KdTc: 0.425, KdTd: 0.44, KdTh: 0.425, KdTs: 0.425, KdJc: 0.49, KdJd: 0.065, KdJh: 0.49, KdJs: 0.49, KdQc: 0.285, KdQd: 1, KdQh: 0.285, KdQs: 0.285, KdKc: 1, Kh2h: 0.225, Kh3h: 0.21, Kh4h: 0.035, Kh5h: 0.12, Kh6h: 0.14, Kh7h: 0.225, Kh8c: 0.12, Kh8d: 0.12, Kh8s: 0.12, Kh9c: 0.525, Kh9d: 0.525, Kh9s: 0.525, KhTc: 0.425, KhTd: 0.425, KhTh: 0.44, KhTs: 0.425, KhJc: 0.49, KhJd: 0.49, KhJh: 0.065, KhJs: 0.49, KhQc: 0.285, KhQd: 0.285, KhQh: 1, KhQs: 0.285, KhKc: 1, KhKd: 1, Ks2s: 0.225, Ks3s: 0.21, Ks4s: 0.035, Ks5s: 0.12, Ks6s: 0.14, Ks7s: 0.225, Ks8c: 0.12, Ks8d: 0.12, Ks8h: 0.12, Ks9c: 0.525, Ks9d: 0.525, Ks9h: 0.525, KsTc: 0.425, KsTd: 0.425, KsTh: 0.425, KsTs: 0.44, KsJc: 0.49, KsJd: 0.49, KsJh: 0.49, KsJs: 0.065, KsQc: 0.285, KsQd: 0.285, KsQh: 0.285, KsQs: 1, KsKc: 1, KsKd: 1, KsKh: 1, Ac2c: 0.1, Ac3c: 0.055, Ac3d: 0.085, Ac3h: 0.085, Ac3s: 0.085, Ac4c: 0.375, Ac4d: 0.335, Ac4h: 0.335, Ac4s: 0.335, Ac5c: 1, Ac5d: 0.395, Ac5h: 0.395, Ac5s: 0.395, Ac6d: 0.115, Ac6h: 0.115, Ac6s: 0.115, Ac7d: 0.545, Ac7h: 0.545, Ac7s: 0.545, Ac8d: 0.305, Ac8h: 0.305, Ac8s: 0.305, Ac9d: 0.17, Ac9h: 0.17, Ac9s: 0.17, AcTc: 0.025, AcTd: 0.34, AcTh: 0.34, AcTs: 0.34, AcJc: 1, AcJd: 0.175, AcJh: 0.175, AcJs: 0.175, AcQc: 1, AcQd: 0.825, AcQh: 0.825, AcQs: 0.825, AcKc: 1, AcKd: 1, AcKh: 1, AcKs: 1, Ad2d: 0.1, Ad3c: 0.085, Ad3d: 0.055, Ad3h: 0.085, Ad3s: 0.085, Ad4c: 0.335, Ad4d: 0.375, Ad4h: 0.335, Ad4s: 0.335, Ad5c: 0.395, Ad5d: 1, Ad5h: 0.395, Ad5s: 0.395, Ad6c: 0.115, Ad6h: 0.115, Ad6s: 0.115, Ad7c: 0.545, Ad7h: 0.545, Ad7s: 0.545, Ad8c: 0.305, Ad8h: 0.305, Ad8s: 0.305, Ad9c: 0.17, Ad9h: 0.17, Ad9s: 0.17, AdTc: 0.34, AdTd: 0.025, AdTh: 0.34, AdTs: 0.34, AdJc: 0.175, AdJd: 1, AdJh: 0.175, AdJs: 0.175, AdQc: 0.825, AdQd: 1, AdQh: 0.825, AdQs: 0.825, AdKc: 1, AdKd: 1, AdKh: 1, AdKs: 1, AdAc: 1, Ah2h: 0.1, Ah3c: 0.085, Ah3d: 0.085, Ah3h: 0.055, Ah3s: 0.085, Ah4c: 0.335, Ah4d: 0.335, Ah4h: 0.375, Ah4s: 0.335, Ah5c: 0.395, Ah5d: 0.395, Ah5h: 1, Ah5s: 0.395, Ah6c: 0.115, Ah6d: 0.115, Ah6s: 0.115, Ah7c: 0.545, Ah7d: 0.545, Ah7s: 0.545, Ah8c: 0.305, Ah8d: 0.305, Ah8s: 0.305, Ah9c: 0.17, Ah9d: 0.17, Ah9s: 0.17, AhTc: 0.34, AhTd: 0.34, AhTh: 0.025, AhTs: 0.34, AhJc: 0.175, AhJd: 0.175, AhJh: 1, AhJs: 0.175, AhQc: 0.825, AhQd: 0.825, AhQh: 1, AhQs: 0.825, AhKc: 1, AhKd: 1, AhKh: 1, AhKs: 1, AhAc: 1, AhAd: 1, As2s: 0.1, As3c: 0.085, As3d: 0.085, As3h: 0.085, As3s: 0.055, As4c: 0.335, As4d: 0.335, As4h: 0.335, As4s: 0.375, As5c: 0.395, As5d: 0.395, As5h: 0.395, As5s: 1, As6c: 0.115, As6d: 0.115, As6h: 0.115, As7c: 0.545, As7d: 0.545, As7h: 0.545, As8c: 0.305, As8d: 0.305, As8h: 0.305, As9c: 0.17, As9d: 0.17, As9h: 0.17, AsTc: 0.34, AsTd: 0.34, AsTh: 0.34, AsTs: 0.025, AsJc: 0.175, AsJd: 0.175, AsJh: 0.175, AsJs: 1, AsQc: 0.825, AsQd: 0.825, AsQh: 0.825, AsQs: 1, AsKc: 1, AsKd: 1, AsKh: 1, AsKs: 1, AsAc: 1, AsAd: 1, AsAh: 1";
    let ip_range = "3d3c: 0.03, 3h3c: 0.03, 3h3d: 0.03, 3s3c: 0.03, 3s3d: 0.03, 3s3h: 0.03, 4d4c: 0.11, 4h4c: 0.11, 4h4d: 0.11, 4s4c: 0.11, 4s4d: 0.11, 4s4h: 0.11, 5c4c: 0.186, 5d4d: 0.186, 5d5c: 0.26, 5h4h: 0.186, 5h5c: 0.26, 5h5d: 0.26, 5s4s: 0.186, 5s5c: 0.26, 5s5d: 0.26, 5s5h: 0.26, 6c5c: 0.116, 6h5h: 0.116, 6h6c: 0.526, 6s5s: 0.116, 6s6c: 0.526, 6s6h: 0.526, 7c6c: 0.23, 7d7c: 0.75, 7h6h: 0.23, 7h7c: 0.75, 7h7d: 0.75, 7s6s: 0.23, 7s7c: 0.75, 7s7d: 0.75, 7s7h: 0.75, 8c7c: 0.256, 8d7d: 0.256, 8d8c: 0.81, 8h7h: 0.256, 8h8c: 0.81, 8h8d: 0.81, 8s7s: 0.256, 8s8c: 0.81, 8s8d: 0.81, 8s8h: 0.81, 9c8c: 0.056, 9d8d: 0.056, 9d9c: 0.956, 9h8h: 0.056, 9h9c: 0.956, 9h9d: 0.956, Tc9c: 0.46, Td9d: 0.46, TdTc: 0.376, Th9h: 0.46, ThTc: 0.376, ThTd: 0.376, TsTc: 0.376, TsTd: 0.376, TsTh: 0.376, Jc9c: 0.216, JcTc: 1, Jd9d: 0.216, JdTd: 1, JdJc: 0.14, Jh9h: 0.216, JhTh: 1, JhJc: 0.14, JhJd: 0.14, JsTs: 1, JsJc: 0.14, JsJd: 0.14, JsJh: 0.14, QcTc: 1, QcJc: 1, QdTd: 1, QdJd: 1, QhTh: 1, QhJh: 1, QsTs: 1, QsJs: 1, Kc6c: 0.54, Kc9c: 0.646, KcTc: 1, KcJc: 1, KcQc: 1, KcQd: 0.336, KcQh: 0.336, KcQs: 0.336, Kd9d: 0.646, KdTd: 1, KdJd: 1, KdQc: 0.336, KdQd: 1, KdQh: 0.336, KdQs: 0.336, Ks6s: 0.54, KsTs: 1, KsJs: 1, KsQc: 0.336, KsQd: 0.336, KsQh: 0.336, KsQs: 1, Ac4c: 0.416, Ac5c: 0.956, Ac7c: 0.316, Ac8c: 0.556, Ac9c: 1, AcTc: 1, AcJc: 1, AcJd: 0.326, AcJh: 0.326, AcJs: 0.326, AcQc: 0.52, AcQd: 0.366, AcQh: 0.366, AcQs: 0.366, Ad4d: 0.416, Ad5d: 0.956, Ad7d: 0.316, Ad8d: 0.556, Ad9d: 1, AdTd: 1, AdJc: 0.326, AdJd: 1, AdJh: 0.326, AdJs: 0.326, AdQc: 0.366, AdQd: 0.52, AdQh: 0.366, AdQs: 0.366, AdAc: 0.216, Ah4h: 0.416, Ah5h: 0.956, Ah7h: 0.316, Ah8h: 0.556, Ah9h: 1, AhTh: 1, AhJc: 0.326, AhJd: 0.326, AhJh: 1, AhJs: 0.326, AhQc: 0.366, AhQd: 0.366, AhQh: 0.52, AhQs: 0.366, AhAc: 0.216, AhAd: 0.216, As4s: 0.416, As5s: 0.956, As7s: 0.316, As8s: 0.556, AsTs: 1, AsJc: 0.326, AsJd: 0.326, AsJh: 0.326, AsJs: 1, AsQc: 0.366, AsQd: 0.366, AsQh: 0.366, AsQs: 0.52, AsAc: 0.216, AsAd: 0.216, AsAh: 0.216";

    let card_config = CardConfig {
        range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
        flop: flop_from_str("Kh9s6d").unwrap(),
        turn: NOT_DEALT,
        river: NOT_DEALT,
    };

    // bet sizes -> 60% of the pot, geometric size, and all-in
    // raise sizes -> 2.5x of the previous bet
    // see the documentation of `BetSizeCandidates` for more details
    let ip_bets = format!("{}%, {}%, {}%", size1, size2, size3);
    let ip_raises = format!("{:.1}x, {:.1}x", raise1, raise2);
    let oop_bet_sizes = BetSizeCandidates::try_from(("60%", "2.5x")).unwrap();
    let ip_bet_sizes = BetSizeCandidates::try_from((ip_bets.as_str(), ip_raises.as_str())).unwrap();
    
    let tree_config = TreeConfig {
        initial_state: BoardState::Flop,
        starting_pot: 265,
        effective_stack: 870,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [oop_bet_sizes.clone(), ip_bet_sizes.clone()], // [OOP, IP]
        turn_bet_sizes: [oop_bet_sizes.clone(), ip_bet_sizes.clone()],
        river_bet_sizes: [oop_bet_sizes.clone(), ip_bet_sizes.clone()],
        turn_donk_sizes: None, // use default bet sizes
        river_donk_sizes: None,
        add_allin_threshold: 1.5, // add all-in if (maximum bet size) <= 1.5x pot
        force_allin_threshold: 0.15, // force all-in if (SPR after the opponent's call) <= 0.15
        merging_threshold: 0.1,
    };

    // build the game tree
    let action_tree = ActionTree::new(tree_config).unwrap();
    let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();


    // allocate memory without compression
    game.allocate_memory(false);

    // allocate memory with compression
    // game.allocate_memory(true);

    // solve the game
    let max_num_iterations = 1000;
    let target_exploitability = game.tree_config().starting_pot as f32 * 0.005;
    let exploitability = solve(&mut game, max_num_iterations, target_exploitability, true);
    println!("Exploitability: {:.2}", exploitability);

    // solve the game manually
    // for i in 0..max_num_iterations {
    //     solve_step(&game, i);
    //     if (i + 1) % 10 == 0 {
    //         let exploitability = compute_exploitability(&game);
    //         if exploitability <= target_exploitability {
    //             println!("Exploitability: {:.2}", exploitability);
    //             break;
    //         }
    //     }
    // }
    // finalize(&mut game);

    // get equity and EV of a specific hand
    game.cache_normalized_weights();
    let ev = game.expected_values(1);

    // get equity and EV of whole hand
    let weights = game.normalized_weights(1);
    let average_ev = compute_average(&ev, weights);
    average_ev as f64
}



fn main()  -> anyhow::Result<()> {

    let choices = [15, 20, 25, 30, 35, 40, 45];
    let choices_med = [50, 55, 60, 65, 70, 75, 80, 85];
    let choices_lg = [90, 100, 110, 120, 130, 140, 150];
    let choices_r1 = [1.5, 2.0, 2.2, 2.5, 2.7, 3.0];
    let choices_r2 = [3.5, 4.0, 4.5, 5.0,6.0, 7.0];
    let mut optim1 =
    tpe::TpeOptimizer::new(tpe::histogram_estimator(), tpe::categorical_range(choices.len())?);
    let mut optim2 =
    tpe::TpeOptimizer::new(tpe::histogram_estimator(), tpe::categorical_range(choices_med.len())?);
    let mut optim3 =
    tpe::TpeOptimizer::new(tpe::histogram_estimator(), tpe::categorical_range(choices_lg.len())?);
    let mut optim4 =
    tpe::TpeOptimizer::new(tpe::histogram_estimator(), tpe::categorical_range(choices_r1.len())?);
    let mut optim5 =
    tpe::TpeOptimizer::new(tpe::histogram_estimator(), tpe::categorical_range(choices_r2.len())?);

    let mut best_value = std::f64::NEG_INFINITY;
    let mut best_b1 = 20;
    let mut best_b2 = 50;
    let mut best_b3 = 90;
    let mut best_r1 = 1.5;
    let mut best_r2 = 3.5;
    let mut rng = rand::rngs::StdRng::from_seed(Default::default());
    for _ in 0..100 {
        let b1 = optim1.ask(&mut rng)?;
        let b2 = optim2.ask(&mut rng)?;
        let b3 = optim3.ask(&mut rng)?;

        let r1 = optim4.ask(&mut rng)?;
        let r2 = optim5.ask(&mut rng)?;
     
        let v = run_solve(choices[b1 as usize], choices_med[b2 as usize], choices_lg[b3 as usize], choices_r1[r1 as usize], choices_r2[r2 as usize]);
        optim1.tell(b1, v)?;
        optim2.tell(b2, v)?;
        optim3.tell(b3, v)?;
        optim4.tell(r1, v)?;
        optim5.tell(r2, v)?;
        
        if v > best_value{
            best_value = v;
            best_b1 = choices[b1 as usize];
            best_b2 = choices_med[b2 as usize];
            best_b3 = choices_lg[b3 as usize];
            best_r1 = choices_r1[r1 as usize];
            best_r2 = choices_r2[r2 as usize];
            println!("{}, {}, {}, {}, {}, {}", best_b1, best_b2, best_b3, best_r1, best_r2, best_value);
        }
     }
     println!("{}, {}, {}, {}, {}, {}", best_b1, best_b2, best_b3, best_r1, best_r2, best_value);
     Ok(())
}