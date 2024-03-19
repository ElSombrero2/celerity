import { useEffect, useState } from "react";
import { Todos } from "../../../../app/types/kanban";
import { BoardKeyIndex } from "../../../../shared/Board/Board";

export const useProjectBoard = (_board?: {[key: string]: Todos}) => {
    const [board, setBoard] = useState<{[key: string]: Todos} | undefined>(_board);
    
    useEffect(() => {
        if(_board) setBoard(_board)
    }, [_board])

    const onTaskMove = (origin: BoardKeyIndex, target: BoardKeyIndex) => {
        if(board){
            if(origin.index !== undefined){
                const removed = board[origin.key].todos.splice(origin.index, 1)
                if(target.index !== undefined){
                    board[target.key].todos = [
                    ...board[target.key].todos.slice(0, target.index),
                    ...removed,
                    ...board[target.key].todos.slice(target.index)
                    ]
                } else {
                    board[target.key].todos.push(removed[0])
                }
                setBoard({...board})
            }
        }
    }

    return { onTaskMove, board }
}