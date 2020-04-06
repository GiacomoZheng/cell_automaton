use orbtk::prelude::*;

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Grid::create()
                .columns(
                    Columns::create()
                        .column("*")
                        .column("*")
                        .column("*")
                        // .column("auto")
                        // .column(50.0)
                        .build(),
                )
                .rows(
                    Rows::create()
                        .row("*")
                        .row("*")
                        .build())
                .child(
                    Grid::create()
                        // .selector("lynch")
                        .margin((10.0, 0.0, 0.0, 4.0))
                        .attach(Grid::column(0))
                        .attach(Grid::row(0))
                        .child(
                            // TextBlock::create()
                            //     .text("(0,0)")
                            //     // .selector("light-text")
                            //     .horizontal_alignment("center")
                            //     .vertical_alignment("center")
                            //     .build(ctx),
                            ToggleButton::create()
                                .horizontal_alignment("center")
                                .vertical_alignment("center")
                                .build(ctx)
                        )
                        .build(ctx),
                )
                .child(
                    Grid::create()
                        // .selector("bluebayoux")
                        .margin(10.0)
                        .constraint(Constraint::create().width(150.0).build())
                        .attach(Grid::column(1))
                        .attach(Grid::row(0))
                        .child(
                            TextBlock::create()
                                .text("(1,0)")
                                // .selector("white")
                                .horizontal_alignment("center")
                                .vertical_alignment("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Grid::create()
                        // .selector("linkwater")
                        .attach(Grid::column(2))
                        .attach(Grid::row(0))
                        .child(
                            TextBlock::create()
                                .text("(2,0)")
                                // .selector("linkwater")
                                .horizontal_alignment("center")
                                .vertical_alignment("center")
                                .build(ctx),
                        )
                        .build(ctx)
                )
                .child(
                    Grid::create()
                        // .selector("goldendream")
                        .attach(Grid::column(0))
                        .attach(Grid::row(1))
                        .attach(Grid::column_span(3))
                        .child(
                            TextBlock::create()
                                .text("(0,1) - ColumnSpan 3")
                                // .selector("goldendream")
                                .horizontal_alignment("center")
                                .vertical_alignment("center")
                                .build(ctx)
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

