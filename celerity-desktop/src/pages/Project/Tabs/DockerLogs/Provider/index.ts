import { useEffect, useRef, useState } from "react"
import { invoke } from "@tauri-apps/api"
import { appWindow } from "@tauri-apps/api/window"
import { listen } from "@tauri-apps/api/event"
import { useAppSelector } from "../../../../../app/store"

export enum State {
    Started = 'Stop',
    Stopped = 'Show Logs',
    Pending = 'Loading...'
}

export const useDockerLogs = (project?: string) => {
    let [logs, setLogs] = useState("")
    const [started, setStarted] = useState(false)
    const [label, setLabel] = useState(State.Stopped)
    const scrollable = useRef(null)
    const config = useAppSelector(state => state.ConfigurationReducer.configuration)
    const form = useRef(null)

    const clear = () => setLogs('')

    const start = async () => {
        if(project) {
            setStarted(true)
            setLabel(State.Started)
            const element = ((form.current as unknown as HTMLFormElement)
            .elements as any as {service: HTMLSelectElement}).service.value

            invoke('exec', {
                window: appWindow,
                config, project,
                command: `logs ${element.replace('*', '')} -f --tail 0`
            })
        }
    }

    listen('message', (payload: any) => {
        logs = logs + '\n' + payload.payload.output
        setLogs(logs)
    })

    let stop = () => {
        if(started) setLabel(State.Pending)
        appWindow.emit('end')
    }

    listen('ended', () => {
        setLabel(State.Stopped)
        setStarted(false)
    })

    useEffect(() => () => { stop() }, [config])

    useEffect(() => {
        const _code = scrollable.current as unknown as HTMLDivElement;
        if(_code?.scrollHeight) { _code.scrollTop = _code.scrollHeight - _code.clientHeight; }
    }, [logs, scrollable])

    return { scrollable, start, stop, logs, config, started, form, clear, label }
}