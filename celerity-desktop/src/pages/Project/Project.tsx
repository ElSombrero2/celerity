import { useAppSelector } from "../../app/store";
import { _board } from "../../__mock__/board";
// import { useProject } from "./Provider";
// import { Board } from "../../app/component/Board/Board";
// import { Title } from "./components/Board/Title/Title";
// import { Body } from "./components/Board/Body/Body";
import { Tab } from "../../shared/Tab/Tab";
import './Project.scss'

export const Project = () => {
    const configuration = useAppSelector((state) => state.ConfigurationReducer.configuration)
    // const { onTaskMove, board,  } = useProject()

    return (
        <div className="project-container">
            <h3>Welcome <span className="text-danger">{configuration?.user?.login},</span></h3>
            <h1>Project Name</h1>
            <Tab>

            </Tab>
        </div>
    )
}