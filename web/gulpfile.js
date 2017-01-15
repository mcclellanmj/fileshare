var gulp = require('gulp');
var elm  = require('gulp-elm');
var exec = require('gulp-exec');
var mkdirp = require('mkdirp');
var concat = require('gulp-concat');
var minifyCSS = require('gulp-minify-css');
var uglify = require('gulp-uglify');
var rename = require('gulp-rename');

gulp.task('elm-init', elm.init);

gulp.task('compile-elm', ['elm-init'], function() {
  gulp.src('src/Main.elm').pipe(elm.bundle('app.min.js')).pipe(uglify()).pipe(gulp.dest('dist/'));
});

gulp.task('copy-css', function() {
  return gulp.src(
    [ 'bower_components/pure/pure.css'
    , 'app.css'
    ]).pipe(gulp.dest('build/css'));
});

gulp.task('js', ['compile-elm'], function() {
});

gulp.task('css', ['copy-css'], function() {
  return gulp.src('build/css/**/*.css')
    .pipe(minifyCSS())
    .pipe(concat('app.min.css'))
    .pipe(gulp.dest('dist'));
});

gulp.task('html', [], function() {
  return gulp.src('index.html').pipe(gulp.dest('dist'));
});

gulp.task('build', ['css', 'js', 'html'], function() {
    return gulp.src('dist/*').pipe(gulp.dest('../resources/'))
});

gulp.task('default', function() {
  gulp.run('build');

  return gulp.watch(['index.html', 'src/**/*'], function() {gulp.run('build')});
});