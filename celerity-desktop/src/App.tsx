import "./App.scss";
import { useAppSelector } from "./app/store";
import { Board, BoardKeyIndex } from "./app/component/Board/Board";
import { board as _board } from "./__mock__/board";
import { Card } from "./ui/components/Cards/Card/Card";
import { Task } from "./app/types/kanban";
import { useState } from "react";

function App() {
  const configuration = useAppSelector((state) => state.ConfigurationReducer.configuration)
  const [board, setBoard] = useState<{[key: string]: Task[]}>(_board);

  const body = (task: Task) => (
    <Card>
      {task.title}
    </Card>
  )

  const title = (name: string) => (
    <div className="title">
      <h3>{name}</h3>
      <i className="fa-solid fa-circle-plus"></i>
    </div>
  )

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

  return (
    <div className="container">
      <h3>Welcome <span className="name">{configuration?.user?.login},</span></h3>
      <h1>Project Name</h1>
      <div className="menu">
        <div className="item active">
          Board
        </div>
        <div className="item">
          State
        </div>
        <div className="item">
          Details
        </div>
        <div className="item">
          Statistics
        </div>
      </div>

      <Board
        onTaskMove={onTaskMove} 
        factory={{ title, body }} 
        board={board}
      />
    </div>
  );
}

export default App;
