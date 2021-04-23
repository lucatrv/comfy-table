use crossterm::style::{Attribute, Color};

use crate::style::CellAlignment;

/// A stylable table cell with content.
#[derive(Clone, Debug)]
pub struct Cell {
    /// The content is a list of strings.\
    /// This is done to make working with newlines more easily.\
    /// When creating a new [Cell], the given content is split by newline.
    pub(crate) content: Vec<String>,
    /// The delimiter which is used to split the text into consistent pieces.\
    /// The default is ` `.
    pub(crate) delimiter: Option<char>,
    pub(crate) alignment: Option<CellAlignment>,
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
    pub(crate) attributes: Vec<Attribute>,
}

impl Cell {
    /// Create a new Cell
    pub fn new<T: ToString>(content: T) -> Self {
        Cell {
            content: content
                .to_string()
                .split('\n')
                .map(|content| content.to_string())
                .collect(),
            delimiter: None,
            alignment: None,
            fg: None,
            bg: None,
            attributes: Vec::new(),
        }
    }

    /// Return a copy of the content contained in this cell.
    pub fn get_content(&self) -> String {
        self.content.join("\n")
    }

    /// Set the delimiter used to split text for this cell. \
    /// Normal text uses spaces (` `) as delimiters. This is necessary to help comfy-table
    /// understand the concept of _words_.
    pub fn set_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = Some(delimiter);

        self
    }

    /// Set the alignment of content for this cell.
    ///
    /// Setting this overwrites alignment settings of the
    /// [Column](crate::column::Column::set_cell_alignment) for this specific cell.
    /// ```
    /// use comfy_table::CellAlignment;
    /// use comfy_table::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .set_alignment(CellAlignment::Center);
    /// ```
    pub fn set_alignment(mut self, alignment: CellAlignment) -> Self {
        self.alignment = Some(alignment);

        self
    }

    /// Set the foreground text color for this cell.
    ///
    /// comfy-table uses [Crossterm Colors](crossterm::style::Color).
    /// Look at their documentation for all possible Colors.
    /// ```
    /// use comfy_table::Color;
    /// use comfy_table::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .fg(Color::Red);
    /// ```
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);

        self
    }

    /// Set the background color for this cell.
    ///
    /// comfy-table uses [Crossterm Colors](crossterm::style::Color).
    /// Look at their documentation for all possible Colors.
    /// ```
    /// use comfy_table::Color;
    /// use comfy_table::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .bg(Color::Red);
    /// ```
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);

        self
    }

    /// Add a styling attribute to the content cell.\
    /// Those can be **bold**, _italic_, blinking and many more.
    ///
    /// comfy-table uses [Crossterm Attributes](crossterm::style::Attribute).
    /// Look at their documentation for all possible [Attributes](Attribute).
    /// ```
    /// use comfy_table::Attribute;
    /// use comfy_table::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .add_attribute(Attribute::Bold);
    /// ```
    pub fn add_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);

        self
    }

    /// Same as add_attribute, but you can pass a vector of [Attributes](Attribute)
    pub fn add_attributes(mut self, mut attribute: Vec<Attribute>) -> Self {
        self.attributes.append(&mut attribute);

        self
    }
}

impl<T: ToString> From<T> for Cell {
    /// Convert to a new [Cell].
    ///
    /// ```
    /// # use comfy_table::Cell;
    /// let cell: Cell = "content".into();
    /// ```
    fn from(content: T) -> Cell {
        Cell::new(content)
    }
}

/// Allow the conversion of a type to a vector of cells.
///
/// By default this is implemented for all types implementing
/// [IntoIterator] where the iterated Item type implements [ToString].\
/// E.g. a `Vec<i32>` works.
pub trait ToCells {
    fn to_cells(self) -> Vec<Cell>;
}

impl<T: IntoIterator> ToCells for T
where
    T::Item: ToCell,
{
    fn to_cells(self) -> Vec<Cell> {
        self.into_iter().map(|item| item.to_cell()).collect()
    }
}

/// Allow the conversion of a type to a [Cell].
///
/// By default this is implemented for all types implementing [ToString].
pub trait ToCell {
    fn to_cell(self) -> Cell;
}

impl<T: ToString> ToCell for T {
    fn to_cell(self) -> Cell {
        Cell::new(self.to_string())
    }
}

impl ToCell for Cell {
    fn to_cell(self) -> Cell {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_generation() {
        let content = "This is\nsome multiline\nstring".to_string();
        let cell = Cell::new(content.clone());

        assert_eq!(cell.get_content(), content);
    }
}
