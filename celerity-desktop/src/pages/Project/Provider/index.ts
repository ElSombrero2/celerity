import { useEffect, useState } from "react";
import { _board } from "../../../app/__mock__/board";
import { useProjectBoard } from "./Board";
import { useAppSelector } from "../../../app/store";
import { invoke } from "@tauri-apps/api";
import { useReadme } from "./Readme";

export const useProject = (id: string | undefined) => {
    const [ project, setProject ] = useState<any>(null)
    const { onTaskMove, board } = useProjectBoard(project?.board)
    const configuration = useAppSelector(state => state.ConfigurationReducer.configuration)
    const readme = useReadme(configuration, id)
    
    useEffect(() => {
        if(id && configuration) {
            invoke('get_project', {config: configuration, id})
            .then(setProject)
        }
    }, [id, configuration]);

    return { board, onTaskMove, configuration, project, readme }
}