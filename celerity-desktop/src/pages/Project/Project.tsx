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
import { useParams } from "react-router-dom";
import { DockerLogs } from "./Tabs/DockerLogs/DockerLogs";
import { Services } from "./Tabs/Services/Services";
import moment from "moment";

export const Project = () => {
    const { id } = useParams<{id: string}>()
    const { onTaskMove, board, configuration, readme, project } = useProject(id)
    const [current, setCurrent] = useState<string | undefined>()

    return (
        <div className="project-container">
            <h3>Welcome <span className="text-danger">{configuration?.user?.login},</span></h3>
            <div>
                <h1>{project?.name}</h1>
                <p className="text-light">
                    {moment(project?.created_at).format('DD MMM YYYY HH:mm ')}
                    from
                    <span className="text-danger">{` ${project?.based_template} `}</span>
                    template
                </p>
            </div>
            <Tab className="tab h-100"
                current={current}
                onItemClick={(title) => setCurrent(title)}
            >
                <TabItem className="h-100" title="Board">
                    {board && <Board
                        factory={{title: Title, body: Body}}
                        board={board}
                        onTaskMove={onTaskMove}
                    />}
                </TabItem>
                <TabItem className="h-100" title="Figma">
                    <Figma />
                </TabItem>
                <TabItem className="h-100" title="Documentation">
                    <Documentation markdown={readme || ''} />
                </TabItem>
                <TabItem title="Services">
                    <Services project={id} />
                </TabItem>
                <TabItem title="Logs">
                    <DockerLogs project={id} />
                </TabItem>
            </Tab>
        </div>
    )
}