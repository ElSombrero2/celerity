import { Todos } from "../kanban";

export interface Project {
    id: string,
    name: string,
    created_at: string,
    docker: boolean,
    based_template: string,
    board: {[key: string]: Todos}
}