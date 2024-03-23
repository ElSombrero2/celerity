import { CodeHighlight } from "../../../../shared/CodeHighlight/CodeHighlight"
import Markdown from "react-markdown"
import remarkGfm from "remark-gfm"
import './DockerLogs.scss'
import { useDockerLogs } from "./Provider"
import { useServices } from "../../../../app/core/hooks/services"
import { themes } from "prism-react-renderer"
import { Controls } from "./components/Controls/Controls"

export const DockerLogs = ({ project }: { project?: string }) => {
    const { start, logs, stop, scrollable, started, form, label, clear } = useDockerLogs(project)
    const { allowLogs, services } = useServices(project)

    return (
        <div className="logs flex flex-col gap-8">
            <Controls 
                started={started}
                form={form}
                onStart={start}
                onStop={stop}
                services={services}
                allowLogs={allowLogs}
                label={label}
                onClear={clear}
            />
            <div className="code">
                <Markdown
                    components={{
                        code: (props) => (
                            <CodeHighlight
                                theme={themes.oceanicNext}
                                itemRef={scrollable}
                                {...props as any} 
                            />
                    )}}
                    remarkPlugins={[[remarkGfm]]}
                >
                    {"```bash\n" + logs + "\n```"}
                </Markdown>
            </div>
        </div>
    )
}