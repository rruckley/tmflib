//! Create Service Candidate Example

use tmflib::tmf633::service_candidate::ServiceCandidate;
use tmflib::tmf633::service_category::{ServiceCategory,ServiceCategoryRef};
use tmflib::tmf633::service_specification::{ServiceSpecification,ServiceSpecificationRef};

fn main() {

    let cat = ServiceCategory::new("A Service Category");
    let spec = ServiceSpecification::new("A Service Specification");
    let candidate = ServiceCandidate::new("A Service Candidate",ServiceSpecificationRef::from(spec))
        .category(ServiceCategoryRef::from(cat));

    dbg!(candidate);
}