import TaskList,  {TaskListProps} from "./TaskList";
import * as TaskStories from './Task.stories'
import { StoryObj } from "@storybook/html";

// console.log(Tasks.TaskDefault);

type Story = StoryObj<TaskListProps>;


export const TaskDefault: Story = {
    args: {
        tasks: [
            {...TaskStories.TaskDefault.args, id: 1},
            {...TaskStories.TaskDefault.args, id: 2},
            {...TaskStories.TaskDefault.args, id: 3}
        ]
    },      
}; 

export const WithPinnedTasks = {
    args: {
      tasks: [
        ...TaskDefault.args.tasks.slice(0, 5),
        { id: 6, title: 'Task 6', state: 'TASK_PINNED' },
      ],
    },
};

export const Loading = {
    args: {
      tasks: [],
      loading: true,
    },
};

export const Empty = {
    args: {
      ...Loading.args,
      loading: false,
    },
  };
  
export default {
    component: TaskList,
    title: 'TaskList',
    decorators: [(story: any) => <div style={{ padding: '3rem' }}>{story()}</div>],
    tags: ['autodocs'],
  };