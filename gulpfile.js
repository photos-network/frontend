const { series, parallel, src, dest, watch } = require('gulp');
const path = require('path');
const livereload = require('livereload');
const del = require('del');

const isProd = require('minimist')(process.argv.slice(2)).prod;
const DIST_PATH = 'dist/';

function eslint () {
	const gulpEslint = require('gulp-eslint');
	return src(['src/**/*.js', 'src/**/*.svelte', '*.js'])
		.pipe(gulpEslint())
		.pipe(gulpEslint.format())
		.pipe(gulpEslint.results(results => {
			if (results.errorCount) console.log('\x07');
		}));
}


function rollupBuild (inputOptions = {}, outputOptions = {}) {
	const stream = require('stream');
	const rollup = require('rollup');
	const readable = new stream.Readable();
	readable._read = function () { };
	rollup
		.rollup(inputOptions)
		.then(bundle => bundle.generate(outputOptions))
		.then(out => {
			if (!out.code) out = out.output[0];
			readable.push(out.code);
			if (outputOptions.output.sourcemap) {
				readable.push('\n//# sourceMappingURL=');
				readable.push(out.map.toUrl());
			}
			readable.push(null);
		})
		.catch(error => setTimeout(() => readable.emit('error', error)));
	return readable;
}


function js () {
	const commonjs = require('@rollup/plugin-commonjs');
	const source = require('vinyl-source-stream');
	const svelte = require('rollup-plugin-svelte');
	const resolve = require('@rollup/plugin-node-resolve').nodeResolve;
	const {terser} = require('rollup-plugin-terser');
	const serve = require('rollup-plugin-serve');
	const inputOptions = {
		input: './src/index.js',
		plugins: [
			commonjs(),
			resolve({
				extensions: ['.mjs', '.js', '.svelte', '.json'],
				dedupe: importee => importee === 'svelte' || importee.startsWith('svelte/')
			}),
			svelte({ compilerOptions: {dev: !isProd, css: false } }),
			isProd && terser(),
			serve({ contentBase: [DIST_PATH, 'storage'], port: 3000 }),
		]
	};
	const outputOptions = {output: {format: 'iife', sourcemap: !isProd}};
	return rollupBuild(inputOptions, outputOptions)
		.pipe(source('index.js'))
		.pipe(dest(DIST_PATH));
}


function css () {
	const noop = require('through2').obj;
	const sourcemap = require('gulp-sourcemaps');
	const concat = require('gulp-concat');
	const cleanCSS = require('gulp-clean-css');

	return src(['src/**/*.css'])
		.pipe(isProd ? noop() : sourcemap.init())
		.pipe(concat('index.css'))
		.pipe(isProd ? noop() : sourcemap.write())
		.pipe(isProd ? cleanCSS() : noop())
		.pipe(dest(DIST_PATH));
}

function assets () {
	return src(['src/assets/**/*.*']).pipe(dest(DIST_PATH + 'assets/'));
}

function htmls () {
	return src(['src/*.html', 'src/*.json']).pipe(dest(DIST_PATH));
}

function cleanup () {
	return del(['dist/*']);
}

async function watchTask (done) {
	if (isProd) return done();
	const server = livereload.createServer({ delay: 100 });
	server.watch(path.join(__dirname, DIST_PATH));
	watch('src/**/*.css', css);
	watch('src/**/*.{js,svelte}', parallel(eslint, js));
	watch('src/*.html', htmls);
	watch('src/assets/**/*.*', assets);
}

const build = parallel(js, css, assets, htmls, eslint);
exports.eslint = eslint;
exports.build = series(cleanup, build);
exports.default = series(build, watchTask);
