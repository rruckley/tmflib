//! Create Service Qualification Example

#[cfg(feature = "tmf645")]
use tmflib::tmf645::check_service_qualification::CheckServiceQualification;

fn main() {
    #[cfg(feature = "tmf645")]
    {
        let sq = CheckServiceQualification::new("Qualification");

        dbg!(sq);
    }
}