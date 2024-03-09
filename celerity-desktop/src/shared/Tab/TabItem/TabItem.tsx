import React from "react"

export interface ITabItem extends React.HTMLAttributes<HTMLDivElement> {
    title: string 
}

export const TabItem: React.FunctionComponent<ITabItem> = (props: ITabItem) => {
    return (
        <div {...props} />
    )
}