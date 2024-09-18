/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['*.html', './src/**/*.rs'],
    theme: {
        extend: {
            keyframes: {
                popperRotateIn: {
                    '0%': {transform: 'scale(0) rotateZ(calc(var(--direction, 0) * 45deg))'},
                    '100%': {transform: 'scale(1)'}
                }
            }
        }
    },
    plugins: []
};
