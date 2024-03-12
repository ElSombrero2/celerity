import { useAppSelector } from "../../app/store";
import { Tab } from "../../shared/Tab/Tab";
import './Project.scss'
import { TabItem } from "../../shared/Tab/TabItem/TabItem";
import { Title } from "./components/Board/Title/Title";
import { Body } from "./components/Board/Body/Body";
import { useProject } from "./Provider";
import { Figma } from "./Tabs/Figma/Figma";
import { useState } from "react";
import { Documentation } from "./Tabs/Documentations/Documentation";
import { Board } from "../../shared/Board/Board";

export const Project = () => {
    const configuration = useAppSelector((state) => state.ConfigurationReducer.configuration)
    const { onTaskMove, board,  } = useProject()
    const [current, setCurrent] = useState<string | undefined>()

    return (
        <div className="project-container">
            <h3>Welcome <span className="text-danger">{configuration?.user?.login},</span></h3>
            <h1>Project Name</h1>
            <Tab className="tab h-100"
                current={current}
                onItemClick={(title) => setCurrent(title)}
            >
                <TabItem className="h-100" title="Board">
                    <Board
                        factory={{title: Title, body: Body}}
                        board={board}
                        onTaskMove={onTaskMove}
                    />
                </TabItem>
                <TabItem className="h-100" title="Figma">
                    <Figma />
                </TabItem>
                <TabItem className="h-100" title="Documentation">
                    <Documentation />
                </TabItem>
                <TabItem title="Services">
                    
                </TabItem>
                <TabItem title="Logs">
                    
                </TabItem>
            </Tab>
        </div>
    )
}