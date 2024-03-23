import { LegacyRef } from "react"
import { DockerServices } from "../../../../../../app/types/docker"
import { State } from "../../Provider"
import { Button } from "@/ui/components/ui/button"
import { Select } from "@/ui/components/ui/select"
import { SelectContent, SelectItem, SelectTrigger, SelectValue } from "@radix-ui/react-select"
import { Input } from "@/ui/components/ui/input"

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
            <Button variant="outline" disabled={disabled} onClick={() => started ? onStop() : onStart()}>
                {label}
            </Button>
            <Button variant="destructive" disabled={disabled} onClick={onClear}>
                Clear
            </Button>
            <Input className="w-[250px]" placeholder="Services" />
        </form>
    )
}