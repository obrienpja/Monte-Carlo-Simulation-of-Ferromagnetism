use gnuplot::*;

pub fn plot(x: f64, y: f64, z: f64)
{
    let mut fg = Figure::new();
    fg.set_terminal("png", "fig.png");

    fg.axes3d()
        .set_title("3D lines + points", &[])
        .lines(vec![0, 1], vec![0, 1], vec![0, 1],
               &[LineWidth(5.0),
                   PointSymbol('o'),
                   Color("#ffaa77"),
                   PointSize(2.0),
                   ArrowType(Filled),
                   ArrowSize(10.2)]);


    fg.show();
}