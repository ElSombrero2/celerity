import remarkGfm from 'remark-gfm'
import Markdown from "react-markdown"
import { CodeHighlight } from '../../../../shared/CodeHighlight/CodeHighlight'

export const Documentation = ({markdown}: {markdown: string}) => {

    return (
        <div className="code">
            <Markdown
                components={{code: (props) => <CodeHighlight {...props as any} />}}
                remarkPlugins={[[remarkGfm]]}
            >
                {markdown}
            </Markdown>
        </div>
    )
}