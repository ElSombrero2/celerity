import { CodeHighlight } from './CodeHighlight/CodeHighlight'
import './Documentation.scss'
import remarkGfm from 'remark-gfm'
import Markdown from "react-markdown"

export const Documentation = ({markdown}: {markdown: string}) => {

    return (
        <div className="markdown">
            <Markdown
                components={{code: (props) => <CodeHighlight {...props as any} />}}
                remarkPlugins={[[remarkGfm]]}
            >
                {markdown}
            </Markdown>
        </div>
    )
}