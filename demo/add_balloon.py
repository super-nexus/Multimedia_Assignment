from pymongo import MongoClient
from bson import Int64
import datetime

def insert_document(collection, document):
    return collection.insert_one(document).inserted_id

def main():
    uri = "mongodb://localhost:27017"
    client = MongoClient(uri)

    db = client['baloon-service']  # Replace with your database name
    collection = db['baloons']  # Replace with your collection name

    # Baloon object
    baloon = {
        "lat": 52.162209,
        "lng": 4.531729,
        "owner": "Simba",
        "message": "I'm a lion!",
        "timestamp": Int64(datetime.datetime.now().timestamp() * 1000),  # Current timestamp in milliseconds
        "popped": True,
        "popped_at": Int64((datetime.datetime.now() + datetime.timedelta(days=1)).timestamp() * 1000)  # Timestamp for 24 hours later in milliseconds
    }

    # Insert the baloon document
    doc_id = insert_document(collection, baloon)
    print(f"Document inserted with ID: {doc_id}")

if __name__ == "__main__":
    main()