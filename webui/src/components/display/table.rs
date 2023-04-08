use crate::prelude::*;

pub struct TableColumns<T> {
    name: String,
    align: LeftCenterRight,
    cell: fn(&T) -> Html,
}
impl<T> TableColumns<T> {
    pub fn new(name: String, cell: fn(&T) -> Html) -> Self {
        Self {
            name,
            align: LeftCenterRight::Left,
            cell,
        }
    }
    pub fn align(&mut self, align: LeftCenterRight) -> TableColumns<T> {
        self.align = align;
        Self {
            name: self.name.to_string(),
            align,
            cell: self.cell,
        }
    }
}

pub struct Table<T> {
    columns: Vec<TableColumns<T>>,
    class: Classes,
    color: Theme,
    elevation: u8,
}

impl<T> Table<T> {
    pub fn new(columns: Vec<TableColumns<T>>) -> Self {
        Self {
            columns,
            class: Classes::new(),
            color: Theme::Primary,
            elevation: 0,
        }
    }
    pub fn add_class(&mut self, class: String) -> &mut Self {
        self.class.push(class);
        self
    }
    pub fn bordered(&mut self) -> &mut Self {
        self.class.push("bordered");
        self
    }
    pub fn elevation(&mut self, elevation: u8) -> &mut Self {
        self.elevation = elevation;
        self
    }
    pub fn render(&mut self, data: Vec<T>) -> Html {
        if self.elevation > 0 {
            self.class.push(format!("elevation-{}", self.elevation));
        }
        self.class.push(format!("table-{}", self.color));
        html!(
            <table class={self.class.to_string()}>
                <thead>
                    <tr>
                        {self.columns.iter().map(|column| {
                            html!{<th class={format!("{}", column.align)}>{column.name.to_string()}</th>}
                        }).collect::<Html>()}
                    </tr>
                </thead>
                <tbody>
                    {data.iter().map(move |data| {
                        html!{
                            <tr>
                            {self.columns.iter().map(|column| {
                                html!{<td class={format!("{}", column.align)}>{(column.cell)(data)}</td>}
                            }).collect::<Html>()}
                            </tr>
                        }
                    }).collect::<Html>()}
                </tbody>
            </table>
        )
    }
}
