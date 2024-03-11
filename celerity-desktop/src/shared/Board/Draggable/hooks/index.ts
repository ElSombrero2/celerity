import React, { useState } from "react"

export const useDraggable = (parent: React.RefObject<HTMLDivElement>, origin: string, index: number) => {
    const [dragged, setDragged] = useState<string | null>(null)
    const [over, setOver] = useState<string | null>(null)
    const [position, setPosition] = useState(0)

    const onDragStart = (e: React.DragEvent<HTMLDivElement>) => {
        e.dataTransfer.setData(`${e.currentTarget.clientHeight}px`, ``)
        e.dataTransfer.setData('origin', origin)
        e.dataTransfer.setData('index', `${index}`)
        setDragged('dragged')
    }

    const onDragEnd = () => setDragged(null)
    const onDragLeave = () => setOver(null)
    const onDrop = () => setOver(null)
    
    const onDragOver = (e: React.DragEvent<HTMLDivElement>) => {
        const height = e.nativeEvent.dataTransfer?.types.find((type) => type.includes('px')) || '0px'
        const y = e.pageY - e.currentTarget.offsetTop + (parent?.current?.scrollTop || 0)
        if(e.currentTarget.clientHeight/2 - y < 0 && over !== 'bottom') {
            document.documentElement.style.setProperty('--card-height', height)
            setOver('bottom')
            setPosition(1)
        }
        else if ((e.currentTarget.clientHeight)/2 - y > 0 && over !== 'top') {
            document.documentElement.style.setProperty('--card-height', height)
            setOver('top')
            setPosition(0)
        }
    }

    return{ 
        dragged,
        over,
        onDragStart,
        onDragEnd,
        onDragLeave,
        onDragOver,
        onDrop,
        position,
    }
}