import { useEffect, useState } from "react";
import { Configuration } from "../../../../app/types/configuration";
import { invoke } from "@tauri-apps/api";

export const useReadme = (config: Configuration | null, id?: string) => {
    const [readme, setReadme] = useState<string>('')

    useEffect(() => {
        if(id && config){
            invoke<String>('get_documentation', {config, id})
            .then(e => setReadme(e.toString()))
        }
    }, [id, config])

    return readme
}