import { _markdown } from '../../../../app/__mock__/docs'
import { CodeHighlight } from './CodeHighlight/CodeHighlight'
import './Documentation.scss'
import remarkGfm from 'remark-gfm'
import Markdown from "react-markdown"

export const Documentation = () => {
    return (
        <div className="markdown">
            <Markdown
                components={{code: (props) => <CodeHighlight {...props as any} />}}
                remarkPlugins={[[remarkGfm]]}
            >
                {_markdown}
            </Markdown>
        </div>
    )
}