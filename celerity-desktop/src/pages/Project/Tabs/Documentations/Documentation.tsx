import Markdown from "react-markdown"
import { _markdown } from "../../../../app/__mock__/docs"

export const Documentation = () => {
    return (
        <div className="markdown" style={{height: '100%', overflow: 'auto'}}>
            <Markdown >{_markdown}</Markdown>
        </div>
    )
}