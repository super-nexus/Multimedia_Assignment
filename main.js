window.onload = function() {
  console.log("Window loaded");
};

let map = {};
let baloons = {};
let fetchInterval = 10;// fetch baloons every 10 seconds


async function initMap() {
  map = new google.maps.Map(document.getElementById("map"), {
    center: { lat: 52.160114, lng: 4.497010 },
    zoom: 13,
  });




  // setInterval(fetchBalloons, fetchInterval * 1000);
  //when backend is done code below should be deleted and code above should be uncommented

  // Create markers
  for (let i = 0; i < 10; i++) {
    const pos = {
      lat: 52.160114 + Math.random() * 0.02,
      lng: 4.497010 + Math.random() * 0.02,
    };
    const marker = new google.maps.Marker({
      position: pos,
      map,
      icon: {
        url: 'baloon.png',
        scaledSize: new google.maps.Size(20, 20),  // 20x20 pixels
      },
    });

    markers.push(marker);
  }

  // Simulate wind movement for each marker
  markers.forEach(marker => {
    const endPos = {
      lat: marker.getPosition().lat() + Math.random() * 0.02 - 0.01,
      lng: marker.getPosition().lng() + Math.random() * 0.02 - 0.01,
    };
    animateMarker(marker, marker.getPosition().toJSON(), endPos, 10000);
  });
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
  fetch('/baloons')
  .then(response => response.json())
  .then(data => {
    data.forEach(baloon => {
      //check if marker already exists in array of markers
      if(baloons[baloon.id]){
        let marker = baloons[baloon.id].marker;
        animateMarker(marker, marker.getPosition().toJSON(), baloon, 10000);
      }else{
        const marker = new google.maps.Marker({
          position: { lat: baloon.lat, lng: baloon.lng },
          map,
          icon: {
            url: 'baloon.png',
            scaledSize: new google.maps.Size(20, 20),  // 20x20 pixels
          },
        });
        baloons[baloon.id] = {}
        baloons[baloon.id].marker = marker;
        baloons[baloon.id].message = baloon.message;
        baloons[baloon.id].owner = baloon.owner;
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
  let modal = document.getElementById("send-baloon-modal");
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

async function sendBaloon(e){
  e.preventDefault();

  let userGeolocation = await getUserGeolocation();
  console.log(userGeolocation)


  //when backend is done code below should be uncommented
  //and modal should be deleted

  // await fetch('/send-baloon', {
  //   method: 'POST',
  //   body: JSON.stringify({
  //     lat: userGeolocation.coords.latitude,
  //     lng: userGeolocation.coords.longitude,
  //     name: document.querySelector("#name").value,
  //     message: document.querySelector("#message").value
  //   })
  // }).then(()=>{
  //   openModal("Your baloon was succesfully sent!")
  // }).catch(()=>{
  //   openModal("Ther was an error when sending your baloon!")
  // })

  openModal("Your baloon was succesfully sent!")



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