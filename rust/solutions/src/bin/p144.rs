// Project Euler 144: Laser beam bouncing in an elliptical white cell
fn main() {
    let mut x: f64 = 1.4;
    let mut y: f64 = -9.6;
    let mut dx: f64 = 1.4;
    let mut dy: f64 = -9.6 - 10.1;
    let mut bounces = 1;

    for _ in 0..10000 {
        // Normal to ellipse at (x,y): gradient of 4x^2+y^2 is (8x, 2y), proportional to (4x, y)
        let mut nx = 4.0 * x;
        let mut ny = y;
        let nlen = (nx * nx + ny * ny).sqrt();
        nx /= nlen;
        ny /= nlen;

        let dlen = (dx * dx + dy * dy).sqrt();
        dx /= dlen;
        dy /= dlen;

        let dot = dx * nx + dy * ny;
        let rx = dx - 2.0 * dot * nx;
        let ry = dy - 2.0 * dot * ny;
        dx = rx;
        dy = ry;

        // Find next intersection with ellipse: 4(x+t*dx)^2 + (y+t*dy)^2 = 100
        let a = 4.0 * dx * dx + dy * dy;
        let b = 8.0 * x * dx + 2.0 * y * dy;
        let c = 4.0 * x * x + y * y - 100.0;
        let disc = b * b - 4.0 * a * c;
        let sq = disc.sqrt();
        let t1 = (-b - sq) / (2.0 * a);
        let t2 = (-b + sq) / (2.0 * a);
        let t = if t1.abs() > t2.abs() { t1 } else { t2 };

        x += t * dx;
        y += t * dy;

        if x.abs() < 0.01 && y > 9.9 {
            break;
        }
        bounces += 1;
    }
    println!("{bounces}");
}
