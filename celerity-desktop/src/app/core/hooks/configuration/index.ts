import { useEffect } from "react"
import { useLoading } from "../loading/loading"
import { invoke } from "@tauri-apps/api"
import { Configuration } from "../../../types/configuration"
import { useAppDispatch } from "../../../store"
import { setConfiguration, setTemplates } from "../../../store/configuration"
import { Template } from "../../../types/configuration/template"

export const useConfiguration = () => {
    const { start, stop, loading } = useLoading(false)
    const dispatch = useAppDispatch()

    useEffect(() => {
        (async () => {
            start()
            try{
                dispatch(setConfiguration(await invoke<Configuration>('get_configuration')))
                dispatch(setTemplates(await invoke<Template[]>('get_templates')))
            }catch(e){}
            finally{ stop()  }
        })();
    }, [])

    return { loading }
}