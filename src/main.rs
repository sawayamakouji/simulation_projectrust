use simulation_project::models::Unit;

fn main() {
    let producer = Unit::new("農家", 1000, 50);
    let manufacturer = Unit::new("食品加工業者", 5000, 10);
    
    println!("✅ シミュレーション開始");
    println!("👷 生産者: {} (資金: {}円, 資源: {}t)", 
        producer.name, producer.funds, producer.resources);
    println!("🏭 加工業者: {} (資金: {}円, 資源: {}t)",
        manufacturer.name, manufacturer.funds, manufacturer.resources);
}
