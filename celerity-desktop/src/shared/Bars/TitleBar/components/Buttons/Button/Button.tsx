import React from "react";
import "./Button.scss"

interface IButton extends React.ButtonHTMLAttributes<HTMLButtonElement> {}

export const Button = ({className, ...props}: IButton) => (
    <button {...props} className={`title-button ${className}`} />
)