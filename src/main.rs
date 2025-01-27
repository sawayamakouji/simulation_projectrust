use std::fs::OpenOptions;
use std::io::{self, Write};
use rand::Rng;
use simulation_project::models::Unit;
use simulation_project::trade::execute_trade;

fn main() {
    // 各ユニットを初期化
    let mut producer = Unit::new("農家", 1000, 100, 10);
    let mut wholesaler = Unit::new("卸売業者", 3000, 50, 5);
    let mut manufacturer = Unit::new("加工業者", 5000, 30, 5);
    let mut retailer = Unit::new("小売業者", 7000, 20, 5);

    println!("✅ シミュレーション開始");
    println!("ターンごとに進める場合は '1' を入力。複数ターンを一気に進める場合はターン数（例: '5'）を入力。終了するには '0' を入力してください。");

    let mut turn = 1;

    loop {
        println!("--- 現在のターン: {} ---", turn);
        println!("進めたいターン数を入力してください (0で終了): ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み取りに失敗しました");
        let input = input.trim();

        // 入力が数字かどうかを確認
        let turns_to_progress: i32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ 無効な入力です。ターン数を正しい形式で入力してください。");
                continue;
            }
        };

        if turns_to_progress == 0 {
            println!("⏹ シミュレーションを終了します。");
            break;
        }

        for _ in 0..turns_to_progress {
            println!("--- ターン {} ---", turn);

            // ランダムイベントの発生
            apply_random_event(&mut producer, &mut wholesaler, &mut manufacturer, &mut retailer);

            // 各ユニット間の取引
            process_trade(&mut producer, &mut wholesaler, 10, 100);
            process_trade(&mut wholesaler, &mut manufacturer, 10, 150);
            process_trade(&mut manufacturer, &mut retailer, 5, 200);

            // 状態表示
            display_status(turn, &producer, &wholesaler, &manufacturer, &retailer);

            // 状況をCSVに保存
            save_to_csv(turn, &producer, &wholesaler, &manufacturer, &retailer);

            turn += 1;
        }
    }
}

/// 各ユニット間の取引を処理
fn process_trade(seller: &mut Unit, buyer: &mut Unit, amount: i32, price_per_unit: i32) {
    if execute_trade(seller, buyer, amount, price_per_unit) {
        println!(
            "✅ 取引成功: {} -> {} (量: {}, 価格: {}円)",
            seller.name, buyer.name, amount, price_per_unit * amount
        );
    } else {
        println!(
            "❌ 取引失敗: {} -> {} (量: {}, 価格: {}円)",
            seller.name, buyer.name, amount, price_per_unit * amount
        );
    }
}

/// 各ユニットのステータスを表示
fn display_status(turn: i32, producer: &Unit, wholesaler: &Unit, manufacturer: &Unit, retailer: &Unit) {
    println!(
        "ターン {} 状況:\n\
         👷 農家: 資金: {}円, 資源: {}t\n\
         🚛 卸売業者: 資金: {}円, 資源: {}t\n\
         🏭 加工業者: 資金: {}円, 資源: {}t\n\
         🛒 小売業者: 資金: {}円, 資源: {}t\n",
        turn,
        producer.funds,
        producer.resources,
        wholesaler.funds,
        wholesaler.resources,
        manufacturer.funds,
        manufacturer.resources,
        retailer.funds,
        retailer.resources
    );
}

/// ランダムイベントを適用
fn apply_random_event(producer: &mut Unit, wholesaler: &mut Unit, manufacturer: &mut Unit, retailer: &mut Unit) {
    let mut rng = rand::thread_rng();
    let event = rng.gen_range(0..4);

    match event {
        0 => {
            let loss = rng.gen_range(5..15);
            producer.resources -= loss;
            println!("⚠️ ランダムイベント: 農家の資源が減少しました (-{}t)", loss);
        }
        1 => {
            let gain = rng.gen_range(10..20);
            wholesaler.resources += gain;
            println!("🌟 ランダムイベント: 卸売業者の資源が増加しました (+{}t)", gain);
        }
        2 => {
            let extra_demand = rng.gen_range(5..15);
            manufacturer.resources += extra_demand;
            println!("📈 ランダムイベント: 加工業者の需要が増加しました (+{}t)", extra_demand);
        }
        3 => {
            let profit = rng.gen_range(500..1000);
            retailer.funds += profit;
            println!("💰 ランダムイベント: 小売業者の利益が増加しました (+{}円)", profit);
        }
        _ => (),
    }
}

/// 状況をCSVに保存
fn save_to_csv(turn: i32, producer: &Unit, wholesaler: &Unit, manufacturer: &Unit, retailer: &Unit) {
    let file_path = "simulation_log.csv";

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("CSVファイルの作成に失敗しました");

    if file.metadata().unwrap().len() == 0 {
        writeln!(
            file,
            "ターン,農家の資金,農家の資源,卸売業者の資金,卸売業者の資源,加工業者の資金,加工業者の資源,小売業者の資金,小売業者の資源"
        )
        .expect("ヘッダーの書き込みに失敗しました");
    }

    writeln!(
        file,
        "{},{},{},{},{},{},{},{},{}",
        turn,
        producer.funds,
        producer.resources,
        wholesaler.funds,
        wholesaler.resources,
        manufacturer.funds,
        manufacturer.resources,
        retailer.funds,
        retailer.resources
    )
    .expect("データの書き込みに失敗しました");
}
