//! Create Service Qualification Example

#[cfg(feature = "tmf645")]
use tmflib::tmf645::check_service_qualification::CheckServiceQualificationItem;
#[cfg(feature = "tmf645")]
use tmflib::tmf645::check_service_qualification::CheckServiceQualification;
#[cfg(feature = "tmf641")]
use tmflib::tmf641::service_order_item::ServiceRefOrValue;

fn main() {
    #[cfg(feature = "tmf645")]
    {
        let mut alternate = ServiceRefOrValue::default();
        alternate.description = Some("Alternate Service".to_string());
        let mut item = CheckServiceQualificationItem::default();
        item.reason("code", "label");
        item.alternate(alternate);
        let sq = CheckServiceQualification::new("Qualification")
            .item(item);

        dbg!(sq);
    }
}