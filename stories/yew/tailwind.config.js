/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['*.html', './src/**/*.rs'],
    theme: {
        extend: {
            keyframes: {
                checkboxFadeIn: {
                    from: {opacity: 0},
                    to: {opacity: 1}
                },
                checkboxFadeOut: {
                    from: {opacity: 1},
                    to: {opacity: 0}
                },
                popperRotateIn: {
                    '0%': {transform: 'scale(0) rotateZ(calc(var(--direction, 0) * 45deg))'},
                    '100%': {transform: 'scale(1)'}
                },
                presenceFadeIn: {
                    from: {opacity: 0},
                    to: {opacity: 1}
                },
                presenceFadeOut: {
                    from: {opacity: 1},
                    to: {opacity: 0}
                },
                presenceSlideUp: {
                    from: {transform: 'translateY(30px)'},
                    to: {transform: 'translateY(0)'}
                },
                presenceSlideDown: {
                    from: {transform: 'translateY(0)'},
                    to: {transform: 'translateY(30px)'}
                }
            }
        }
    },
    plugins: []
};
