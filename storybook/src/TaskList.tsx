import Task, { TaskProps } from "./Task";

export interface TaskListProps {
    loading: boolean,
    tasks: TaskProps[],
    onPinTask: any,
    onArchiveTask: any,
}


export default function (props: TaskListProps){
    if (props.loading) {
        return <div class="list-items">loading</div>;
    }

    if (props.tasks.length === 0) {
        return <div class="list-items">empty</div>;
    }

    return (
        <div class="list-items">
            {props.tasks.map((task) => (
            <Task {...task} />
            ))}
        </div>
    )
}