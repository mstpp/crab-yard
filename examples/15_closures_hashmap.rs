use std::collections::HashMap;

// v1: not idiomatic and functional, key is use of Entry
// whole fn replaced by:
//      `*counts.entry(key.into()).or_insert(0) += 1;`
//
// fn counter(hasp_map: &mut HashMap<String, u32>, key: &str) {
//     if hasp_map.contains_key(key) {
//         if let Some(v) = hasp_map.get_mut(key) {
//             *v += 1;
//         }
//     } else {
//         hasp_map.insert(String::from(key), 1);
//     }
// }

// v2: solid, acceptable
fn stats(logs: &Vec<&str>) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    logs.iter().for_each(|line| {
        if let Some(key) = line.split_ascii_whitespace().next() {
            *counts.entry(key.into()).or_insert(0) += 1;
        }
    });
    counts
}

// v3: 100% functional
fn stats_f(logs: &Vec<&str>) -> HashMap<String, u32> {
    logs.iter()
        .filter_map(|line| line.split_ascii_whitespace().next())
        .fold(HashMap::new(), |mut counts, level| {
            *counts.entry(level.into()).or_insert(0) += 1;
            counts
        })
}

fn main() {
    let logs = vec![
        "INFO start process",
        "INFO process start",
        "ERROR proc hang",
        "WARN no timeout set",
        "INFO recovered",
    ];

    let res = stats(&logs);
    let res2 = stats_f(&logs);
    assert_eq!(res, res2);
    println!("{res:?}"); // {"INFO": 3, "ERROR": 1, "N/A": 1}
}
