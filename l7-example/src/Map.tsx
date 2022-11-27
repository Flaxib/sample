import { Scene } from "@antv/l7";
import { GaodeMap } from "@antv/l7-maps";

// Similarly you can also initialize a Mapbox map
const scene = new Scene({
  id: "map",
  map: new GaodeMap({
    pitch: 35.210526315789465,
    style: "dark",
    center: [104.288144, 31.239692],
    zoom: 4.4,
  }),
});

export default scene;
