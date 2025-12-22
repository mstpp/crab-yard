use byte_unit::{Byte, UnitType};
use prettytable::{Table, format, row};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ExtStats {
    pub count: u32,
    pub bytes: u64,
    pub is_bin: bool,
}

#[derive(Debug)]
pub struct ExtAgg {
    pub stats: HashMap<String, ExtStats>,
}

impl ExtAgg {
    pub fn new() -> Self {
        ExtAgg {
            stats: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, extension: String, size: u64, is_bin: bool) {
        self.stats
            .entry(extension)
            .and_modify(|s| {
                s.count += 1;
                s.bytes += size
            })
            .or_insert(ExtStats {
                count: 1,
                bytes: size,
                is_bin,
            });
    }

    pub fn display(&self) {
        let mut table = Table::from_csv_string("Extension,Count,Size").unwrap();
        let mut bin_total_size = 0_u64;

        // Remove internal horizontal lines
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        // Sort by size desc
        let mut sorted: Vec<(&String, &ExtStats)> = self.stats.iter().collect();
        sorted.sort_by_key(|&(_, v)| std::cmp::Reverse(v.bytes));

        for (ext, stats) in sorted.iter() {
            let byte_size = Byte::from_u64(stats.bytes).get_appropriate_unit(UnitType::Binary);
            let formatted_size = format!("{:.2}", byte_size);
            table.add_row(row![ext, stats.count, formatted_size,]);
            if stats.is_bin {
                bin_total_size += stats.bytes;
            }
        }
        println!("{table}");
        let bin_total = Byte::from_u64(bin_total_size).get_appropriate_unit(UnitType::Binary);
        println!("\nTotal bin files size: {bin_total:.2}");
    }
}
