import type { Component } from "solid-js";


export interface TaskProps{
    id: number,
    title: string,
    state: string,
    // onArchiveTask: (id: number) => {},
    // onPinTask: (id: number) => {},
}

export default function Task(props:TaskProps) {
    return (
      
      <div class={`list-item ${props.state}`}>
      <label class="checkbox">
        <input
          type="checkbox"
          checked={props.state === 'TASK_ARCHIVED'}
          disabled={false}
          name="checked"
          id={"check-" + props.id}
        />
        <span class="checkbox-custom" onClick={() => {}} />
      </label>
      <div class="title">
        <label for={"check-" + props.id}>{props.title}</label>
      </div>

      <div class="actions" onClick={(event) => event.stopPropagation()}>
        {props.state !== 'TASK_ARCHIVED' && (
          <span class="fa fa-star" classList={{checked: props.state == 'TASK_PINNED'}}></span>
        )}
      </div>
    </div>
    );
  }