import React, { ReactElement } from 'react'
import { ITabItem } from './TabItem/TabItem'
import { useTab } from './Providers'
import './Tab.scss'

export interface ITab extends React.HTMLAttributes<HTMLDivElement> {
    children: ReactElement<ITabItem>[],
    current?: string,
    onItemClick?: (title: string) => void
}

export const Tab: React.FunctionComponent<ITab> = ({ children, current, onItemClick, ...props }: ITab) => {

    const { isActive, currentElement } = useTab(children, current)

    return (
        <div {...props} className={`tab-wrapper flex flex-col  gap-8 ${props.className}`}>
            <div className="menu">
                {children.map(({props}, index) => (
                    <div
                        onClick={() => onItemClick && onItemClick(props.title)}
                        key={`${props.title}-${index}`}
                        className={`item ${isActive(props) && 'active'}`}
                    >
                        {props.title}
                    </div>
                ))}
            </div>
            {currentElement}
        </div>
    )
}