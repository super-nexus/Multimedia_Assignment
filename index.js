console.log('Hello world')
const express = require("express");
const fs  = require("fs");
var axios = require('axios');


const app = express();
const port = 3004


var config = {
  method: 'get',
  url: 'https://api.openweathermap.org/data/3.0/onecall?lat=4.48&lon=52.15&exclude=hourly,daily&appid=dbde08b4797828949a4cf02ba7c369fe',
  headers: { }
};

axios(config)
.then(function (response) {
  console.log(JSON.stringify(response.data));
})
.catch(function (error) {
  console.log(error);
});





app.listen(port, () => {


    console.log(`App listening port: ${port}`)
  })
  