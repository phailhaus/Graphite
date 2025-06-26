use dyn_any::DynAny;

/// Represents different ways of rendering an object
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash, DynAny, specta::Type)]
pub enum ViewMode {
	/// Render with normal coloration at the current viewport resolution
	#[default]
	Normal,
	/// Render only the outlines of shapes at the current viewport resolution
	Outline,
	/// Render with normal coloration at the document resolution, showing the pixels when the current viewport resolution is higher
	Pixels,
}
