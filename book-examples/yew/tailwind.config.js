const colors = require('@radix-ui/colors');

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['*.html', './src/**/*.rs'],
    theme: {
        extend: {
            colors: {
                ...colors
            }
        }
    },
    plugins: []
};
