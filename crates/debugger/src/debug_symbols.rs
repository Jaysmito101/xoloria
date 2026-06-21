use crate::disassembly::DisasmEntry;
use ratatui::style::Style;
use std::collections::HashMap;

pub type SourceFileCache = HashMap<String, Option<Vec<Vec<(String, Style)>>>>;

pub struct DebugSymbols {
    pub source_lines: HashMap<u64, String>,
    pub source_locations: HashMap<u64, (String, u32)>,
    pub source_files_cache: SourceFileCache,
    pub symbols: HashMap<u64, String>,
    pub sorted_symbols: Vec<(u64, String)>,
    pub function_params: HashMap<u64, Vec<String>>,
    pub syntax_set: syntect::parsing::SyntaxSet,
    pub theme_set: syntect::highlighting::ThemeSet,
}

impl DebugSymbols {
    fn syntect_style_to_ratatui(style: syntect::highlighting::Style) -> Style {
        Style::default().fg(ratatui::style::Color::Rgb(
            style.foreground.r,
            style.foreground.g,
            style.foreground.b,
        ))
    }

    pub fn new(elf_path: &str) -> Self {
        let (source_lines, source_locations, symbols, function_params) =
            Self::load_elf_symbols(elf_path);

        let sorted_symbols = {
            let mut s: Vec<_> = symbols.clone().into_iter().collect();
            s.sort_by_key(|(addr, _)| *addr);
            s
        };

        Self {
            source_lines,
            source_locations,
            source_files_cache: HashMap::new(),
            symbols,
            sorted_symbols,
            function_params,
            syntax_set: syntect::parsing::SyntaxSet::load_defaults_newlines(),
            theme_set: syntect::highlighting::ThemeSet::load_defaults(),
        }
    }

