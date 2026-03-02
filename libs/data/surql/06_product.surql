-- US-14: Product table definition
DEFINE TABLE product SCHEMAFULL;

DEFINE FIELD name ON TABLE product TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD description ON TABLE product TYPE option<string>;
DEFINE FIELD sku ON TABLE product TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD price ON TABLE product TYPE int ASSERT $value >= 0;
DEFINE FIELD created_at ON TABLE product TYPE datetime DEFAULT time::now() VALUE $before OR time::now();
DEFINE FIELD updated_at ON TABLE product TYPE datetime DEFAULT time::now() VALUE time::now();

DEFINE INDEX sku_unique ON TABLE product FIELDS sku UNIQUE;
