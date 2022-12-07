// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use crate::layout_builder::region::MemoryRegions;

pub fn parse_memory<P: ToString>(mut memory_regions: MemoryRegions, file_path: P) -> MemoryRegions {
    let mut reader = csv::Reader::from_path(file_path.to_string())
        .unwrap_or_else(|_| panic!("Open {} failed", file_path.to_string()));
    for record in reader.records() {
        let record = record.expect("Invalid file");
        if record.len() != 2 {
            panic!("Invalid column");
        }
        let name = &record[0];
        let length = parse_int(&record[1]).unwrap_or_else(|| panic!("Invalid Length {:?}", record));
        memory_regions = memory_regions.create_region(name, length);
    }
    memory_regions
}

fn parse_int(value: &str) -> Option<usize> {
    let value = value.trim();
    let mut base = 10;
    let value = if value.starts_with("0x") || value.ends_with('h') {
        base = 16;
        value.trim_start_matches("0x").trim_end_matches('h')
    } else if value.starts_with("0o") || value.ends_with('o') {
        base = 8;
        value.trim_start_matches("0o").trim_end_matches('o')
    } else if value.starts_with("0b") || value.ends_with('b') {
        base = 2;
        value.trim_start_matches("0b").trim_end_matches('b')
    } else {
        value
    };
    usize::from_str_radix(value, base).ok()
}
