mod brickcolor;
pub use brickcolor::*;
mod content;
pub use content::*;
mod datetime;
pub use datetime::*;
mod vector2;
pub use vector2::*;
// possible stub zone
#[derive(Debug, Default, Clone, Copy)]
pub struct TweenInfo;
#[derive(Debug, Default, Clone, Copy)]
pub struct QDir;
#[derive(Debug, Default, Clone, Copy)]
pub struct QFont;
#[derive(Debug, Default, Clone, Copy)]
pub struct CSGPropertyData;
#[derive(Debug, Default, Clone, Copy)]
pub struct Path2DControlPoint;
#[derive(Debug, Default, Clone, Copy)]
pub struct FacsReplicationData;
#[derive(Debug, Default, Clone, Copy)]
pub struct ReplicationPV;
// definitely should not be stub zone start
#[derive(Debug, Default, Clone, Copy)]
pub struct NetAssetRef;
#[derive(Debug, Default, Clone, Copy)]
pub struct SharedString;
#[derive(Debug, Default, Clone, Copy)]
pub struct UniqueId;
#[derive(Debug, Default, Clone, Copy)]
pub struct SecurityCapabilities;
#[derive(Debug, Default, Clone, Copy)]
pub struct Rect;
#[derive(Debug, Default, Clone, Copy)]
pub struct Axes;
#[derive(Debug, Default, Clone, Copy)]
pub struct Font;
#[derive(Debug, Default, Clone, Copy)]
pub struct Ray;
#[derive(Debug, Default, Clone, Copy)]
pub struct PhysicalProperties;
#[derive(Debug, Default, Clone, Copy)]
pub struct Faces;
#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3int16(pub (i16, i16, i16));
#[derive(Debug, Default, Clone, Copy)]
pub struct Region3int16(pub (Vector3int16, Vector3int16));
#[derive(Debug, Default, Clone, Copy)]
pub struct Color3uint8(u8);
#[derive(Debug, Default, Clone, Copy)]
pub struct NumberRange(pub (f32, f32));
#[derive(Debug, Default, Clone, Copy)]
pub struct UDim(pub (f32, f32));
#[derive(Debug, Default, Clone, Copy)]
pub struct UDim2(pub (UDim, UDim));
#[derive(Debug, Default, Clone, Copy)]
pub struct CFrame;
#[derive(Debug, Default, Clone, Copy)]
pub struct Color3;
#[derive(Debug, Default, Clone, Copy)]
pub struct ColorSequence;
#[derive(Debug, Default, Clone, Copy)]
pub struct NumberSequence;
// end stub zone
#[derive(Debug, Clone, Copy)]
pub struct SystemAddress(std::net::IpAddr);
impl Default for SystemAddress {
    fn default() -> Self {
        SystemAddress("127.0.0.1".parse().unwrap())
    }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct OptionalCoordinateFrame(Option<CFrame>);
pub fn open_rblx_datatypes(l: &mlua::Lua) -> mlua::Result<()> {
    open_vector2(l)?;
    open_datetime(l)?;
    Ok(())
}
