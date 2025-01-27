//! 取引関係管理モジュール

pub type TradeAmount = i32; // 取引量

use crate::models::Unit;

pub fn execute_trade(
    seller: &mut Unit,
    buyer: &mut Unit,
    amount: TradeAmount,
    price_per_unit: i32,
) -> bool {
    let total_price = amount * price_per_unit;

    // 売り手の資源確認
    if seller.resources < amount {
        println!("❌ 売り手の資源が不足しています");
        return false;
    }

    // 買い手の資金確認
    if buyer.funds < total_price {
        println!("❌ 買い手の資金が不足しています");
        return false;
    }

    // 取引実行
    seller.resources -= amount;
    seller.funds += total_price;
    buyer.resources += amount;
    buyer.funds -= total_price;

    println!(
        "✅ 取引成功: {} -> {} (量: {}, 価格: {}円)",
        seller.name, buyer.name, amount, total_price
    );

    true
}
