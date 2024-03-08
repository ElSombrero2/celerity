import { useState } from "react";
import { _board } from "../../../__mock__/board";
import { BoardKeyIndex } from "../../../app/component/Board/Board";
import { Task } from "../../../app/types/kanban";

export const useProject = () => {
    const [board, setBoard] = useState<{[key: string]: Task[]}>(_board);
    
    const onTaskMove = (origin: BoardKeyIndex, target: BoardKeyIndex) => {
        if(origin.index !== undefined){
        const removed = board[origin.key].splice(origin.index, 1)
        if(target.index !== undefined){
            board[target.key] = [
            ...board[target.key].slice(0, target.index),
            ...removed,
            ...board[target.key].slice(target.index)
            ]
        } else {
            board[target.key].push(removed[0])
        }
        setBoard({...board})
        }
    }

    return { onTaskMove, board }
}