import { StoryObj } from '@storybook/html';
import Task, { TaskProps } from './Task';



type Story = StoryObj<TaskProps>;

export const TaskDefault: Story = {
    args: {
        id: 1,
        title: 'Test Task',
        state: 'TASK_INBOX',
    },      
};  

export const Pinned: Story = {
    args: {
        id: 1,
        title: 'Test Task',
        state: 'TASK_PINNED',
    },      
};

export const Archived: Story = {
    args: {
        id: 1,
        title: 'Test Task',
        state: 'TASK_ARCHIVED',
    },      
};

export default {
    component: Task,
    title: 'Task',
    tags: ['autodocs'],
    render: (props: TaskProps) => <Task {...props}/>,
};