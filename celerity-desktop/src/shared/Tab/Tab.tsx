import React, { ReactElement } from 'react'
import { ITabItem } from './TabItem/TabItem'
import { useTab } from './Providers'
import './Tab.scss'

export interface ITab {
    children: ReactElement<ITabItem>[],
    current?: string,
    onItemClick?: (title: string) => void
}

export const Tab: React.FunctionComponent<ITab> = ({ children, current, onItemClick }: ITab) => {

    const { isActive, currentElement } = useTab(children, current)

    return (
        <div className="tab-wrapper d-flex flex-column gap-24 h-100">
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