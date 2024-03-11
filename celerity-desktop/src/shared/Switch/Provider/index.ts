import React from "react"
import { ISlot } from "../Switch"

export const useSwitch = (elements: React.ReactElement<ISlot>[]) => {
    const defaultElement = elements.find((e) => e.props.name === 'default')
    const fallback = elements.find((e) => e.props.name === 'fallback')
    return { defaultElement, fallback }
}