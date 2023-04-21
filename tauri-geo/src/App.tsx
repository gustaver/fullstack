import { useState } from "react";
import "./App.css";
import "leaflet/dist/leaflet.css";
import { MapContainer, TileLayer, Marker, Popup } from 'react-leaflet';

const CENTER_POSITION = [59.3293, 18.0686];

function App() {
  const [position, setPosition] = useState(CENTER_POSITION);

  // setInterval(() => {
  //   const [dLat, dLong] = [0.0001, 0.0];
  //   const [lat, long] = position;
  //   setPosition([lat + dLat, long + dLong]);
  // }, 500);

  return (
      <MapContainer center={CENTER_POSITION} zoom={13} scrollWheelZoom={false}>
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        <Marker position={position}/>
      </MapContainer>
  );
}

export default App;
