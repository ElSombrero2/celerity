import React from 'react'
import './Draggable.scss'
import { useDraggable } from './hooks'

export interface IDraggable {
    children: React.ReactElement,
    among: React.RefObject<HTMLDivElement>,
    onDrop: (e: React.DragEvent<HTMLDivElement>, position: number) => void,
    index: number,
    origin: string,
}

export const Draggable = ({children, among, onDrop, index, origin}: IDraggable) => {
    const { 
        dragged, over, onDragEnd, onDragOver, position,
        onDragLeave, onDragStart, onDrop: _onDrop,
    } = useDraggable(among, origin, index)

    const drop = (e: React.DragEvent<HTMLDivElement>) => {
        _onDrop()
        onDrop(e, position)
    }

    return (
        <div draggable 
            className={`drag-wrapper ${dragged} ${over}`}
            onDragStart={onDragStart}
            onDrop={drop}
            onDragEnd={onDragEnd}
            onDragOver={onDragOver}
            onDragLeaveCapture={onDragLeave}
            >
            {children}
        </div>
    )
}