use graphene_core::math::bbox::AxisAlignedBbox;
use graphene_core::transform::Footprint;

pub trait FootprintExt {
	fn viewport_bounds_in_local_space(&self) -> AxisAlignedBbox;
}

impl FootprintExt for Footprint {
	fn viewport_bounds_in_local_space(&self) -> AxisAlignedBbox {
		let inverse = self.transform.inverse();
		let start = inverse.transform_point2((0., 0.).into());
		let end = inverse.transform_point2(self.resolution.as_dvec2());
		AxisAlignedBbox { start, end }
	}
}
