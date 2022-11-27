import * as L from "leaflet";
import "leaflet/dist/leaflet.css";
import { LineLayer } from "@antv/l7";
import { L7Layer } from "@antv/l7-leaflet";
import { onMount } from "solid-js";

function buildMap(div: HTMLDivElement) {
  const map = L.map("main-map", {
    minZoom: 1,
  }).setView([30, 112], 3);

  L.marker([30, 112])
    .addTo(map)
    .setIcon(
      new L.Icon({
        iconUrl:
          "https://gw.alipayobjects.com/mdn/rms_5e897d/afts/img/A*6ONoRKNECC0AAAAAAAAAAAAAARQnAQ",
        iconSize: [16, 16],
      })
    )
    .bindPopup("A pretty CSS3 popup.<br> Easily customizable.")
    .openPopup();

  // TO KNOW: what is the argument of the 'L7Layer' constructor?
  const l7layer = new L7Layer([]).addTo(map);
  const scene = l7layer.getScene();
  fetch("https://gw.alipayobjects.com/os/rmsportal/UEXQMifxtkQlYfChpPwT.txt")
    .then((res) => res.text())
    .then((data) => {
      const layer = new LineLayer({})
        .source(data, {
          parser: {
            type: "csv",
            x: "lng1",
            y: "lat1",
            x1: "lng2",
            y1: "lat2",
          },
        })
        .size(1)
        .shape("arc")
        .color("#8C1EB2")
        .style({
          opacity: 0.8,
          blur: 0.99,
        });
      console.log("layer", layer);
      scene.addLayer(layer);
    });

  L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    attribution:
      '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
  }).addTo(map);
}

function Map() {
  let mapDiv: any;
  onMount(() => buildMap(mapDiv));
  return (
    <div
      ref={mapDiv}
      style="min-height: 100vh; justify-content: center;position: relative"
      id="main-map"
    />
  );
}

export default Map;
