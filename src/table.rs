/// Simple table renderer with Unicode box drawing characters
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        Self {
            headers: headers.into_iter().map(|s| s.to_string()).collect(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn render(&self) -> String {
        let col_widths = self.calculate_column_widths();
        let mut output = String::new();

        // Top border: ╭───┬───╮
        output.push_str(&self.render_border(&col_widths, '╭', '┬', '╮'));

        // Header row
        output.push_str(&self.render_row(&self.headers, &col_widths));

        // Header separator: ├───┼───┤
        output.push_str(&self.render_border(&col_widths, '├', '┼', '┤'));

        // Data rows
        for (i, row) in self.rows.iter().enumerate() {
            output.push_str(&self.render_row(row, &col_widths));
            // Add dotted separator between rows (not after last row)
            if i < self.rows.len() - 1 {
                output.push_str(&self.render_dotted_border(&col_widths));
            }
        }

        // Bottom border: ╰───┴───╯
        output.push_str(&self.render_border(&col_widths, '╰', '┴', '╯'));

        output
    }

    fn calculate_column_widths(&self) -> Vec<usize> {
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.len()).collect();

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }

        widths
    }

    fn render_dotted_border(&self, widths: &[usize]) -> String {
        let mut line = String::new();
        line.push('├');

        for (i, &width) in widths.iter().enumerate() {
            line.push_str(&"╌".repeat(width + 2));
            if i < widths.len() - 1 {
                line.push('┼');
            }
        }

        line.push('┤');
        line.push('\n');
        line
    }

    fn render_border(&self, widths: &[usize], left: char, mid: char, right: char) -> String {
        let mut line = String::new();
        line.push(left);

        for (i, &width) in widths.iter().enumerate() {
            line.push_str(&"─".repeat(width + 2));
            if i < widths.len() - 1 {
                line.push(mid);
            }
        }

        line.push(right);
        line.push('\n');
        line
    }

    fn render_row(&self, cells: &[String], widths: &[usize]) -> String {
        let mut line = String::new();
        line.push('│');

        for (i, width) in widths.iter().enumerate() {
            let cell = cells.get(i).map(|s| s.as_str()).unwrap_or("");
            line.push_str(&format!(" {:width$} │", cell, width = width));
        }

        line.push('\n');
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_table() {
        let mut table = Table::new(vec!["#", "Name"]);
        table.add_row(vec!["1".to_string(), "Alice".to_string()]);
        table.add_row(vec!["2".to_string(), "Bob".to_string()]);

        let output = table.render();
        assert!(output.contains("╭"));
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
    }

    #[test]
    fn test_column_width_calculation() {
        let mut table = Table::new(vec!["#", "Branch"]);
        table.add_row(vec!["0".to_string(), "feature/very-long-branch-name".to_string()]);

        let output = table.render();
        assert!(output.contains("feature/very-long-branch-name"));
    }
}
