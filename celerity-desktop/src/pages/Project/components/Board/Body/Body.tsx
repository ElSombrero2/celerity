import { Card } from "@/ui/components/ui/card";
import { Task } from "../../../../../app/types/kanban";

export const Body = (task: Task) => (
    <Card>
        {task.title}
    </Card>
)
