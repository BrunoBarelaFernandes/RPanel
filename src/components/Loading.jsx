
function Loading({ cor = "#3b82f6", tamanho = 40 }) {
    return (
        <svg width={tamanho} height={tamanho} viewBox="0 0 50 50">
            <circle
                cx="25"
                cy="25"
                r="20"
                fill="none"
                stroke={cor}
                strokeWidth="5"
                strokeLinecap="round"
                strokeDasharray="70 126"
            >
                <animateTransform
                    attributeName="transform"
                    type="rotate"
                    from="0 25 25"
                    to="360 25 25"
                    dur="01"
                    repeatCount="indefinite"
                />

                <animate
                    attributeName="stroke-dasharray"
                    values="10 126;70 126;10 126"
                    dur="05s"
                    repeatCount="indefinite"
                    calcMode="linear"
                />
            </circle>
        </svg>
    );
}

export default Loading;

