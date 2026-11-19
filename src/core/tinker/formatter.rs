/// Formatter tabel ANSI ASCII box-drawing dan JSON pretty untuk Lumina Tinker REPL
pub struct TinkerFormatter;

impl TinkerFormatter {
    /// Render data kolom dan baris ke dalam format tabel box-drawing yang elegan
    pub fn render_table(columns: &[String], rows: &[Vec<String>]) -> String {
        if columns.is_empty() {
            return "(No columns)\n".to_string();
        }

        let num_cols = columns.len();
        let mut col_widths = vec![0usize; num_cols];

        for (i, col) in columns.iter().enumerate() {
            col_widths[i] = col.len();
        }

        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < num_cols {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }

        // Berikan padding 1 spasi kiri dan kanan
        let padded_widths: Vec<usize> = col_widths.iter().map(|w| w + 2).collect();

        let mut out = String::new();

        // 1. Garis Atas ┌───┬───┐
        out.push('┌');
        for (i, w) in padded_widths.iter().enumerate() {
            out.push_str(&"─".repeat(*w));
            if i + 1 < num_cols {
                out.push('┬');
            }
        }
        out.push_str("┐\n");

        // 2. Baris Header │ col1 │ col2 │
        out.push('│');
        for (i, col) in columns.iter().enumerate() {
            let width = col_widths[i];
            out.push_str(&format!(" {:<width$} ", col, width = width));
            out.push('│');
        }
        out.push('\n');

        // 3. Garis Pemisah Header ├───┼───┤
        out.push('├');
        for (i, w) in padded_widths.iter().enumerate() {
            out.push_str(&"─".repeat(*w));
            if i + 1 < num_cols {
                out.push('┼');
            }
        }
        out.push_str("┤\n");

        // 4. Baris Data
        if rows.is_empty() {
            let total_inner_width: usize = padded_widths.iter().sum::<usize>() + (num_cols - 1);
            let msg = "(no records found)";
            out.push('│');
            out.push_str(&format!(" {:<width$} ", msg, width = total_inner_width.saturating_sub(2)));
            out.push_str("│\n");
        } else {
            for row in rows {
                out.push('│');
                for (i, cell) in row.iter().enumerate() {
                    let width = col_widths.get(i).copied().unwrap_or(0);
                    out.push_str(&format!(" {:<width$} ", cell, width = width));
                    out.push('│');
                }
                out.push('\n');
            }
        }

        // 5. Garis Bawah └───┴───┘
        out.push('└');
        for (i, w) in padded_widths.iter().enumerate() {
            out.push_str(&"─".repeat(*w));
            if i + 1 < num_cols {
                out.push('┴');
            }
        }
        out.push_str("┘\n");

        out
    }

    /// Render nilai skalar sederhana dengan durasi
    pub fn render_scalar(value: &str, duration_ms: f64) -> String {
        format!("=> {}\n⚡ Executed in {:.2}ms\n", value, duration_ms)
    }

    /// Render JSON terformat indah
    pub fn render_json(val: &serde_json::Value, duration_ms: f64) -> String {
        let pretty = serde_json::to_string_pretty(val).unwrap_or_else(|_| val.to_string());
        format!("=> {}\n⚡ Executed in {:.2}ms\n", pretty, duration_ms)
    }
}
