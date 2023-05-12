import type { Component } from "solid-js";
import { Show } from "solid-js";

import logo from "./logo.svg";
import styles from "./App.module.css";

type Item = {
  hour: string;
  name: string;
  caption: string | null;
};

function Timeline_item(props: Item) {
  return (
    <div
      class="v-timeline-item"
      style="--v-timeline-dot-size:30px; --v-timeline-line-inset:0px;"
    >
      <div class="v-timeline-item__body">
        <div class="d-flex">
          <strong class="me-4">{props.hour}</strong>
          <div>
            <strong>{props.name}</strong>
            <Show when={props.caption != null}>
              <div class="text-caption"> {props.caption} </div>
            </Show>
          </div>
        </div>
      </div>
      <div class="v-timeline-divider">
        <div class="v-timeline-divider__before"></div>
        <div class="v-timeline-divider__dot v-timeline-divider__dot--size-small">
          <div class="v-timeline-divider__inner-dot bg-pink">
            <i class="" aria-hidden="true"></i>
          </div>
        </div>
        <div class="v-timeline-divider__after"></div>
      </div>
    </div>
  );
}

function Timeline() {
  return (
    <div class="pa-4">
      <div
        class="v-timeline v-timeline--align-start v-timeline--justify-auto v-timeline--side-end v-timeline--vertical"
        style="--v-timeline-line-thickness:2px;"
      >
        <Timeline_item hour="5pm" name="New Icon" caption="Mobile app" />
        <Timeline_item hour="12pm" name="Lunch break" caption={null} />
        <Timeline_item hour="5pm" name="New Icon" caption={null} />
        <Timeline_item
          hour="9-11am"
          name="Finish Home Screen"
          caption="Web App"
        />
      </div>
    </div>
  );
}

export default Timeline;
