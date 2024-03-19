import { LegacyRef } from "react"
import { DockerServices } from "../../../../../../app/types/docker"
import { State } from "../../Provider"

export interface IControls {
    form: LegacyRef<HTMLFormElement> | null,
    started: boolean,
    allowLogs: boolean,
    onStart: () => void,
    onStop: () => void,
    onClear: () => void,
    services: DockerServices[],
    label: string,
}

export const Controls = ({form, started, allowLogs, onStart, onStop, services, label, onClear}: IControls) => {

    const disabled = !allowLogs || label === State.Pending

    return (
        <form ref={form} onSubmit={e => e.preventDefault()}>
            <button disabled={disabled} onClick={() => started ? onStop() : onStart()}>
                {label}
            </button>
            <button disabled={disabled} onClick={onClear}>
                Clear
            </button>
            <select name="service" disabled={!allowLogs || started}>
                <option value="*" style={{color: 'black'}}>
                    -- All --
                </option>
                {services.map((service, index) => (
                    <option style={{color: 'black'}} key={`${service.name}-${index}`} value={service.name}>
                        {service.name}
                    </option>
                ))}
            </select>
        </form>
    )
}