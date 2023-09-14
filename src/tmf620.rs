//! TMF620 Product Catalogue Management

struct CategoryRef {}
struct RelatedParty {}
struct Catalog {
    category : Vec<CategoryRef>,
    relatedParty : Vec<RelatedParty>,
}