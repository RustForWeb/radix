/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['*.html', './src/**/*.rs'],
    theme: {
        extend: {
            keyframes: {
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
