#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

pub fn f2rgb(r: f64, g: f64, b: f64) -> Rgb {
    // make rgb from 3 floats
    Rgb {
        r: (r * 1.0 / 255.0) as u8,
        g: (g * 1.0 / 255.0) as u8,
        b: (b * 1.0 / 255.0) as u8,
        a: 255,
    }
}

pub fn f2hsv(h: f64, s: f64, v: f64) -> Hsv {
    // make hsv from 3 floats
    Hsv { h, s, v }
}

pub fn to_rgb(s: &str) -> Rgb {
    let r = u8::from_str_radix(&s[1..3], 16).unwrap();
    let g = u8::from_str_radix(&s[3..5], 16).unwrap();
    let b = u8::from_str_radix(&s[5..7], 16).unwrap();
    Rgb { r, g, b, a: 255 }
}

impl Rgb {
    pub fn to_hsv(self) -> Hsv {
        let r = 255.0 * f64::from(self.r);
        let g = 255.0 * f64::from(self.g);
        let b = 255.0 * f64::from(self.b);
        let min = f64::min(f64::min(r, g), b);
        let v = f64::max(f64::max(r, g), b);
        let c = v - min;
        let mut s = 0.0;
        if v != 0.0 {
            s = c / v;
        }
        let mut h = 0.0;
        if min != v {
            if v == r {
                h = (g - b) / c % 6.0;
            }
            if v == g {
                h = (b - r) / c + 2.0;
            }
            if v == b {
                h = (r - g) / c + 4.0;
            }
            h *= 60.0;
            if h < 0.0 {
                h += 360.0;
            }
        }
        f2hsv(h, s, v)
    }

    pub fn brighter_than(&self, ref_: &Rgb, delta: f64) -> Rgb {
        let primary = self.to_hsv();
        let secondary = ref_.to_hsv();
        if primary.v >= secondary.v + delta {
            return *self;
        }
        let mut primary = primary;
        primary.v = secondary.v + delta;
        if primary.v > 360.0 {
            primary.v = 360.0;
        }
        primary.to_rgb()
    }

    pub fn darker_than(&self, ref_: &Rgb, delta: f64) -> Rgb {
        let primary = self.to_hsv();
        let secondary = ref_.to_hsv();
        if primary.v <= secondary.v - delta {
            return *self;
        }
        let mut primary = primary;
        primary.v = secondary.v - delta;
        if primary.v < 0.0 {
            primary.v = 0.0;
        }
        primary.to_rgb()
    }

    pub fn brighter_or_darker_than(&self, ref_: &Rgb, delta: f64) -> Rgb {
        let primary = self.to_hsv();
        let secondary = ref_.to_hsv();
        if primary.v <= secondary.v {
            return self.darker_than(ref_, delta);
        }
        self.brighter_than(ref_, delta)
    }

    pub fn with_alpha(&self, alpha: f64) -> Rgb {
        Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
            a: (alpha * 255.0) as u8,
        }
    }

    pub fn html(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }
}

impl Hsv {
    pub fn to_rgb(&self) -> Rgb {
        let h = self.h as i32 / 60;
        let f = self.h / 60.0 - h as f64;
        let p = self.v * (1.0 - self.s);
        let q = self.v * (1.0 - self.s * f);
        let t = self.v * (1.0 - self.s * (1.0 - f));
        match h {
            0 | 6 => f2rgb(self.v, t, p),
            1 => f2rgb(q, self.v, p),
            2 => f2rgb(p, self.v, t),
            3 => f2rgb(p, q, self.v),
            4 => f2rgb(t, p, self.v),
            5 => f2rgb(self.v, p, q),
            _ => f2rgb(0.0, 0.0, 0.0),
        }
    }
}
