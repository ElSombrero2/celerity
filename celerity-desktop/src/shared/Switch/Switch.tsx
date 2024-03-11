import React from 'react'
import { useSwitch } from './Provider'
import './Switch.scss'

export interface ISlot extends React.HTMLAttributes<HTMLDivElement> {
    name: 'default' | 'fallback'
    children: React.ReactNode
}

export const Slot = ({children}: ISlot) => children

interface ISwitch {
    condition: boolean,
    children: [React.ReactElement<ISlot>, React.ReactElement<ISlot>],
}

export const Switch = ({children, condition}: ISwitch) => {
    const { defaultElement, fallback } = useSwitch(children)
    if(!defaultElement || !fallback) throw new Error("You must specify default and fallback element")
    return (
        <>
            <div
                {...defaultElement.props}
                className={!condition ? 'switch-hidden' : defaultElement.props.className}
            />
            <div
                {...fallback.props}
                className={condition ? 'switch-hidden' : fallback.props.className}
            />
        </>
    )
}