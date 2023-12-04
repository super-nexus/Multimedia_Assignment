const express = require("express");
const cors = require("cors");
const { MongoClient } = require('mongodb');
const { Long } = require('bson');
require('dotenv').config();


const app = express();
app.use(cors());
const port = process.env.PORT || 3000;

const mongo_url = process.env.MONGO_URL || "mongodb://localhost:27017";
const client = new MongoClient(mongo_url);

async function connectDB() {
  try {
    await client.connect();
    console.log("Connected correctly to server");
  } catch (err) {
    console.log(err.stack);
  }
}

const database = client.db("baloon-service");
const collection = database.collection("baloons");

// Middleware to parse JSON bodies
app.use(express.json());


/*
  should return [{id, lat, lng, owner, message}]
*/
app.get("/baloons", async (req, res) => {
  try {
    const result = await collection.find({ popped: false }).toArray();
    res.status(200).send(result);
  } catch (err) {
    res.status(500).send();
    console.log(err.stack);
  }
});

/*
  receives {lat, lng, name(owner), message}
*/
app.post('/baloons', async (req, res) => {

  let data = req.body;
  console.log("Received: ", data )

  let baloon = {
    lat: data.lat,
    lng: data.lng,
    owner: data.name,
    message: data.message,
    timestamp: Long.fromNumber(Date.now()),
    popped: false,
    popped_at: Long.fromNumber(0)
  }

  try{
    const result = await collection.insertOne(baloon);
    res.status(200).send(`Baloon ${data.name} saved`);
  } catch (err) {
    res.status(500).send(err);
    console.log(err.stack);
  }
});

app.get('/popped-baloons', async (req, res) => {
  try {
    const result = await collection.find({ popped: true }).toArray();
    res.status(200).send(result);
  } catch (err) {
    res.status(500).send();
    console.log(err.stack);
  }
});

connectDB().catch(console.error);

app.listen(port, () => {
  console.log(`App listening port: ${port}`)
})
  