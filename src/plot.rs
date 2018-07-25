use gnuplot::*;

pub fn plot(x: f64, y: f64, z: f64, seq: usize)
{
    let mut fg = Figure::new();
    fg.set_terminal("png", &format!("figs/fig{}.png", seq));

    fg.axes3d()
        .set_title("3D lines + points", &[])
        .lines(vec![0.0, x], vec![0.0, y], vec![0.0, z],
               &[LineWidth(5.0),
                   PointSymbol('o'),
                   Color("#ffaa77"),
                   PointSize(2.0),
                   ArrowType(Filled),
                   ArrowSize(1.2)]);


    fg.show();
}