import { Task } from "../../../../../app/types/kanban";
import { Card } from "../../../../../ui/components/Cards/Card/Card";

export const Body = (task: Task) => (
    <Card>
        {task.title}
    </Card>
)
