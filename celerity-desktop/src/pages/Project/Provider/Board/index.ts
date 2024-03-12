import { useEffect, useState } from "react";
import { Task } from "../../../../app/types/kanban";
import { BoardKeyIndex } from "../../../../shared/Board/Board";

export const useProjectBoard = (_board?: {[key: string]: Task[]}) => {
    const [board, setBoard] = useState<{[key: string]: Task[]} | undefined>(_board);
    
    useEffect(() => {
        if(_board) setBoard(_board)
    }, [_board])

    const onTaskMove = (origin: BoardKeyIndex, target: BoardKeyIndex) => {
        if(board){
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
    }

    return { onTaskMove, board }
}