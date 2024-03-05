import React from 'react'
import './Card.scss'

export interface ICard extends React.HTMLAttributes<HTMLDivElement>{}

export const Card = ({className, ...props}: ICard) => {
    return (
        <div className={`card ${className}`} 
            {...props} 
        />
    )
}