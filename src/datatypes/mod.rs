mod brickcolor;
pub use brickcolor::*;
mod color3;
pub use color3::*;
mod content;
pub use content::*;
mod datetime;
pub use datetime::*;
mod vector2;
pub use vector2::*;
#[cfg(feature = "mlua")]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3(pub mlua::Vector);
#[cfg(not(feature = "mlua"))]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3;
// possible stub zone
#[derive(Debug, Default, Clone, Copy)]
pub struct TweenInfo {
    pub time: f32,
    pub easing_style: crate::generated::EnumEasingStyle,
    pub easing_direction: crate::generated::EnumEasingDirection,
    pub repeat_count: f32,
    pub reverses: bool,
    pub delay_time: f32
}
#[derive(Debug, Default, Clone, Copy)]
pub struct QDir;
#[derive(Debug, Default, Clone, Copy)]
pub struct QFont;
#[derive(Debug, Default, Clone, Copy)]
pub struct CSGPropertyData;
#[derive(Debug, Default, Clone, Copy)]
pub struct FacsReplicationData;
// definitely should not be stub zone start
#[derive(Debug, Default, Clone, Copy)]
pub struct BString;
#[derive(Debug, Default, Clone, Copy)]
pub struct NetAssetRef;
#[derive(Debug, Default, Clone, Copy)]
pub struct SharedString;
#[derive(Debug, Default, Clone, Copy)]
pub struct UniqueId;
#[derive(Debug, Default, Clone, Copy)]
pub struct SecurityCapabilities;
#[derive(Debug, Default, Clone, Copy)]
pub struct Font;
#[derive(Debug, Default, Clone, Copy)]
pub struct PhysicalProperties;

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray(pub Vector3, pub Vector3);
#[derive(Debug, Default, Clone, Copy)]
pub struct Rect;
#[derive(Debug, Default, Clone, Copy)]
pub struct UDim(pub f32, pub f32);
#[derive(Debug, Default, Clone, Copy)]
pub struct UDim2(pub UDim, pub UDim);

#[derive(Debug, Default, Clone, Copy)]
pub struct CFrame;
#[derive(Debug, Default, Clone, Copy)]
pub struct Axes {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub back: bool,
    pub front: bool
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Faces {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub back: bool,
    pub front: bool
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3int16(pub i16, pub i16, pub i16);
#[derive(Debug, Default, Clone, Copy)]
pub struct Region3int16(pub Vector3int16, pub Vector3int16);
//
#[derive(Debug, Default, Clone, Copy)]
pub struct ReplicationPV(pub CFrame, pub Vector3);
#[derive(Debug, Default, Clone, Copy)]
pub struct Path2DControlPoint(pub UDim2, pub Option<UDim2>, pub Option<UDim2>);
#[derive(Debug, Default, Clone, Copy)]
pub struct Color3uint8(pub u8, pub u8, pub u8);
#[derive(Debug, Default, Clone, Copy)]
pub struct NumberRange(pub f32, pub f32);
#[derive(Debug, Default, Clone, Copy)]
pub struct ColorSequenceKeypoint(pub f32, pub Color3);
#[derive(Debug, Default, Clone)]
pub struct ColorSequence(pub Vec<ColorSequenceKeypoint>);
#[derive(Debug, Default, Clone, Copy)]
pub struct NumberSequenceKeypoint {
    pub envelope: f32,
    pub time: f32,
    pub value: f32
}
#[derive(Debug, Default, Clone)]
pub struct NumberSequence(pub Vec<NumberSequenceKeypoint>);
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
#[cfg(feature = "mlua")]
pub fn open_rblx_datatypes(l: &mlua::Lua) -> mlua::Result<()> {
    open_vector2(l)?;
    open_datetime(l)?;
    Ok(())
}
