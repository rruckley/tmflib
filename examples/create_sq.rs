//! Create Service Qualification Example

#[cfg(feature = "tmf641")]
use tmflib::tmf641::service_order_item::ServiceRefOrValue;
#[cfg(feature = "tmf645")]
use tmflib::tmf645::check_service_qualification::CheckServiceQualification;
#[cfg(feature = "tmf645")]
use tmflib::tmf645::check_service_qualification::CheckServiceQualificationItem;
use tmflib::HasDescription;

fn main() {
    #[cfg(feature = "tmf645")]
    {
        let alternate = ServiceRefOrValue::default().description("Alternate Service");
        let mut item = CheckServiceQualificationItem::default()
            .description("Check for Internet Service [Item]");
        item.reason("code", "label");
        item.alternate(alternate);
        let sq = CheckServiceQualification::new("Qualification")
            .item(item)
            .description("Check for Service");

        dbg!(sq);
    }
}
