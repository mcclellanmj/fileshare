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
  gulp.src('src/Main.elm').pipe(elm()).pipe(gulp.dest('build/js'));
});

gulp.task('compile-elm-css', function() {
  var options = {
    outputDir: "build/css"
  };

  mkdirp(options.outputDir, function(err) {
    if(err) {
      console.log("Failed to create directory [" + options.outputDir + "]");
      process.exit(1);
    }
  });

  var reportOptions = {};
  return gulp.src('stylesheet/Stylesheets.elm')
    .pipe(exec('elm-css <%= file.path %> --output <%= options.outputDir %>', options))
    .pipe(exec.reporter(reportOptions));
});

gulp.task('copy-third-party-css', function() {
  return gulp.src(
    [ 'bower_components/font-awesome/css/font-awesome.css'
    , 'bower_components/pure/pure.css'
    ]).pipe(gulp.dest('build/css'));
});

gulp.task('combine-js', ['compile-elm'], function() {
  return gulp.src('build/js/**/*.js')
    .pipe(concat('app.min.js'))
    .pipe(uglify())
    .pipe(gulp.dest('dist'))
});

gulp.task('combine-css', ['compile-elm-css', 'copy-third-party-css'], function() {
  return gulp.src('build/css/**/*.css')
    .pipe(minifyCSS())
    .pipe(concat('app.min.css'))
    .pipe(gulp.dest('dist'))
});

gulp.task('build', ['combine-css', 'combine-js'])

gulp.task('default', function() {
  gulp.run('build');

  gulp.watch(['src/**/*'], function() {gulp.run('combine-js')});
  gulp.watch(['stylesheet/**/*'], function() {gulp.run('combine-css')});
});