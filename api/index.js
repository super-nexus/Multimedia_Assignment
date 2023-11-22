const express = require("express");
const fs  = require("fs");
var axios = require('axios');

const app = express();
const port = 3004

// Middleware to parse JSON bodies
app.use(express.json());

var get_weather_request = {
  method: 'get',
  url: 'https://api.openweathermap.org/data/3.0/onecall?lat=4.48&lon=52.15&exclude=hourly,daily&appid=dbde08b4797828949a4cf02ba7c369fe',
  headers: { }
};


/*
  should return [{id, lat, lng, owner, message}]
*/
app.get("/baloons", (req, res) => {

});

/*
  receives {lat, lng, name, message}
*/
app.post('/send-baloon', (req, res) => {
  let data = req.body;
});


app.listen(port, () => {
  console.log(`App listening port: ${port}`)
})
  