use crate::color;

/// A cell with display text and actual width (for color support)
pub struct Cell {
    pub text: String,
    pub width: usize,
}

impl Cell {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            width: text.len(),
        }
    }

    pub fn colored(text: String, width: usize) -> Self {
        Self { text, width }
    }
}

/// Simple table renderer with Unicode box drawing characters
pub struct Table {
    headers: Vec<Cell>,
    rows: Vec<Vec<Cell>>,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        Self {
            headers: headers
                .into_iter()
                .map(|s| Cell::colored(color::green(s), s.len()))
                .collect(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<Cell>) {
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
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.width).collect();

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.width);
                }
            }
        }

        widths
    }

    fn render_dotted_border(&self, widths: &[usize]) -> String {
        let mut line = String::new();
        line.push_str(&color::gray("├"));

        for (i, &width) in widths.iter().enumerate() {
            line.push_str(&color::gray(&"╌".repeat(width + 2)));
            if i < widths.len() - 1 {
                line.push_str(&color::gray("┼"));
            }
        }

        line.push_str(&color::gray("┤"));
        line.push('\n');
        line
    }

    fn render_border(&self, widths: &[usize], left: char, mid: char, right: char) -> String {
        let mut line = String::new();
        line.push_str(&color::gray(&left.to_string()));

        for (i, &width) in widths.iter().enumerate() {
            line.push_str(&color::gray(&"─".repeat(width + 2)));
            if i < widths.len() - 1 {
                line.push_str(&color::gray(&mid.to_string()));
            }
        }

        line.push_str(&color::gray(&right.to_string()));
        line.push('\n');
        line
    }

    fn render_row(&self, cells: &[Cell], widths: &[usize]) -> String {
        let mut line = String::new();
        line.push_str(&color::gray("│"));

        for (i, width) in widths.iter().enumerate() {
            let cell = cells.get(i);
            let text = cell.map(|c| c.text.as_str()).unwrap_or("");
            let cell_width = cell.map(|c| c.width).unwrap_or(0);
            let padding = width - cell_width;
            line.push_str(&format!(" {}{} ", text, " ".repeat(padding)));
            line.push_str(&color::gray("│"));
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
        table.add_row(vec![Cell::new("1"), Cell::new("Alice")]);
        table.add_row(vec![Cell::new("2"), Cell::new("Bob")]);

        let output = table.render();
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
    }

    #[test]
    fn test_column_width_calculation() {
        let mut table = Table::new(vec!["#", "Branch"]);
        table.add_row(vec![
            Cell::new("0"),
            Cell::new("feature/very-long-branch-name"),
        ]);

        let output = table.render();
        assert!(output.contains("feature/very-long-branch-name"));
    }
}
