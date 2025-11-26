use chrono::DateTime as ChronoDateTime;
#[derive(Debug, Clone, Default, Copy)]
pub struct DateTime(pub ChronoDateTime<chrono::Utc>);
#[cfg(feature = "mlua")]
pub fn open_datetime(l: &mlua::Lua) -> mlua::Result<()> {
    Ok(())
}
