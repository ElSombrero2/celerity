import { Highlight, themes } from "prism-react-renderer"
import React from "react"

interface CodeHighlightProps extends React.HTMLAttributes<HTMLDivElement> {
    node: Element
}

export const CodeHighlight = ({children, className, node, ...rest}: CodeHighlightProps) => {
    const match = /language-(\w+)/.exec(className || '')
    const langage = className?.split('language-')[1]

    if(langage && match) return (
        <Highlight
            theme={themes.oceanicNext}
            code={children?.toString() || ''}
            language={langage}
        >
            {({ style, tokens, getLineProps, getTokenProps }) => (
                <pre style={style} className="inside">
                    {tokens.filter((e) => e.length > 0 && e[0].content !== '\n').map((line, i) => (
                        <div key={i} {...getLineProps({ line })}>
                            {line.map((token, key) => <span key={key} {...getTokenProps({ token })} />)}
                        </div>
                    ))}
                </pre>
            )}
        </Highlight>
    )

    return  (
        <code {...rest} className={className}>
            {children}
        </code>
    )
}