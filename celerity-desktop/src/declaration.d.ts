interface ILottiePlayer extends React.HTMLAttributes<HTMLDivElement> {
    autoplay?: boolean,
    controls?: boolean,
    loop?: boolean,
    mode?: string,
    src?: string,
}

declare namespace JSX {
    interface IntrinsicElements {
      "lottie-player": ILottiePlayer;
    }
  }