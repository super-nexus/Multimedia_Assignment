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

function calculateDistance(lat1, lon1, lat2, lon2) {
  const R = 6371e3; // radius of the Earth in meters
  const phi1 = lat1 * Math.PI / 180; // convert degrees to radians
  const phi2 = lat2 * Math.PI / 180;
  const deltaPhi = (lat2 - lat1) * Math.PI / 180;
  const deltaLambda = (lon2 - lon1) * Math.PI / 180;

  const a = Math.sin(deltaPhi / 2) * Math.sin(deltaPhi / 2) +
            Math.cos(phi1) * Math.cos(phi2) *
            Math.sin(deltaLambda / 2) * Math.sin(deltaLambda / 2);
  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

  return R * c; // distance in meters
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
    const result = await collection.find({}).toArray();
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

    let userCoordinates = {
      lat: req.query.lat,
      lng: req.query.lng
    }

    let poppedArray = result.filter(balloon => {
      let balloonCoordinates = {
        lat: balloon.lat,
        lng: balloon.lng
      }

      let distance = calculateDistance(userCoordinates.lat, userCoordinates.lng, balloonCoordinates.lat, balloonCoordinates.lng);
      return distance < 100;
    });

    res.status(200).send(poppedArray);
  } catch (err) {
    res.status(500).send();
    console.log(err.stack);
  }
});

connectDB().catch(console.error);

app.listen(port, () => {
  console.log(`App listening port: ${port}`)
})
  