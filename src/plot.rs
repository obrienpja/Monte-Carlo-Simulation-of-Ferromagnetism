use gnuplot::*;

pub fn example()
{
    let zw = 61;
    let zh = 61;
    let mut z1 = Vec::with_capacity((zw * zh) as usize);
    for i in 0..zh
        {
            for j in 0..zw
                {
                    let y = 8.0 * (i as f64) / zh as f64 - 8.0;
                    let x = 8.0 * (j as f64) / zw as f64 - 4.0;
                    z1.push(x.cos() * y.cos() / ((x * x + y * y).sqrt() + 1.0));
                }
        }

    let mut fg = Figure::new();
    fg.set_terminal("png", "fig.png");

    fg.axes3d()
        .set_title("Custom Palette fg4.5", &[])
        .surface(z1.iter(), zw, zh, Some((-40.0, -40.0, 4.0, 49.0)), &[])
        .set_x_label("X", &[])
        .set_y_label("Y", &[])
        .set_z_label("Z", &[])
        .set_z_range(Fix(-1.0), Fix(1.0))
        .set_z_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
        .set_view(45.0, 45.0);

    fg.show();
}