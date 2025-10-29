use iced::{
    Border, Color, Element, Length, Renderer, Theme,
    widget::{Column, Container, Row, container::Style, text},
};

#[derive(Default, Debug, Clone)]
pub struct Table {
    pub rows: Vec<Vec<String>>,
}

impl Table {
    /// Typically use Table::default() then build using add_headers() and add_row()
    /// However can provide a list of headers and rows to create Table
    pub fn new(headers: Vec<&str>, rows: Vec<Vec<&str>>) -> Self {
        let mut new_rows: Vec<Vec<String>> = vec![];
        new_rows.push(owned_rows(headers));
        for row in rows {
            new_rows.push(owned_rows(row));
        }

        Self { rows: new_rows }
    }

    /// Add a row of headers to the table at the top
    pub fn add_headers(&mut self, headers: Vec<&str>) {
        self.rows.insert(0, owned_rows(headers));
    }

    /// Add a data row to the table
    pub fn add_row(&mut self, row: Vec<&str>) {
        self.rows.push(owned_rows(row));
    }

    /// Add data rows to the table
    /// Type must implement "to_string()"
    pub fn add_rows<T>(&mut self, rows: Vec<Vec<T>>)
    where
        T: ToString,
    {
        for row_of_type in rows {
            let row = row_of_type
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>();

            self.rows.push(row);
        }
    }

    /// Constructs a visually styled table component from structured data.
    ///
    /// This function iterates over the provided `Table` data, applying optional
    /// styling parameters like `color`, `text_size`, and `padding` to each row.
    /// The resulting table is wrapped in a `Container`, allowing for flexible width
    /// control using the `width` parameter.
    ///
    /// # Type Parameters
    ///
    /// * `Message`: The application's message type, used for handling user interaction events.
    ///
    /// # Arguments
    ///
    /// * `table_data`: The source data for the table, expected to contain a collection of rows.
    /// * `color`: An optional `Color` to apply primary styling to the table or its elements. None sets the color to black. Consider "self.theme().palette().text" as a reasonable default.
    /// * `width`: An optional fixed width (`f32`) for the container. If `None`, the container
    ///   will fill the available space (`Length::Fill`).
    /// * `text_size`: An optional font size (`u16`) for the text within the table cells. If `None` this will be set to size 16
    /// * `padding`: An optional padding amount (`u16`) to apply around individual cells or rows. If `None` this will be set to padding 2
    ///
    /// # Returns
    ///
    /// An `Element<'static, Message>` representing the complete, iced table component
    pub fn build<Message>(
        table_data: Table,
        color: Option<Color>,
        width: Option<f32>,
        text_size: Option<u16>,
        padding: Option<u16>,
    ) -> Element<'static, Message>
    where
        Message: Clone + Send + Sync + 'static,
    {
        let mut table = Column::new();

        // with header and rows
        for row in table_data.rows {
            let row = Table::with_row(referenced_rows(&row), color, text_size, padding);

            table = table.push(row);
        }

        match width {
            Some(width) => Container::new(table)
                .width(Length::Fixed(width))
                .padding(10)
                .into(),
            None => Container::new(table).width(Length::Fill).padding(10).into(),
        }
    }

    fn with_row<Message>(
        rows: Vec<&str>,
        color: Option<Color>,
        text_size: Option<u16>,
        padding: Option<u16>,
    ) -> Row<'static, Message>
    where
        Message: Clone + Send + Sync + 'static,
    {
        let mut data_row = vec![];
        let rows = owned_rows(rows);

        let color = match color {
            Some(color) => color,
            None => Color::from_rgb(0.0, 0.0, 0.0),
        };

        let text_size = text_size.unwrap_or(16);

        let padding = padding.unwrap_or(2);

        for row in rows.into_iter() {
            data_row.push(
                Container::<Message, Theme, Renderer>::new(text(row).size(text_size))
                    .padding(padding)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .max_height(22)
                    .style(move |_| table_theme(color))
                    .into(),
            );
        }

        // build the data row
        Row::with_children(data_row)
    }
}

fn table_theme(color: Color) -> Style {
    Style {
        text_color: Some(color),
        background: Default::default(),
        border: Border {
            color,
            width: 1.0,
            radius: Default::default(),
        },
        shadow: Default::default(),
    }
}

fn owned_rows(rows: Vec<&str>) -> Vec<String> {
    let row_strings: Vec<String> = rows.iter().map(|s| s.to_string()).collect();

    row_strings
}

fn referenced_rows(rows: &[String]) -> Vec<&str> {
    rows.iter().map(|s| s.as_str()).collect()
}
