use crate::baloon::model::Baloon;
use mongodb::{Client, options::ClientOptions, Collection, bson::{self, Document}, bson::doc};
use futures::stream::TryStreamExt;

pub async fn get_client() -> Client {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.expect("Unable to connect to MongoDB");
    client_options.app_name = Some("baloon-service".to_string());
    Client::with_options(client_options).expect("Failed to create client")
}

pub async fn get_baloons(client: &Client) -> Vec<Baloon> {
    let baloons_collection = client.database("baloon-service").collection("baloons");
    
    let mut cursor = baloons_collection.find(None, None).await.expect("Could not find baloons");
    let mut baloons: Vec<Baloon> = Vec::new();

    while let Some(result) = cursor.try_next().await.expect("Could not get next baloon") {
        let baloon: Baloon = bson::from_document(result).expect("Could not convert baloon to bson");
        println!("{:?}", baloon);
        baloons.push(baloon);
    }

    baloons
}

pub async fn add_baloon(client: &Client, baloon: &Baloon) {
    let baloons_collection = client.database("baloon-service").collection("baloons");
    let baloon_doc = bson::to_document(baloon).expect("Could not convert baloon to bson");

    baloons_collection.insert_one(baloon_doc, None).await.expect("Could not insert baloon");
}

pub async fn update_baloons(client: &Client, baloons: &Vec<Baloon>) {
    let baloons_collection: Collection<Document> = client.database("baloon-service").collection("baloons");

    for baloon in baloons {
        if let Some(id) = baloon.id {
            let filter = doc! { "_id": id };
            let update = bson::to_document(baloon).expect("Could not convert baloon to bson");
            baloons_collection.update_one(filter, doc! {"$set": update}, None).await.expect("Could not update baloon");
        }
    }
}

pub async fn delete_baloons(client: &Client, baloons: &Vec<Baloon>) {
    let baloons_collection: Collection<Document> = client.database("baloon-service").collection("baloons");

    for baloon in baloons {
        if let Some(id) = baloon.id {
            let filter = doc! { "_id": id };
            baloons_collection.delete_one(filter, None).await.expect("Could not delete baloon");
        }
    }
}

pub async fn get_baloons_and_popped_baloons(client: &Client) -> (Vec<Baloon>, Vec<Baloon>) {
    let baloons: Vec<Baloon> = get_baloons(client).await;
    let popped_baloons: Vec<Baloon> = baloons
        .iter()
        .filter(|baloon| baloon.popped)
        .cloned()
        .collect();

    let not_popped_baloons: Vec<Baloon> = baloons
        .iter()
        .filter(|baloon| !baloon.popped)
        .cloned()
        .collect(); 


    (not_popped_baloons, popped_baloons)
}


mod tests {
    use super::*;
    use tokio::test as tokio_test;
    
    #[tokio_test]
    async fn test_get_baloons() {
        let client = get_client().await;
        let baloons = get_baloons(&client).await;

        assert!(baloons.len() > 0);
    }

    #[tokio_test]
    async fn test_add_baloon() {
        let client = get_client().await;
        let baloon = Baloon {
            id: None,
            lat: 51.50,
            lng: 4.49,
            timestamp: chrono::offset::Utc::now().timestamp(),
            owner: String::from("Andrija"),
            message: String::from("Hello World!"),
            popped: false,
            popped_at: 0
        };

        add_baloon(&client, &baloon).await;

        let baloons: Vec<Baloon> = get_baloons(&client).await;
        let baloon = baloons.iter().find(|baloon| baloon.message == "Hello World!");
        
        println!("{:?}", baloon);
        assert!(baloon.is_some());
    }
}