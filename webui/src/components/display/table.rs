use crate::*;

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
    pub fn align(self: &mut Self, align: LeftCenterRight) -> &mut Self {
        self.align = align;
        self
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
    pub fn add_class(self: &mut Self, class: String) -> &mut Self {
        self.class.push(class.to_string());
        self
    }
    pub fn bordered(self: &mut Self) -> &mut Self {
        self.class.push("bordered");
        self
    }
    pub fn elevation(self: &mut Self, elevation: u8) -> &mut Self {
        self.elevation = elevation;
        self
    }
    pub fn render(self: &mut Self, data: Vec<T>) -> Html {
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
                        self.columns.iter().map(|column| {
                            html!{<td class={format!("{}", column.align)}>{(column.cell)(data)}</td>}
                        }).collect::<Html>()
                    }).collect::<Html>()}
                </tbody>
            </table>
        )
    }
}
