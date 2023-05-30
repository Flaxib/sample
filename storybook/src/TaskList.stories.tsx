import TaskList,  {TaskListProps} from "./TaskList";
import * as TaskStories from './Task.stories'
import { StoryObj } from "@storybook/html";

// console.log(Tasks.TaskDefault);

type Story = StoryObj<TaskListProps>;


export const TaskDefault: Story = {
    args: {
        tasks: [
            {...TaskStories.TaskDefault.args},
            {...TaskStories.Pinned.args},
            {...TaskStories.Archived.args},
        ]
    },      
}; 


export default {
    component: TaskList,
    title: 'TaskList',
    decorators: [(story) => <div style={{ padding: '3rem' }}>{story()}</div>],
    tags: ['autodocs']
  };