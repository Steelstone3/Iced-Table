use iced::{
    Border, Color, Element, Length, Renderer, Theme,
    widget::{Column, Container, Row, container::Style, text},
};

const TEXT_SIZE: u16 = 16;
const PADDING: u16 = 2;

#[derive(Default, Debug, Clone)]
pub struct Table {
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        let mut new_rows: Vec<Vec<String>> = vec![];
        new_rows.push(headers);
        for row in rows {
            new_rows.push(row);
        }

        Self { rows: new_rows }
    }

    pub fn add_headers(&mut self, headers: Vec<String>) {
        self.rows.insert(0, headers);
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn build<Message>(
        table_data: Table,
        color: Color,
        width: Option<f32>,
    ) -> Element<'static, Message>
    where
        Message: Clone + Send + Sync + 'static,
    {
        let mut table = Column::new();

        // with header and rows
        for row in table_data.rows {
            let row = Table::with_row(row, color);

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

    fn with_row<Message>(rows: Vec<String>, color: Color) -> Row<'static, Message>
    where
        Message: Clone + Send + Sync + 'static,
    {
        let mut data_row = vec![];
        for row in rows.into_iter() {
            data_row.push(
                Container::<Message, Theme, Renderer>::new(text(row).size(TEXT_SIZE))
                    .padding(PADDING)
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
