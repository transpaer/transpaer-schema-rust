use pretty_assertions::assert_eq;

use sustainity_schema as schema;

// TODO: Add json and jsonl parsing.
// TODO: Add review and producer roots.

#[test]
fn serde_meta() {
    let value = schema::Meta {
        authors: vec!["Sustainity Testing Team".to_owned()],
        creation_timestamp: None,
        description: None,
        title: "models fixture 1".to_owned(),
        valid_from: None,
        valid_to: None,
        variant: schema::ProviderVariant::Cataloger,
        version: "0.0.1".to_owned(),
    };

    let yaml_string = indoc::indoc!(
        r#"
        authors:
        - Sustainity Testing Team
        title: models fixture 1
        variant: cataloger
        version: 0.0.1
        "#
    );

    let received_yaml_string = serde_yaml::to_string(&value).unwrap();
    assert_eq!(yaml_string, received_yaml_string);

    let received_value = serde_yaml::from_str(&yaml_string).unwrap();
    assert_eq!(value, received_value);
}

#[test]
fn serde_cataloger_data() {
    let value = schema::CatalogerData {
        cataloger: schema::AboutCataloger {
            description: Some("Test Cataloger".to_owned()),
            id: "test test".to_owned(),
            name: "Tester".to_owned(),
            variant: schema::CatalogVariant::Store,
            website: "https://www.example.com/".to_owned(),
        },
        producers: vec![schema::CatalogProducer {
            id: "fairphone".to_owned(),
            ids: schema::ProducerIds {
                vat: None,
                domains: Some(vec!["fairphone.com".to_owned()]),
                wiki: Some(vec!["5019402".to_owned()]),
            },
            names: vec!["Fairphone".to_owned()],
            description: None,
            images: Vec::new(),
            origins: None,
            websites: Vec::new(),
        }],
        products: vec![schema::CatalogProduct {
            id: "fairphone-5".to_owned(),
            ids: schema::ProductIds {
                ean: Some(vec![
                    "8718819372271".to_owned(),
                    "8718819372288".to_owned(),
                    "8718819372448".to_owned(),
                    "8718819372295".to_owned(),
                    "8718819372431".to_owned(),
                ]),
                gtin: Some(Vec::new()),
                wiki: Some(vec!["5019402".to_owned()]),
            },
            names: vec!["Fairphone 5".to_owned()],
            description: None,
            categorisation: Some(schema::ProductCategorisation {
                categories: vec![schema::ProductCategory("smartphone".to_owned())],
            }),
            availability: None,
            images: Vec::new(),
            origins: None,
            related: None,
            shopping: Some(schema::Shopping(vec![
                schema::ShoppingEntry {
                    shop: schema::VerifiedShop::Amazon,
                    id: "B0CH3QTV43".to_owned(),
                    description: "8GB, 256GB, Transparent".to_owned(),
                },
                schema::ShoppingEntry {
                    shop: schema::VerifiedShop::Amazon,
                    id: "B0DJP6NNL8".to_owned(),
                    description: "6GB, 128GB, Black".to_owned(),
                },
                schema::ShoppingEntry {
                    shop: schema::VerifiedShop::Amazon,
                    id: "B0CH3Q8V2F".to_owned(),
                    description: "8GB, 256GB, Black".to_owned(),
                },
                schema::ShoppingEntry {
                    shop: schema::VerifiedShop::Amazon,
                    id: "B0CH3N3Y3D".to_owned(),
                    description: "8GB, 256GB, Blue".to_owned(),
                },
                schema::ShoppingEntry {
                    shop: schema::VerifiedShop::Fairphone,
                    id: "fairphone-5-273".to_owned(),
                    description: "All models".to_owned(),
                },
            ])),
        }],
    };

    let yaml_string = indoc::indoc!(
        r#"
        cataloger:
          description: Test Cataloger
          id: test test
          name: Tester
          variant: store
          website: https://www.example.com/
        producers:
        - id: fairphone
          ids:
            domains:
            - fairphone.com
            wiki:
            - '5019402'
          names:
          - Fairphone
        products:
        - categorisation:
            categories:
            - smartphone
          id: fairphone-5
          ids:
            ean:
            - '8718819372271'
            - '8718819372288'
            - '8718819372448'
            - '8718819372295'
            - '8718819372431'
            gtin: []
            wiki:
            - '5019402'
          names:
          - Fairphone 5
          shopping:
          - description: 8GB, 256GB, Transparent
            id: B0CH3QTV43
            shop: amazon
          - description: 6GB, 128GB, Black
            id: B0DJP6NNL8
            shop: amazon
          - description: 8GB, 256GB, Black
            id: B0CH3Q8V2F
            shop: amazon
          - description: 8GB, 256GB, Blue
            id: B0CH3N3Y3D
            shop: amazon
          - description: All models
            id: fairphone-5-273
            shop: fairphone
        "#
    );

    let received_yaml_string = serde_yaml::to_string(&value).unwrap();
    assert_eq!(yaml_string, received_yaml_string);

    let received_value = serde_yaml::from_str(&yaml_string).unwrap();
    assert_eq!(value, received_value);
}
