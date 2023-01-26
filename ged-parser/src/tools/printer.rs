use std::collections::HashMap;

use image_builder::{Color, Image, Position, Size, Text};

use crate::spreadsheets::general::{GeneralSpreadsheet, BACKLOG, NOTIFICATION};

pub fn comparative(
    current: &GeneralSpreadsheet,
    last: &HashMap<String, HashMap<String, String>>,
    data_type: &str,
) {
    let amount_columns = last.len() as i32;
    let column_size: i32 = 150;

    let x_second_column: i32 = 90;
    let x_third_column = x_second_column + column_size;
    let x_fourth_column = x_third_column + column_size;

    let y_column_headers = 40;
    let y_column_sub_headers = 60;

    let mut image = Image::new((5 + (amount_columns * x_fourth_column)) as u32, 235);

    let roboto_bold = Vec::from(include_bytes!("../fonts/Roboto-Bold.ttf") as &[u8]);
    image.add_custom_font("bold", roboto_bold);

    // PRINT SPREADSHEET NAME
    let binding = format!("Comparativo - {}", data_type.to_owned().to_uppercase());
    image.print_text(Text {
        content: &binding,
        size: 30,
        custom_font: Some("bold"),
        position: Position { x: 5, y: 5 },
    });

    // PRINT THE COLUMN HEADER
    for (i, column_header) in last.iter().enumerate() {
        let factor: i32 = i.try_into().unwrap();

        let mut dynamic_color = "gray";
        if BACKLOG.contains(&&column_header.0[..]) {
            dynamic_color = "green";
        } else if NOTIFICATION.contains(&&column_header.0[..]) {
            dynamic_color = "yellow";
        }

        image.print_rect(
            Position {
                x: x_second_column + (factor * x_fourth_column),
                y: y_column_headers,
            },
            Size {
                width: (column_size * 2) as u32,
                height: 18,
            },
            Color::Name(dynamic_color),
        );
        image.print_text(Text {
            content: column_header.0,
            size: 18,
            custom_font: Some("bold"),
            position: Position {
                x: x_second_column + 5 + (factor * x_fourth_column),
                y: 40,
            },
        });

        image.print_rect(
            Position {
                x: x_second_column + (factor * x_fourth_column) as i32,
                y: y_column_sub_headers,
            },
            Size {
                width: (column_size - 1) as u32,
                height: 16,
            },
            Color::Name("gray"),
        );
        image.print_text(Text {
            content: "Exec. Anterior",
            size: 16,
            custom_font: Some("bold"),
            position: Position {
                x: x_second_column + 5 + (factor * x_fourth_column),
                y: y_column_sub_headers,
            },
        });

        image.print_rect(
            Position {
                x: x_third_column + 1 + (factor * x_fourth_column),
                y: y_column_sub_headers,
            },
            Size {
                width: (column_size - 1) as u32,
                height: 16,
            },
            Color::Name(dynamic_color),
        );
        image.print_text(Text {
            content: "Exec. Atual",
            size: 16,
            custom_font: Some("bold"),
            position: Position {
                x: x_third_column + 6 + (factor * x_fourth_column),
                y: y_column_sub_headers,
            },
        });
    }

    // PRINT THE ROW HEADER
    let headers = &current.headers.columns[3..10];
    for (i, row_header) in headers.iter().enumerate() {
        let factor: i32 = i.try_into().unwrap();

        image.print_text(Text {
            content: row_header,
            size: 18,
            custom_font: Some("bold"),
            position: Position {
                x: 5,
                y: 80 + (22 * factor),
            },
        });

        for (x, row) in last.iter().enumerate() {
            // PRINT THE LAST CONTENT
            let mut printed = false;
            for (key, value) in row.1.iter() {
                if key == row_header {
                    image.print_text(Text {
                        content: value,
                        size: 18,
                        custom_font: Some("bold"),
                        position: Position {
                            x: x_second_column + 5 + ((x as i32) * x_fourth_column),
                            y: (80 + (22 * factor)) as i32,
                        },
                    });
                    printed = true;
                }
            }
            if !printed {
                image.print_text(Text {
                    content: "0",
                    size: 18,
                    custom_font: Some("bold"),
                    position: Position {
                        x: x_second_column + 5 + ((x as i32) * x_fourth_column),
                        y: (80 + (22 * factor)) as i32,
                    },
                });
            }
        }
    }

    for rows in last.iter() {
        let row_number = current
            .headers
            .rows
            .iter()
            .position(|cr| cr == rows.0)
            .unwrap();
        for (x, row) in current.content.iter().enumerate() {
            if x == row_number {
                let x = last
                    .iter()
                    .position(|column| column.0 == current.headers.rows[x])
                    .unwrap();
                let rows = &row[3..10];
                for (y, row) in rows.iter().enumerate() {
                    image.print_text(Text {
                        content: row,
                        size: 18,
                        custom_font: Some("bold"),
                        position: Position {
                            x: x_third_column + 5 + ((x as i32) * x_fourth_column),
                            y: (80 + (22 * y as i32)) as i32,
                        },
                    });
                }
            }
        }
    }

    image.save("comparison.png");
}

