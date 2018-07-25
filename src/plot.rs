use gnuplot::*;

pub fn plot(x: f64, y: f64, z: f64)
{
    let mut fg = Figure::new();
    fg.set_terminal("png", "fig.png");

    let mut xs: Vec<f64> = vec![0, x];
    let mut ys: Vec<f64> = vec![0, y];
    let mut zs: Vec<f64> = vec![0, z];

    fg.axes3d()
        .set_title("3D lines + points", &[])
        .lines(vec![0, xs], vec![0, ys], vec![0, zs],
               &[LineWidth(5.0),
                   PointSymbol('o'),
                   Color("#ffaa77"),
                   PointSize(2.0),
                   ArrowType(Filled),
                   ArrowSize(1.2)]);


    fg.show();
}