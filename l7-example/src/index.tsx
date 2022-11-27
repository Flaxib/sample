/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
// import App from "./App";
// import scene from "./Map";
// import scene from "./ChinaPolygonLayer";
import Map from "./Leaflet";

// render(() => <App />, document.getElementById("root") as HTMLElement);

// console.log("scene", scene);
render(() => <Map />, document.getElementById("root") as HTMLElement);
