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

        for(let i = 0; i < 20; i++){
          
          let rand1 = Math.floor(Math.random() * 1000)+ 100
          let randMove1 = Math.floor(Math.random() * 10) / rand1;
          
          let rand2 = Math.floor(Math.random() * 1000)+ 100
          let randMove2 = Math.floor(Math.random() * 10) / rand2;

          const baloon = {
            lat: 52.1452536+randMove1,
            lng: 4.473652+randMove2,
            owner: "Simba",
            message: "I'm a lion!",
            timestamp: Long.fromNumber(Date.now()),
            popped: false,
            popped_at: Long.fromNumber(0),
          }; 

          const result = await collection.insertOne(baloon);
          console.log(`A document was inserted with the _id: ${result.insertedId}`);
        }
        
    } catch (e) {
        console.error(e);
    } finally {
        await client.close();
    }
}

main().catch(console.error);