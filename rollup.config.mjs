import terser from '@rollup/plugin-terser';

const JsPath = 'dist/tigerros_web.js';

const TerserOptions = {
    compress: {
        booleans_as_integers: true,
        drop_console: true,
    },
    mangle: {
        toplevel: true,
    },
};

export default {
    input: [JsPath],
    output: {
        file: JsPath,
        format: 'es',
    },
    plugins: [terser(TerserOptions),],
};