    use std::env::{self};
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use bigdecimal::{BigDecimal, FromPrimitive};
    use diesel::RunQueryDsl;
    use domain::models::NewProduct;
    use infrastructure::database::connection::establish_connection;
    use rand::Rng;
    use uuid::Uuid;
    use chrono::Utc;

    fn read_products_from_csv(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(BufReader::new(file));
        let mut bucket_keys = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let bucket_key = record.get(0).ok_or("Missing bucket_key")?.to_string();
            bucket_keys.push(bucket_key);
        }

        Ok(bucket_keys)
    }

    //This seeder script can be used to seed the database with bucket keys from the Digitalocean bucket. There
    //is around 200 keys for now, each corresponding to medium or large images.
    fn main() -> Result<(), Box<dyn Error>> {
        use domain::schema::products;

        let connection = &mut establish_connection();
        let current_dir = env::current_dir()?;
        let csv_path = current_dir.join("infrastructure/src/bin/bucket_keys.csv");
        let bucket_keys = read_products_from_csv(csv_path.to_str().expect("Invalid UTF-8 in path"))?;

        let categories = vec![1, 2, 3, 4];
        let names = vec!["Shirt", "Dress", "Jacket", "Pants", "Blouse"];
        let descriptions = vec![
            "Stylish and comfortable",
            "Elegant and chic",
            "Casual everyday wear",
            "Trendy and fashionable",
            "Classic design",
        ];

        let prices = vec![19.99, 29.99, 39.99, 49.99, 59.99];

        let seller_ids = vec![
            "58346d65-40e1-4d88-b938-13588c0caa15",
            "5cf522b0-ac95-4526-a0c3-3163d38115f0",
            "ef7903d1-ec4b-4264-bcbd-f46524d601d6",
            "8d71442b-99f0-4557-af99-7d3e78aa1ea5",
            "7af689f1-3f74-4586-a56c-29b913815f0b",
        ];

        let mut rng = rand::thread_rng();

        let new_products: Vec<NewProduct> = bucket_keys.into_iter().map(|bucket_key| NewProduct {
            name: names[rng.gen_range(0..names.len())].to_string(),
            description: descriptions[rng.gen_range(0..descriptions.len())].to_string(),
            price: BigDecimal::from_f64(prices[rng.gen_range(0..prices.len())])
                .expect("Failed to convert f64 to BigDecimal"),
            quantity: rng.gen_range(1..20),
            seller_id: Uuid::parse_str(seller_ids[rng.gen_range(0..seller_ids.len())]).unwrap(),
            category_id: categories[rng.gen_range(0..categories.len())],
            creation_date: Utc::now().naive_utc(),
            bucket_key,
        }).collect();

        diesel::insert_into(products::table)
            .values(&new_products)
            .execute(connection)
            .map_err(|err| {
                eprintln!("Failed to insert products: {}", err);
                err
            })?;

        println!("Seeded {} products", new_products.len());
        Ok(())
    }
