const { MongoClient } = require('mongodb');
const { Long } = require('bson');

async function main() {
    const uri = "mongodb://localhost:27017";
    const client = new MongoClient(uri);

    try {
        await client.connect();
        console.log("Connected to MongoDB");

        const database = client.db("baloon-service");
        const collection = database.collection("baloons");

        const baloon = {
            lat: 52.162209,
            lng: 4.531729,
            owner: "Simba",
            message: "I'm a lion!",
            timestamp: Long.fromNumber(Date.now()),
            popped: true,
            popped_at: Long.fromNumber(Date.now() + 1000 * 60 * 60 * 24),
        }; 

        const result = await collection.insertOne(baloon);
        console.log(`A document was inserted with the _id: ${result.insertedId}`);
    } catch (e) {
        console.error(e);
    } finally {
        await client.close();
    }
}

main().catch(console.error);