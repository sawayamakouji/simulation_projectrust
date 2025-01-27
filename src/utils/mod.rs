use std::fs::File;
use std::io::Write;
use petgraph::dot::{Dot, Config};
use petgraph::Graph;

pub fn export_graph() {
    let mut graph = Graph::<&str, &str>::new();

    // ノードを追加
    let farmer = graph.add_node("農家");
    let manufacturer = graph.add_node("加工業者");

    // エッジを追加
    graph.add_edge(farmer, manufacturer, "取引");

    // .dot形式でファイルにエクスポート
    let dot_output = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let mut file = File::create("graph_output.dot").expect("ファイル作成に失敗しました");
    file.write_all(dot_output.as_bytes()).expect("ファイルへの書き込みに失敗しました");

    println!("✅ グラフを 'graph_output.dot' に出力しました");
}