    fn load_elf_symbols(
        path: &str,
    ) -> (
        HashMap<u64, String>,
        HashMap<u64, (String, u32)>,
        HashMap<u64, String>,
        HashMap<u64, Vec<String>>,
    ) {
        let mut source_map = HashMap::new();
        let mut source_locs = HashMap::new();
        let mut symbol_map = HashMap::new();
        let mut function_params = HashMap::new();
        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Warning: could not read ELF file: {}", e);
                return (source_map, source_locs, symbol_map, function_params);
            }
        };

        let obj = match object::File::parse(&*data) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Warning: could not parse ELF: {}", e);
                return (source_map, source_locs, symbol_map, function_params);
            }
        };

        use object::{Object, ObjectSection, ObjectSymbol};

        for sym in obj.symbols() {
            if sym.is_definition()
                && let Ok(name) = sym.name()
                && !name.is_empty()
                && !name.starts_with(".L")
            {
                let demangled = rustc_demangle::demangle(name).to_string();
                symbol_map.insert(sym.address(), demangled);
            }
        }
        let endian = if obj.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };

        let load_section = |id: gimli::SectionId| -> Result<
            gimli::EndianSlice<'_, gimli::RunTimeEndian>,
            gimli::Error,
        > {
            let section_data = obj
                .section_by_name(id.name())
                .and_then(|s| s.uncompressed_data().ok());
            let slice = match section_data {
                Some(std::borrow::Cow::Borrowed(bytes)) => bytes,
                Some(std::borrow::Cow::Owned(_)) => &[],
                None => &[],
            };
            Ok(gimli::EndianSlice::new(slice, endian))
        };

        let dwarf = match gimli::Dwarf::load(&load_section) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Warning: could not load DWARF sections: {}", e);
                return (source_map, source_locs, symbol_map, function_params);
            }
        };

        let mut iter = dwarf.units();
        while let Ok(Some(header)) = iter.next() {
            if let Ok(unit) = dwarf.unit(header) {
                let mut entries = unit.entries();
                let mut current_subprogram_addr = None;
                let mut depth = 0;
                while let Ok(Some((delta_depth, entry))) = entries.next_dfs() {
                    depth += delta_depth;
                    if entry.tag() == gimli::DW_TAG_subprogram {
                        if let Ok(Some(gimli::AttributeValue::Addr(addr))) =
                            entry.attr_value(gimli::DW_AT_low_pc)
                        {
                            current_subprogram_addr = Some(addr);
                            function_params.insert(addr, Vec::new());
                        } else {
                            current_subprogram_addr = None;
                        }
                    } else if entry.tag() == gimli::DW_TAG_formal_parameter {
                        if let Some(addr) = current_subprogram_addr
                            && let Ok(Some(attr)) = entry.attr_value(gimli::DW_AT_name)
                            && let Ok(s) = dwarf.attr_string(&unit, attr)
                            && let Ok(name) = s.to_string()
                            && let Some(params) = function_params.get_mut(&addr)
                        {
                            params.push(name.to_string());
                        }
                    } else if delta_depth <= 0 && depth <= 1 {
                        current_subprogram_addr = None;
                    }
                }
            }
        }

        let ctx = match addr2line::Context::from_dwarf(dwarf) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: could not build addr2line context: {}", e);
                return (source_map, source_locs, symbol_map, function_params);
            }
        };

        for section in obj.sections() {
            let addr = section.address();
            let size = section.size();
            if size == 0 {
                continue;
            }
            let mut offset = 0u64;
            while offset < size {
                let pc = addr + offset;
                if let Ok(Some(loc)) = ctx.find_location(pc)
                    && let (Some(file), Some(line)) = (loc.file, loc.line)
                {
                    let short: &str = file.rsplit(['/', '\\']).next().unwrap_or(file);
                    source_map.insert(pc, format!("{}:{}", short, line));
                    source_locs.insert(pc, (file.to_string(), line));
                }
                offset += 2;
            }
        }

        (source_map, source_locs, symbol_map, function_params)
    }

    pub fn get_source_file(
        &mut self,
        path: &str,
    ) -> Option<&Vec<Vec<(String, ratatui::style::Style)>>> {
        if !self.source_files_cache.contains_key(path) {
            let content = std::fs::read_to_string(path).ok();
            let parsed = content.map(|s| {
                let syntax = self
                    .syntax_set
                    .find_syntax_for_file(path)
                    .unwrap_or(None)
                    .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

                let theme = &self.theme_set.themes["base16-ocean.dark"];
                let mut h = syntect::easy::HighlightLines::new(syntax, theme);

                s.lines()
                    .map(|line| {
                        let ranges: Vec<(syntect::highlighting::Style, &str)> =
                            h.highlight_line(line, &self.syntax_set).unwrap_or_default();
                        ranges
                            .into_iter()
                            .map(|(style, text)| {
                                (text.to_string(), Self::syntect_style_to_ratatui(style))
                            })
                            .collect()
                    })
                    .collect()
            });
            self.source_files_cache.insert(path.to_string(), parsed);
        }
        self.source_files_cache.get(path).unwrap().as_ref()
    }

    pub fn map_addr_to_source(
        &self,
        target_addr: u64,
        local_entries: Option<&[DisasmEntry]>,
    ) -> Option<(String, u32)> {
        if let Some(loc) = self.source_locations.get(&target_addr) {
            return Some(loc.clone());
        }

        let mut sym_start = 0;
        let mut sym_end = u64::MAX;
        let search = self
            .sorted_symbols
            .binary_search_by_key(&target_addr, |(a, _)| *a);
        match search {
            Ok(idx) => {
                sym_start = self.sorted_symbols[idx].0;
                if idx + 1 < self.sorted_symbols.len() {
                    sym_end = self.sorted_symbols[idx + 1].0;
                }
            }
            Err(idx) => {
                if idx > 0 {
                    sym_start = self.sorted_symbols[idx - 1].0;
                }
                if idx < self.sorted_symbols.len() {
                    sym_end = self.sorted_symbols[idx].0;
                }
            }
        }

        if let Some(entries) = local_entries
            && let Some(abs_cursor) = entries.iter().position(|e| e.addr == target_addr)
        {
            for i in (0..=abs_cursor).rev() {
                if let Some(entry) = entries.get(i) {
                    if entry.addr < sym_start {
                        break;
                    }
                    if let Some(loc) = self.source_locations.get(&entry.addr) {
                        return Some(loc.clone());
                    }
                }
            }
            for i in (abs_cursor + 1)..entries.len() {
                if let Some(entry) = entries.get(i) {
                    if entry.addr >= sym_end {
                        break;
                    }
                    if let Some(loc) = self.source_locations.get(&entry.addr) {
                        return Some(loc.clone());
                    }
                }
            }
        }

        let mut best_addr = None;
        for &addr in self.source_locations.keys() {
            if addr <= target_addr
                && addr >= sym_start
                && (best_addr.is_none() || addr > best_addr.unwrap())
            {
                best_addr = Some(addr);
            }
        }
        best_addr.and_then(|addr| self.source_locations.get(&addr).cloned())
    }

    pub fn map_source_to_addr(&self, path: &str, line: u32, hw_pc: u64) -> Option<u64> {
        let mut best_addr = None;
        let mut min_diff = u64::MAX;
        for (addr, (loc_path, loc_line)) in &self.source_locations {
            if *loc_path == path && *loc_line == line {
                let diff = addr.abs_diff(hw_pc);
                if best_addr.is_none() || diff < min_diff {
                    min_diff = diff;
                    best_addr = Some(*addr);
                }
            }
        }
        best_addr
    }

    pub fn get_hw_pc_line(&self, path: &str, hw_pc: u64) -> Option<u32> {
        let mut best_addr = None;
        for &addr in self.source_locations.keys() {
            if addr <= hw_pc && (best_addr.is_none() || addr > best_addr.unwrap()) {
                best_addr = Some(addr);
            }
        }
        if let Some(addr) = best_addr
            && let Some((p, l)) = self.source_locations.get(&addr)
            && *p == path
        {
            Some(*l)
        } else {
            None
        }
    }
}
