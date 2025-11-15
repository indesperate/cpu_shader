use std::error::Error;
use std::io::BufWriter;
use std::sync::Arc;
use std::{fs, io::Write};

use glam::{Vec2, Vec2Swizzles, Vec4};

struct Resolution {
    w: i32,
    h: i32,
    fps: i32,
}

fn write_image(frame: i32, res: &Resolution) -> Result<(), Box<dyn Error + Send + Sync>> {
    let output_path = format!("output_{:02}.ppm", frame);
    let file = fs::File::create(output_path)?;
    let mut f = BufWriter::new(file);
    let w = res.w;
    let h = res.h;
    writeln!(f, "P6")?;
    writeln!(f, "{} {}", w, h)?;
    writeln!(f, "255")?;
    let t = frame as f32 / res.fps as f32;
    let r = Vec2::new(w as f32, h as f32);
    for y in (0..h).rev() {
        for x in 0..w {
            let mut o = Vec4::default();
            let fc = Vec2::new(x as f32, y as f32);
            let p = (fc * 2. - r) / r.y;
            let mut l = Vec2::default();
            let mut i = Vec2::default();
            let wt = Vec4::new(-1., 1., 2., 0.);
            l += 4. - 4. * (0.7 - p.dot(p)).abs();
            let mut v = p * l;
            while i.y < 8. {
                i.y += 1.;
                v += (v.yx() * i.y + i + t).map(|x| x.cos()) / i.y + 0.7;
                o += (v.xyyx().map(|x| x.sin()) + 1.) * (v.x - v.y).abs();
            }
            o = (5. * (l.x - 4. - wt * p.y).map(|x| x.exp()) / o).map(|x| x.tanh());
            let color: &[u8; 3] = &[(o.x * 255.) as u8, (o.y * 255.) as u8, (o.z * 255.) as u8];
            f.write(color)?;
        }
    }
    f.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut handles = Vec::new();

    let seconds = 8;

    let res = Arc::new(Resolution {
        w: 1920,
        h: 1080,
        fps: 60,
    });

    for frame in 0..(seconds * res.fps) {
        let res_cloned = Arc::clone(&res);
        let handle = tokio::task::spawn_blocking(move || write_image(frame, &res_cloned));
        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }
    Ok(())
}