pub fn general(spreadsheet: &GeneralSpreadsheet, data_type: &str) {
    let mut image = Image::new(1220, 256);
    let roboto_bold = Vec::from(include_bytes!("../fonts/Roboto-Bold.ttf") as &[u8]);
    image.add_custom_font("bold", roboto_bold);

    let last_row = spreadsheet.size.rows - 1;
    let last_column = spreadsheet.size.columns - 1;

    // PRINT SPREADSHEET NAME
    let binding = data_type.to_owned().to_uppercase();
    image.print_text(Text {
        content: &binding,
        size: 50,
        custom_font: Some("bold"),
        position: Position { x: 550, y: 195 },
    });

    // PRINT THE ROW HEADER
    for (i, row_header) in spreadsheet.headers.rows.iter().enumerate() {
        let mut spacing = 20;
        let mut font_size = 14;
        let width = 1210;
        if i == last_row {
            spacing = 22;
            font_size = 16;
        }
        if BACKLOG.contains(row_header) {
            image.print_rect(
                Position {
                    x: 5,
                    y: 4 + (spacing * (i + 1)) as i32,
                },
                Size {
                    width,
                    height: font_size + 2,
                },
                Color::Name("green"),
            );
        }
        if NOTIFICATION.contains(row_header) {
            image.print_rect(
                Position {
                    x: 5,
                    y: 4 + (spacing * (i + 1)) as i32,
                },
                Size {
                    width,
                    height: font_size + 2,
                },
                Color::Name("yellow"),
            );
        }
        if row_header == &"Total Geral" {
            image.print_rect(
                Position {
                    x: 5,
                    y: 4 + (spacing * (i + 1)) as i32,
                },
                Size {
                    width,
                    height: font_size + 2,
                },
                Color::Name("gray"),
            );
        }
        image.print_text(Text {
            content: row_header,
            size: font_size,
            custom_font: Some("bold"),
            position: Position {
                x: 10,
                y: 5 + (spacing * (i + 1)) as i32,
            },
        })
    }

    // PRINT THE COLUMN HEADER
    for (i, column_header) in spreadsheet.headers.columns.iter().enumerate() {
        let mut spacing = 160;
        let mut font_size = 14;
        let mut y = 5;
        if i == last_column {
            spacing = 175;
            font_size = 16;
            y = 3;
        }
        image.print_text(Text {
            content: column_header,
            size: font_size,
            custom_font: Some("bold"),
            position: Position {
                x: spacing + (60 * (i + 1)) as i32,
                y,
            },
        });
    }

    // PRINT SPREADSHEET
    for (y, row) in spreadsheet.content.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let mut part_of_x = 160;
            let mut factor_of_y = 20;
            let mut size = 14;
            let mut custom_font = None;

            if y == last_row {
                factor_of_y = 22;
                size += 2;
                custom_font = Some("bold");
            }
            if x == last_column {
                part_of_x = 175;
                size += 2;
                custom_font = Some("bold");
            }

            image.print_text(Text {
                content: &value,
                size,
                custom_font,
                position: Position {
                    x: part_of_x + (60 * (x + 1)) as i32,
                    y: 5 + (factor_of_y * (y + 1)) as i32,
                },
            });
        }
    }

    // PRINT BACKLOG
    {
        let y = 205;
        let size = 20;
        image.print_rect(
            Position { x: 5, y },
            Size {
                width: 400,
                height: size,
            },
            Color::Name("green"),
        );
        image.print_text(Text {
            content: "Backlog:",
            size,
            custom_font: Some("bold"),
            position: Position { x: 10, y },
        });
        image.print_text(Text {
            content: &spreadsheet.backlog,
            size,
            custom_font: Some("bold"),
            position: Position { x: 300, y },
        });
    }

    // PRINT NOTIFICATION
    {
        let y = 230;
        let size = 20;
        image.print_rect(
            Position { x: 5, y },
            Size {
                width: 400,
                height: size,
            },
            Color::Name("yellow"),
        );
        image.print_text(Text {
            content: "Notification:",
            size,
            custom_font: Some("bold"),
            position: Position { x: 10, y },
        });
        image.print_text(Text {
            content: &spreadsheet.notification,
            size,
            custom_font: Some("bold"),
            position: Position { x: 300, y },
        });
    }

    image.save(&format!("{}.png", data_type));
}
