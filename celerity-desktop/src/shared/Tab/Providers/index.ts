import React, { useEffect, useState } from "react";
import { ITabItem } from "../TabItem/TabItem";

export const useTab = (children: React.ReactElement<ITabItem>[], current?: string) => {

    const [currentElement, setCurrentElement] = useState<React.ReactElement<ITabItem> | undefined>();

    useEffect(() => {
        if(children && children.length && current){
            const element = children.find((e) => e.props.title === current)
            setCurrentElement(element || children[0])
        }else setCurrentElement(children[0]);
    }, [children, current]);

    const isActive = (props: ITabItem) => currentElement?.props.title === props.title

    return { currentElement, isActive }
}