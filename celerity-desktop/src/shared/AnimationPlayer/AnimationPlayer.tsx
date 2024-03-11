import * as LottiePlayer from '@lottiefiles/lottie-player'

export const AnimationPlayer = (props: ILottiePlayer) => {
    return (
        LottiePlayer.LottiePlayer &&
        <lottie-player {...props} />
    )
}