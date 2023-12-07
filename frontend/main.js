window.onload = function () {
  console.log("Window loaded");
};

let map = {};
let balloons = {};
let fetchInterval = 5;// fetch balloons every 10 seconds

async function initMap() {

  console.log('Initialising Map')
  map = new google.maps.Map(document.getElementById("map"), {
    center: { lat: 52.160114, lng: 4.497010 },
    zoom: 13,
  });


  searchBalloon()
  fetchBalloons()

  setInterval(fetchBalloons, fetchInterval * 1000);

}

async function searchBalloon(){

  let userGeoLocation = await getUserGeolocation();
  var requestOptions = {
    method: 'GET',
    redirect: 'follow'
  };

  let lat = userGeoLocation.coords.latitude;
  let lng = userGeoLocation.coords.longitude;
  
  fetch(`http://localhost:3000/popped-baloons?lat=${lat}&lng=${lng}`, requestOptions)
    .then(response => {
      return response.json();
    })
    .then(data=>{
      data.forEach(balloon=>{
        console.log(balloon)
      })
    })

}

// Function to animate marker (same as your code)
function animateMarker(marker, startPos, duration) {
  let startTime = null;
  function animate(time) {
    if (!startTime) startTime = time;
    const progress = (time - startTime) / duration;
    const lat = startPos.lat
    const lng = startPos.lng

    marker.setPosition(new google.maps.LatLng(lat, lng));

    if (progress < 1) {
      requestAnimationFrame(animate);
    }
  }
  requestAnimationFrame(animate);
}

function createMarker(lat, lng) {
 return new google.maps.Marker({
    position: {
      lat: lat,
      lng: lng
    },
    map,
    icon: {
      url: './baloon.png',
      scaledSize: new google.maps.Size(20, 20),  // 20x20 pixels
    },
  });
}

function fetchBalloons() {
  console.log("Fetching balloons");
  var requestOptions = {
    method: 'GET',
    redirect: 'follow'
  };
  fetch("http://localhost:3000/baloons", requestOptions)
    .then(response => response.json())
    .then(data => {
      data.forEach(balloon => {
        //check if marker already exists in array of markers and has a valid coordinates
        if (balloon.lat != null) {
          if (balloons[balloon._id]) {
            let marker = balloons[balloon._id].marker;
            animateMarker(marker, {lat: balloon.lat, lng: balloon.lng}, fetchInterval * 1000);
          } else {
            const marker = new google.maps.Marker({
              position: {
                lat: balloon.lat,
                lng: balloon.lng,
              },
              map,
              icon: {
                url: './baloon.png',
                scaledSize: new google.maps.Size(20, 20),  // 20x20 pixels
              },
            });
            balloons[balloon._id] = {};
            balloons[balloon._id].marker = marker;
            balloons[balloon._id].message = balloon.message;
            balloons[balloon._id].owner = balloon.owner;
          }
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

function showElement(querySelector) {
  document.querySelector(querySelector).style.display = "block";
}

function hideElement(querySelector) {
  document.querySelector(querySelector).style.display = "none";
}


function openModal(message) {
  let modal = document.getElementById("send-balloon-modal");
  modal.style.display = "block";

  let modalMessage = document.getElementById("modal-message");
  modalMessage.innerHTML = message;

  var span = document.getElementsByClassName("close-button")[0];
  span.onclick = function () {
    modal.style.display = "none";
  }

  window.onclick = function (event) {
    if (event.target === modal) {
      modal.style.display = "none";
    }
  }
}


async function postData(url = "", data) {

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

async function sendballoon(e) {
  e.preventDefault();

  let userGeolocation = await getUserGeolocation();
  console.log(userGeolocation)

  console.log(document.querySelector("#name").value, userGeolocation.coords.longitude)

  var balloonInfo = JSON.stringify({
    lat: userGeolocation.coords.latitude,
    lng: userGeolocation.coords.longitude,
    name: document.querySelector("#name").value,
    message: document.querySelector("#message").value
  })
  postData("http://localhost:3000/baloons", balloonInfo).then((data) => {
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
