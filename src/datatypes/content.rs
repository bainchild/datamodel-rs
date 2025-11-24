#[derive(Debug, Default, Clone)]
pub struct Content(Option<String>);
#[derive(Debug, Default, Clone)]
pub struct ContentId(Option<u64>);
// pub fn open_content(l: &Lua) -> mlua::Result<()> {
//     let tab = l.create_table_with_capacity(0, 12)?;
//     tab.set(
//         "new",
//         l.create_function(|_, (a, b): (f32, f32)| Ok(Vector2::new(a, b)))?,
//     )?;
//     tab.set_readonly(true);
//     tab.set_safeenv(true);
//     l.globals().set("Content", tab)?;
//     Ok(())
// }
