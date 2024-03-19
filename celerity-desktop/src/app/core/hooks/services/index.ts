import { useEffect, useState } from "react";
import { useAppSelector } from "../../../store";
import { useLoading } from "../loading/loading";
import { DockerServices } from "../../../types/docker";
import { invoke } from "@tauri-apps/api";

export const useServices = (project?: string) => {
    const { start, stop, loading } = useLoading(false)
    const [services, setServices] = useState<DockerServices[]>([])
    const config = useAppSelector(state => state.ConfigurationReducer.configuration)
    const [allowLogs, setAllowLogs] = useState(false)

    const fetchService = async () => {
        if(config && project){
            start()
            try{
                setServices(await invoke<DockerServices[]>('get_services', {config, project}))
            }catch(e){}
            finally{ stop()  }
        }
    }

    useEffect(() => { fetchService() }, [config, project])

    useEffect(() => {
        setAllowLogs(services.reduce((prev, current) => prev || current.state === 'running', false))
     }, [services])

    return { loading, services, allowLogs }
}