window.onload = function() {
    console.log("Window loaded");
  };
  
  let map, markers = [];
  
  async function initMap() {
    map = new google.maps.Map(document.getElementById("map"), {
      center: { lat: 52.160114, lng: 4.497010 },
      zoom: 13,
    });
  
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
  