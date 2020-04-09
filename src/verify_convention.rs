use crate::conventions::{datapack_advancement, Reporter, Files, Diagnostic};

pub enum VerifyConvention {
	DatapackAdvancement(datapack_advancement::Verify)
}

impl Reporter for VerifyConvention {
	type Info = ();

	fn report(&self, _: Self::Info) -> Option<(Files, Diagnostic)> {
		match self {
			Self::DatapackAdvancement(inner) => inner.report(())
		}
	}
}

impl From<datapack_advancement::Verify> for VerifyConvention {
	fn from(item: datapack_advancement::Verify) -> Self {
		VerifyConvention::DatapackAdvancement(item)
	}
}