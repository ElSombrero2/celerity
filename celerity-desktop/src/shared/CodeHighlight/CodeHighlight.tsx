import { Highlight, PrismTheme, themes } from "prism-react-renderer"
import React from "react"
import Convert from 'ansi-to-html'
import parser from 'html-react-parser'

interface CodeHighlightProps extends React.HTMLAttributes<HTMLDivElement> {
    node: Element,
    theme?: PrismTheme,
}

export const CodeHighlight = ({children, className, node, theme, ...rest}: CodeHighlightProps) => {
    const match = /language-(\w+)/.exec(className || '')
    const langage = className?.split('language-')[1]

    const converter = new Convert()

    if(langage && match) return (
        <Highlight
            theme={theme || themes.oceanicNext}
            code={children?.toString() || ''}
            language={langage}
        >
            {({ style, tokens, getLineProps, getTokenProps }) => (
                <pre ref={rest.itemRef} style={style} className="inside">
                    {tokens.filter((e) => e.length > 0 && e[0].content !== '\n').map((line, i) => (
                        <div key={i} {...getLineProps({ line })}>
                            {line.map((token, key) => 
                                <span key={key} {...getTokenProps({ token })}>
                                    {parser(converter.toHtml(getTokenProps({ token }).children))}
                                </span>
                            )}
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