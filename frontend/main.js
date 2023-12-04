window.onload = function() {
  console.log("Window loaded");
};

let map = {};
let balloons = {};
let fetchInterval = 10;// fetch balloons every 10 seconds
let markers = []; //should be deleted later

async function initMap() {

  console.log('Initialising Map')
  map = new google.maps.Map(document.getElementById("map"), {
    center: { lat: 52.160114, lng: 4.497010 },
    zoom: 13,
  });




  // setInterval(fetchBalloons, fetchInterval * 1000);
  //when backend is done code below should be deleted and code above should be uncommented
  // let baloonData = await getData('http://localhost:3000/baloons')
  
  
  if(false){
    console.log('Converting baloon data to markers')
  // Create baloon markers
  for (let i = 0; i < baloonData.length; i++) {
    console.log(baloonData[0])
    if (baloonData[i].lat!=null){
      const pos = {
        lat: baloonData[i].lat,
        lng: baloonData[i].lng,
      };
      const marker = new google.maps.Marker({
        position: pos,
        map,
        icon: {
          url: 'balloon.png',
          scaledSize: new google.maps.Size(20, 20),  // 20x20 pixels
        },
      });
  
      markers.push(marker);
    }
   
  }
  }else{
    for (let i = 0; i < 10; i++) {
      const pos = {
        lat: 52.160114 + Math.random() * 0.02,
        lng: 4.497010 + Math.random() * 0.02,
      };
      const marker = new google.maps.Marker({
        position: pos,
        map,
        icon: {
          url: 'balloon.png',
          scaledSize: new google.maps.Size(20, 20),  // 20x20 pixels
        },
      });
  
      markers.push(marker);
    }
  }


  // Simulate wind movement for each marker
  markers.forEach(marker => {
    console.log(marker)
    const endPos = {
      lat: marker.getPosition().lat() + Math.random() * 0.02 - 0.01,
      lng: marker.getPosition().lng() + Math.random() * 0.02 - 0.01,
    };
    animateMarker(marker, marker.getPosition().toJSON(), endPos, 10000);
  });
  

}



// Example POST method implementation:
async function getData(url = "") {
  // Default options are marked with *
  const response = await fetch(url, {
    method: "GET", 
    mode: "cors", 
    cache: "no-cache",
    credentials: "same-origin", 
    redirect: "follow", // manual, *follow, error
    referrerPolicy: "no-referrer", 
  });
  return response.json(); 
}




// Function to animate marker (same as your code)
function animateMarker(marker, startPos, endPos, duration) {
  let startTime = null;
  function animate(time) {
    if (!startTime) startTime = time;
    const progress = (time - startTime) / duration;
    const lat = startPos.lat + (endPos.lat - startPos.lat) * progress;
    const lng = startPos.lng + (endPos.lng - startPos.lng) * progress;

    marker.setPosition(new google.maps.LatLng(lat, lng));

    if (progress < 1) {
      requestAnimationFrame(animate);
    }
  }
  requestAnimationFrame(animate);
}




function fetchBalloons() {
  fetch('/balloons')
  .then(response => response.json())
  .then(data => {
    data.forEach(balloon => {
      //check if marker already exists in array of markers
      if(balloons[balloon.id]){
        let marker = balloons[balloon.id].marker;
        animateMarker(marker, marker.getPosition().toJSON(), balloon, 10000);
      }else{
        const marker = new google.maps.Marker({
          position: { lat: balloon.lat, lng: balloon.lng },
          map,
          icon: {
            url: 'balloon.png',
            scaledSize: new google.maps.Size(20, 20),  // 20x20 pixels
          },
        });
        balloons[balloon.id] = {}
        balloons[balloon.id].marker = marker;
        balloons[balloon.id].message = balloon.message;
        balloons[balloon.id].owner = balloon.owner;
      }
    });
  });
}



function getUserGeolocation() {
  return new Promise((resolve, reject) => {
    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(resolve, reject);
    } else {
      reject(new Error("Geolocation is not supported by this browser."));
    }
  });
}

function showElement(querySelector){
  document.querySelector(querySelector).style.display = "block";
}

function hideElement(querySelector){
  document.querySelector(querySelector).style.display = "none";
}


function openModal(message){
  let modal = document.getElementById("send-balloon-modal");
  modal.style.display = "block";

  let modalMessage = document.getElementById("modal-message");
  modalMessage.innerHTML = message;

  var span = document.getElementsByClassName("close-button")[0];
  span.onclick = function() {
      modal.style.display = "none";
  }

  window.onclick = function(event) {
    if (event.target === modal) {
        modal.style.display = "none";
    }
  }
}


async function postData(url = "", data ) {

  const response = await fetch(url, {
    method: "POST", 
    mode: "cors", 
    cache: "no-cache", 
    credentials: "same-origin", 
    headers: {
      "Content-Type": "application/json",
    
    },
    redirect: "follow", 
    referrerPolicy: "no-referrer", 
    body: data, 
  });
  return response; 
}




document.getElementById('myForm').addEventListener('submit', sendballoon);

// async function handleSubmit(event) {
//   event.preventDefault();

//   postData("http://localhost:3000/baloons", { answer: 42 }).then((data) => {
//     console.log(data); 
//   });
// }

async function sendballoon(e){
  e.preventDefault();

  let userGeolocation = await getUserGeolocation();
  console.log(userGeolocation)

  console.log(document.querySelector("#name").value,userGeolocation.coords.longitude)

  var balloonInfo=  JSON.stringify({
    lat: userGeolocation.coords.latitude,
    lng: userGeolocation.coords.longitude,
    name: document.querySelector("#name").value,
    message: document.querySelector("#message").value
  })
  postData("http://localhost:3000/baloons",  balloonInfo ).then((data) => {
    console.log(data); 
  });

  document.getElementById('name').value = '';
  document.getElementById('message').value = '';
}



function openTab(evt, tabName) {
  var i, tabcontent, tablinks;
  tabcontent = document.getElementsByClassName("tabcontent");
  for (i = 0; i < tabcontent.length; i++) {
    tabcontent[i].style.display = "none";
  }
  tablinks = document.getElementsByClassName("tablink");
  for (i = 0; i < tablinks.length; i++) {
    tablinks[i].className = tablinks[i].className.replace(" active", "");
  }
  document.getElementById(tabName).style.display = "block";
  evt.currentTarget.className += " active";
}