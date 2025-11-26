#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector2(pub f32, pub f32);
const VEC2_X: Vector2 = Vector2(1.0, 0.0);
const VEC2_Y: Vector2 = Vector2(0.0, 1.0);
const VEC2_ZERO: Vector2 = Vector2(0.0, 0.0);
const VEC2_ONE: Vector2 = Vector2(1.0, 1.0);
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2(x, y)
    }
    pub fn magnitude(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
    pub fn unit(&self) -> Vector2 {
        self / self.magnitude()
    }
    pub fn lerp(&self, other: &Vector2, alpha: f32) -> Vector2 {
        (other - self) * alpha
    }
}
impl std::ops::Div for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Self) -> Self::Output {
        Vector2(self.0 / rhs.0, self.1 / rhs.1)
    }
}
impl std::ops::Div for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Self) -> Self::Output {
        Vector2(self.0 / rhs.0, self.1 / rhs.1)
    }
}
impl std::ops::Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Self::Output {
        Vector2(self.0 / rhs, self.1 / rhs)
    }
}
impl std::ops::Div<f32> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Self::Output {
        Vector2(self.0 / rhs, self.1 / rhs)
    }
}
impl std::ops::Mul for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl std::ops::Mul for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl std::ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2(self.0 * rhs, self.1 * rhs)
    }
}
impl std::ops::Mul<f32> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2(self.0 * rhs, self.1 * rhs)
    }
}
impl std::ops::Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Add for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: f32) -> Self::Output {
        Vector2(self.0 + rhs, self.1 + rhs)
    }
}
impl std::ops::Add<f32> for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: f32) -> Self::Output {
        Vector2(self.0 + rhs, self.1 + rhs)
    }
}
impl std::ops::Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl std::ops::Sub for &Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl std::ops::Sub<f32> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: f32) -> Self::Output {
        Vector2(self.0 - rhs, self.1 - rhs)
    }
}
impl std::ops::Sub<f32> for &Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: f32) -> Self::Output {
        Vector2(self.0 - rhs, self.1 - rhs)
    }
}
#[cfg(feature = "mlua")]
use mlua::{AnyUserData, Lua, MetaMethod, UserData, UserDataFields, UserDataMethods};
#[cfg(feature = "mlua")]
impl UserData for Vector2 {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, vec| Ok(vec.0));
        fields.add_field_method_get("y", |_, vec| Ok(vec.1));
        fields.add_field_method_get("X", |_, vec| Ok(vec.0));
        fields.add_field_method_get("Y", |_, vec| Ok(vec.1));
        fields.add_field_method_get("Magnitude", |_, vec| Ok(vec.magnitude()));
        fields.add_field_method_get("Unit", |_, vec| Ok(vec.unit()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "lerp",
            |_, sel: &Vector2, (other, alpha): (AnyUserData, f32)| {
                let other2 = other.borrow::<Vector2>()?;
                Ok(Vector2::lerp(sel, &other2, alpha))
            },
        );
        methods.add_method(
            "Lerp",
            |_, sel: &Vector2, (other, alpha): (AnyUserData, f32)| {
                let other2 = other.borrow::<Vector2>()?;
                Ok(Vector2::lerp(sel, &other2, alpha))
            },
        );
        methods.add_meta_method(MetaMethod::Add, |_, sel: &Vector2, other: AnyUserData| {
            let other2 = other.borrow::<Vector2>()?;
            Ok(sel + &other2.to_owned())
        });
        methods.add_meta_method(MetaMethod::Sub, |_, sel: &Vector2, other: AnyUserData| {
            let other2 = other.borrow::<Vector2>()?;
            Ok(sel - &other2.to_owned())
        });
        methods.add_meta_method(MetaMethod::Mul, |_, sel: &Vector2, other: AnyUserData| {
            let other2 = other.borrow::<Vector2>()?;
            Ok(sel * &other2.to_owned())
        });
        methods.add_meta_method(MetaMethod::Div, |_, sel: &Vector2, other: AnyUserData| {
            let other2 = other.borrow::<Vector2>()?;
            Ok(sel / &other2.to_owned())
        });
        methods.add_meta_method(MetaMethod::ToString, |_, sel: &Vector2, other: ()| {
            Ok(format!("({}, {})", sel.0, sel.1))
        });
    }
}
#[cfg(feature = "mlua")]
pub fn open_vector2(l: &Lua) -> mlua::Result<()> {
    let tab = l.create_table_with_capacity(0, 5)?;
    tab.set(
        "new",
        l.create_function(|_, (a, b): (f32, f32)| Ok(Vector2::new(a, b)))?,
    )?;
    tab.set("xAxis", VEC2_X)?;
    tab.set("yAxis", VEC2_Y)?;
    tab.set("one", VEC2_ONE)?;
    tab.set("zero", VEC2_ZERO)?;
    tab.set_readonly(true);
    tab.set_safeenv(true);
    l.globals().set("Vector2", tab)?;
    Ok(())
}
