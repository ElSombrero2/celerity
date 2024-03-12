import { useState } from 'react'
import { AnimationPlayer } from '../../../../shared/AnimationPlayer/AnimationPlayer'
import { Slot, Switch } from '../../../../shared/Switch/Switch'
import '../../Project.scss'
import './Figma.scss'

export const Figma = () => {

    const [loading, setLoading] = useState(true)

    return (
        <>
            <Switch condition={!loading}>
                <Slot name="default" className="w-100 tab">
                    <iframe
                        onLoad={() => setLoading(false)}
                        src="https://www.figma.com/embed?embed_host=celerity.io
                        &embed_origin=http://localhost:1420
                        &url=https://www.figma.com/file/gzwMlEjliQymdB2KQ7oTQX/music-player"
                    />
                </Slot>
                <Slot name="fallback" className="tab w-100 h-100 d-flex justify-content-center align-items-center">
                    <AnimationPlayer
                        autoplay
                        loop
                        mode="normal"
                        src="/loading.json"
                        style={{display: 'block', width: '50vw', height: '50vh'}}
                    />
                </Slot>
            </Switch>
        </>
    )
}